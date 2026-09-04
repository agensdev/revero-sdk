use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use reqwest::{header::RETRY_AFTER, Client, Method, StatusCode, Url};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{
    ApiError, ClientConfig, GeneratedReveroClient, MeetingCreate, MeetingRead, OperationRead,
    OutputContentRead, OutputRead, PresignedUpload, PublicOperationStatus, RecordingCreate,
    RecordingPrepared, RequestOptions,
};

const TOKEN_PATH: &str = "/oauth/partner/token";
const TOKEN_EARLY_REFRESH: Duration = Duration::from_secs(30);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scope {
    MeetingsRead,
    MeetingsWrite,
    RecordingsWrite,
}

impl Scope {
    const fn as_str(self) -> &'static str {
        match self {
            Self::MeetingsRead => "meetings:read",
            Self::MeetingsWrite => "meetings:write",
            Self::RecordingsWrite => "recordings:write",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ReveroClientOptions {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub scopes: Vec<Scope>,
    pub timeout: Duration,
}

impl ReveroClientOptions {
    pub fn new(
        base_url: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        scopes: Vec<Scope>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            scopes,
            timeout: Duration::from_secs(60),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WaitOptions {
    pub timeout: Duration,
    pub initial_delay: Duration,
    pub max_delay: Duration,
}

impl WaitOptions {
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            ..Self::default()
        }
    }
}

impl Default for WaitOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(15 * 60),
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(15),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid Revero client configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Revero transport failed: {0}")]
    Transport(#[from] reqwest::Error),
    #[error(transparent)]
    Api(#[from] ApiError),
    #[error("Revero returned an invalid OAuth token response")]
    InvalidTokenResponse,
    #[error("the presigned upload request is invalid: {0}")]
    InvalidUploadRequest(String),
    #[error("object storage rejected the upload with HTTP {status}: {body}")]
    UploadRejected { status: StatusCode, body: String },
    #[error("operation {operation_id} did not finish before the deadline")]
    DeadlineExceeded { operation_id: String },
}

#[derive(Clone)]
pub struct ReveroClient {
    http: Client,
    base_url: Arc<str>,
    resource: Arc<str>,
    client_id: Arc<str>,
    client_secret: Arc<str>,
    scopes: Arc<str>,
    token: Arc<Mutex<Option<CachedToken>>>,
}

#[derive(Clone, Debug)]
struct CachedToken {
    value: String,
    refresh_at: Instant,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

enum PollAttempt {
    Operation(OperationRead),
    RateLimited(Option<Duration>),
}

impl ReveroClient {
    pub fn new(options: ReveroClientOptions) -> Result<Self, Error> {
        Self::build(options, false)
    }

    fn build(options: ReveroClientOptions, allow_http: bool) -> Result<Self, Error> {
        let parsed = Url::parse(&options.base_url)
            .map_err(|_| Error::InvalidConfiguration("base_url must be an absolute URL".into()))?;
        if parsed.scheme() != "https" && !(allow_http && parsed.scheme() == "http") {
            return Err(Error::InvalidConfiguration(
                "base_url must use HTTPS".into(),
            ));
        }
        if options.client_id.is_empty()
            || options.client_secret.is_empty()
            || options.scopes.is_empty()
            || options.timeout.is_zero()
        {
            return Err(Error::InvalidConfiguration(
                "client_id, client_secret, scopes and a non-zero timeout are required".into(),
            ));
        }
        let base_url = options.base_url.trim_end_matches('/').to_owned();
        let mut scopes: Vec<_> = options.scopes.iter().map(|scope| scope.as_str()).collect();
        scopes.sort_unstable();
        scopes.dedup();
        let http = Client::builder().timeout(options.timeout).build()?;
        Ok(Self {
            http,
            resource: format!("{base_url}/public/v1").into(),
            base_url: base_url.into(),
            client_id: options.client_id.into(),
            client_secret: options.client_secret.into(),
            scopes: scopes.join(" ").into(),
            token: Arc::new(Mutex::new(None)),
        })
    }

    #[cfg(test)]
    fn new_for_test(options: ReveroClientOptions) -> Result<Self, Error> {
        Self::build(options, true)
    }

    async fn generated(&self) -> Result<GeneratedReveroClient, Error> {
        let config = ClientConfig {
            base_url: self.base_url.to_string(),
            token: Some(self.access_token().await?),
            reqwest_client: Some(self.http.clone()),
            ..Default::default()
        };
        Ok(GeneratedReveroClient::new(config)?)
    }

    pub async fn create_meeting(&self, request: &MeetingCreate) -> Result<MeetingRead, Error> {
        Ok(self
            .generated()
            .await?
            .public_api
            .create_public_meeting(request, None)
            .await?)
    }

    pub async fn get_meeting(&self, meeting_id: &str) -> Result<MeetingRead, Error> {
        Ok(self
            .generated()
            .await?
            .public_api
            .get_public_meeting(meeting_id, None)
            .await?)
    }

    pub async fn prepare_recording(
        &self,
        meeting_id: &str,
        request: &RecordingCreate,
    ) -> Result<RecordingPrepared, Error> {
        Ok(self
            .generated()
            .await?
            .public_api
            .prepare_public_recording_upload(meeting_id, request, None)
            .await?)
    }

    /// Upload bytes using the method, URL, and headers from the presigned request.
    /// No Revero bearer token is sent to object storage.
    pub async fn upload_bytes(
        &self,
        upload: &PresignedUpload,
        bytes: impl Into<bytes::Bytes>,
    ) -> Result<(), Error> {
        let method = upload.method.as_deref().unwrap_or("PUT");
        let method = Method::from_bytes(method.as_bytes())
            .map_err(|error| Error::InvalidUploadRequest(error.to_string()))?;
        let mut request = self.http.request(method, &upload.url);
        for (name, value) in &upload.headers {
            request = request.header(name, value);
        }
        let response = request.body(bytes.into()).send().await?;
        if response.status().is_success() {
            return Ok(());
        }
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(Error::UploadRejected { status, body })
    }

    pub async fn complete_recording(
        &self,
        recording_id: &str,
        idempotency_key: &str,
    ) -> Result<OperationRead, Error> {
        let options = RequestOptions::new().additional_header("Idempotency-Key", idempotency_key);
        Ok(self
            .generated()
            .await?
            .public_api
            .complete_public_recording_upload(recording_id, Some(options))
            .await?)
    }

    pub async fn get_operation(&self, operation_id: &str) -> Result<OperationRead, Error> {
        Ok(self
            .generated()
            .await?
            .public_api
            .get_public_operation(operation_id, None)
            .await?)
    }

    pub async fn wait_for_operation(
        &self,
        operation_id: &str,
        options: WaitOptions,
    ) -> Result<OperationRead, Error> {
        if options.timeout.is_zero()
            || options.initial_delay.is_zero()
            || options.max_delay.is_zero()
        {
            return Err(Error::InvalidConfiguration(
                "wait durations must be non-zero".into(),
            ));
        }
        let deadline = Instant::now() + options.timeout;
        let mut delay = options.initial_delay.min(options.max_delay);
        loop {
            let next_delay = match self.poll_operation(operation_id).await? {
                PollAttempt::Operation(operation)
                    if matches!(
                        operation.status,
                        PublicOperationStatus::Completed
                            | PublicOperationStatus::Failed
                            | PublicOperationStatus::Cancelled
                    ) =>
                {
                    return Ok(operation);
                }
                PollAttempt::Operation(_) => delay,
                PollAttempt::RateLimited(retry_after) => retry_after.unwrap_or(delay),
            };
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() || next_delay >= remaining {
                return Err(Error::DeadlineExceeded {
                    operation_id: operation_id.to_owned(),
                });
            }
            tokio::time::sleep(next_delay).await;
            delay = delay.saturating_mul(2).min(options.max_delay);
        }
    }

    pub async fn list_outputs(&self, meeting_id: &str) -> Result<Vec<OutputRead>, Error> {
        Ok(self
            .generated()
            .await?
            .public_api
            .list_public_meeting_outputs(meeting_id, None)
            .await?)
    }

    pub async fn get_output(&self, output_id: &str) -> Result<OutputContentRead, Error> {
        Ok(self
            .generated()
            .await?
            .public_api
            .get_public_meeting_output(output_id, None)
            .await?)
    }

    async fn access_token(&self) -> Result<String, Error> {
        let mut cached = self.token.lock().await;
        if let Some(token) = cached.as_ref() {
            if Instant::now() < token.refresh_at {
                return Ok(token.value.clone());
            }
        }
        let response = self
            .http
            .post(format!("{}{TOKEN_PATH}", self.base_url))
            .basic_auth(self.client_id.as_ref(), Some(self.client_secret.as_ref()))
            .form(&[
                ("grant_type", "client_credentials"),
                ("resource", self.resource.as_ref()),
                ("scope", self.scopes.as_ref()),
            ])
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.ok();
            return Err(ApiError::from_response(status, body.as_deref()).into());
        }
        let token: TokenResponse = response.json().await?;
        if token.access_token.is_empty() || token.token_type != "Bearer" || token.expires_in == 0 {
            return Err(Error::InvalidTokenResponse);
        }
        let refresh_after =
            Duration::from_secs(token.expires_in).saturating_sub(TOKEN_EARLY_REFRESH);
        *cached = Some(CachedToken {
            value: token.access_token.clone(),
            refresh_at: Instant::now() + refresh_after,
        });
        Ok(token.access_token)
    }

    async fn poll_operation(&self, operation_id: &str) -> Result<PollAttempt, Error> {
        let response = self
            .http
            .get(format!(
                "{}/public/v1/operations/{operation_id}",
                self.base_url
            ))
            .bearer_auth(self.access_token().await?)
            .send()
            .await?;
        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            return Ok(PollAttempt::RateLimited(parse_retry_after(&response)));
        }
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.ok();
            return Err(ApiError::from_response(status, body.as_deref()).into());
        }
        Ok(PollAttempt::Operation(response.json().await?))
    }
}

fn parse_retry_after(response: &reqwest::Response) -> Option<Duration> {
    let value = response.headers().get(RETRY_AFTER)?.to_str().ok()?;
    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }
    let retry_at = httpdate::parse_http_date(value).ok()?;
    retry_at.duration_since(SystemTime::now()).ok()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use axum::body::Bytes;
    use axum::extract::State;
    use axum::http::{HeaderMap, StatusCode as AxumStatusCode};
    use axum::response::{IntoResponse, Response};
    use axum::routing::{get, post, put};
    use axum::{Form, Json, Router};
    use chrono::DateTime;
    use serde_json::{json, Value};
    use tokio::net::TcpListener;

