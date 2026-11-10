use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PublicApiClient {
    pub http_client: HttpClient,
}

impl PublicApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the output profiles your organisation may name in `output_profiles`.
    ///
    /// Resolved for the calling organisation, not the platform catalogue: a
    /// document Revero restricts to selected organisations is absent here for
    /// everyone else, and `POST /meetings` refuses exactly what this omits. Each
    /// entry's `version` is what a meeting created now would pin.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use revero::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         username: Some("<username>".to_string()),
    ///         password: Some("<password>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    ///     client.public_api.list_public_output_profiles(None).await;
    /// }
    /// ```
    pub async fn list_public_output_profiles(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<OutputProfileRead>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "public/v1/output-profiles",
                None,
                None,
                options,
            )
            .await
    }

    /// Create a meeting, idempotent within the OAuth client by external_id.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use revero::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         username: Some("<username>".to_string()),
    ///         password: Some("<password>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    ///     client
    ///         .public_api
    ///         .create_public_meeting(
    ///             &MeetingCreate {
    ///                 external_id: "gm-2026-04-17".to_string(),
    ///                 language: None,
    ///                 output_profiles: None,
    ///                 locked_context: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_public_meeting(
        &self,
        request: &MeetingCreate,
        options: Option<RequestOptions>,
    ) -> Result<MeetingRead, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "public/v1/meetings",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one meeting owned by the calling OAuth client.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use revero::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         username: Some("<username>".to_string()),
    ///         password: Some("<password>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    ///     client
    ///         .public_api
    ///         .get_public_meeting(&"meeting_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_public_meeting(
        &self,
        meeting_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MeetingRead, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("public/v1/meetings/{}", meeting_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Create a recording and return a one-hour direct-upload request.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use revero::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         username: Some("<username>".to_string()),
    ///         password: Some("<password>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    ///     client
    ///         .public_api
    ///         .prepare_public_recording_upload(
    ///             &"meeting_id".to_string(),
    ///             &RecordingCreate {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn prepare_public_recording_upload(
        &self,
        meeting_id: &str,
        request: &RecordingCreate,
        options: Option<RequestOptions>,
    ) -> Result<RecordingPrepared, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("public/v1/meetings/{}/recordings", meeting_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Verify the uploaded object and accept transcription asynchronously.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use revero::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         username: Some("<username>".to_string()),
    ///         password: Some("<password>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    ///     client
    ///         .public_api
    ///         .complete_public_recording_upload(
    ///             &"recording_id".to_string(),
    ///             Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn complete_public_recording_upload(
        &self,
        recording_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<OperationRead, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("public/v1/recordings/{}/complete", recording_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List the documents this meeting declared, and where each one stands.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use revero::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         username: Some("<username>".to_string()),
    ///         password: Some("<password>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    ///     client
    ///         .public_api
    ///         .list_public_meeting_outputs(&"meeting_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_public_meeting_outputs(
        &self,
        meeting_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<OutputRead>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("public/v1/meetings/{}/outputs", meeting_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Read one generated output owned by the calling integration.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use revero::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         username: Some("<username>".to_string()),
    ///         password: Some("<password>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    ///     client
    ///         .public_api
    ///         .get_public_meeting_output(&"output_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_public_meeting_output(
        &self,
        output_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<OutputContentRead, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("public/v1/outputs/{}", output_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Poll the authoritative coarse state of one accepted command.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use revero::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         username: Some("<username>".to_string()),
    ///         password: Some("<password>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    ///     client
    ///         .public_api
    ///         .get_public_operation(&"operation_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_public_operation(
        &self,
        operation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<OperationRead, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("public/v1/operations/{}", operation_id),
                None,
                None,
                options,
            )
            .await
    }
}