    use super::*;

    #[derive(Clone, Default)]
    struct TestState {
        token_calls: Arc<AtomicUsize>,
        operation_calls: Arc<AtomicUsize>,
        upload_calls: Arc<AtomicUsize>,
    }

    async fn token(
        State(state): State<TestState>,
        headers: HeaderMap,
        Form(form): Form<HashMap<String, String>>,
    ) -> Json<Value> {
        state.token_calls.fetch_add(1, Ordering::SeqCst);
        assert!(headers["authorization"]
            .to_str()
            .unwrap()
            .starts_with("Basic "));
        assert_eq!(form["grant_type"], "client_credentials");
        assert_eq!(form["scope"], "meetings:read meetings:write");
        assert!(form["resource"].ends_with("/public/v1"));
        Json(json!({
            "access_token": "opaque-token",
            "token_type": "Bearer",
            "expires_in": 600,
            "scope": form["scope"],
        }))
    }

    async fn operation(State(state): State<TestState>, headers: HeaderMap) -> Response {
        assert_eq!(headers["authorization"], "Bearer opaque-token");
        if state.operation_calls.fetch_add(1, Ordering::SeqCst) == 0 {
            return (
                AxumStatusCode::TOO_MANY_REQUESTS,
                [("Retry-After", "0")],
                Json(json!({"code": "rate_limited", "message": "wait"})),
            )
                .into_response();
        }
        Json(json!({
            "id": "operation-1",
            "meeting_id": "meeting-1",
            "command": "complete_recording",
            "status": "completed",
            "phase": "delivery",
            "created_at": "2026-09-01T09:15:00+00:00",
            "updated_at": "2026-09-01T09:16:00+00:00",
            "completed_at": "2026-09-01T09:16:00+00:00"
        }))
        .into_response()
    }

    async fn upload(
        State(state): State<TestState>,
        headers: HeaderMap,
        body: Bytes,
    ) -> AxumStatusCode {
        assert_eq!(headers["content-type"], "audio/webm");
        assert!(headers.get("authorization").is_none());
        assert_eq!(body, Bytes::from_static(b"audio"));
        state.upload_calls.fetch_add(1, Ordering::SeqCst);
        AxumStatusCode::NO_CONTENT
    }

    async fn client() -> (ReveroClient, TestState) {
        let state = TestState::default();
        let app = Router::new()
            .route(TOKEN_PATH, post(token))
            .route("/public/v1/operations/{id}", get(operation))
            .route("/upload", put(upload))
            .with_state(state.clone());
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = ReveroClient::new_for_test(ReveroClientOptions::new(
            format!("http://{address}"),
            "rvc_test",
            "rvs_secret",
            vec![Scope::MeetingsWrite, Scope::MeetingsRead],
        ))
        .unwrap();
        (client, state)
    }

    #[tokio::test]
    async fn token_exchange_is_basic_form_encoded_cached_and_single_flight() {
        let (client, state) = client().await;
        let first = client.clone();
        let second = client.clone();

        let (a, b) = tokio::join!(first.access_token(), second.access_token());

        assert_eq!(a.unwrap(), "opaque-token");
        assert_eq!(b.unwrap(), "opaque-token");
        assert_eq!(state.token_calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn wait_honours_retry_after_and_returns_only_at_a_terminal_state() {
        let (client, state) = client().await;

        let operation = client
            .wait_for_operation(
                "operation-1",
                WaitOptions {
                    timeout: Duration::from_secs(1),
                    initial_delay: Duration::from_millis(1),
                    max_delay: Duration::from_millis(2),
                },
            )
            .await
            .unwrap();

        assert_eq!(operation.status, PublicOperationStatus::Completed);
        assert_eq!(state.operation_calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn upload_uses_the_presigned_request_verbatim_without_revero_auth() {
        let (client, state) = client().await;
        let upload = PresignedUpload {
            method: Some("PUT".into()),
            url: format!("{}/upload", client.base_url),
            headers: HashMap::from([("Content-Type".into(), "audio/webm".into())]),
            expires_at: DateTime::parse_from_rfc3339("2026-09-01T10:15:00+00:00").unwrap(),
        };

        client
            .upload_bytes(&upload, Bytes::from_static(b"audio"))
            .await
            .unwrap();

        assert_eq!(state.upload_calls.load(Ordering::SeqCst), 1);
        assert_eq!(state.token_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn production_client_rejects_plain_http() {
        let result = ReveroClient::new(ReveroClientOptions::new(
            "http://api.revero.test",
            "client",
            "secret",
            vec![Scope::MeetingsRead],
        ));
        assert!(matches!(result, Err(Error::InvalidConfiguration(_))));
    }
}
