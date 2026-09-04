use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::post;
use axum::{Json, Router};
use revero::webhooks::{self, WebhookEnvelope};
use revero::{
    MeetingCreate, OperationRead, PresignedUpload, RecordingCreate, ReveroClient,
    ReveroClientOptions, Scope, WaitOptions,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    revero: ReveroClient,
    webhook_ids: Arc<Mutex<HashSet<String>>>,
    webhook_secret: Arc<str>,
}

#[derive(Deserialize)]
struct CreateMeetingRequest {
    external_id: String,
    output_profiles: Vec<String>,
    locked_context: HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
struct BrowserUpload {
    recording_id: String,
    upload: PresignedUpload,
}

async fn create_meeting(
    State(state): State<AppState>,
    Json(request): Json<CreateMeetingRequest>,
) -> Result<Json<revero::MeetingRead>, StatusCode> {
    state
        .revero
        .create_meeting(&MeetingCreate {
            external_id: request.external_id,
            language: Some("en".into()),
            output_profiles: Some(request.output_profiles),
            locked_context: Some(request.locked_context),
        })
        .await
        .map(Json)
        .map_err(|_| StatusCode::BAD_GATEWAY)
}

async fn prepare_recording(
    State(state): State<AppState>,
    Path(meeting_id): Path<String>,
) -> Result<Json<BrowserUpload>, StatusCode> {
    let prepared = state
        .revero
        .prepare_recording(
            &meeting_id,
            &RecordingCreate {
                content_type: Some("audio/webm".into()),
            },
        )
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    Ok(Json(BrowserUpload {
        recording_id: prepared.id,
        upload: prepared.upload,
    }))
}

async fn complete_recording(
    State(state): State<AppState>,
    Path(recording_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<OperationRead>, StatusCode> {
    let key = headers
        .get("Idempotency-Key")
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;
    let accepted = state
        .revero
        .complete_recording(&recording_id, key)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    let settled = state
        .revero
        .wait_for_operation(
            &accepted.id,
            WaitOptions::new(Duration::from_secs(10 * 60)),
        )
        .await
        .map_err(|_| StatusCode::GATEWAY_TIMEOUT)?;
    Ok(Json(settled))
}

async fn webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, StatusCode> {
    let signature = headers
        .get(webhooks::SIGNATURE_HEADER)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .as_secs() as i64;
    let event: WebhookEnvelope = webhooks::verify_and_parse(
        signature,
        state.webhook_secret.as_bytes(),
        &body,
        now,
        webhooks::DEFAULT_TOLERANCE_SECONDS,
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;
    if state.webhook_ids.lock().await.insert(event.id) {
        let _authoritative = state
            .revero
            .get_meeting(&event.data.meeting_id)
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?;
    }
    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() {
    let revero = ReveroClient::new(ReveroClientOptions::new(
        std::env::var("REVERO_BASE_URL").expect("REVERO_BASE_URL"),
        std::env::var("REVERO_CLIENT_ID").expect("REVERO_CLIENT_ID"),
        std::env::var("REVERO_CLIENT_SECRET").expect("REVERO_CLIENT_SECRET"),
        vec![
            Scope::MeetingsRead,
            Scope::MeetingsWrite,
            Scope::RecordingsWrite,
        ],
    ))
    .expect("valid Revero configuration");
    let state = AppState {
        revero,
        webhook_ids: Arc::new(Mutex::new(HashSet::new())),
        webhook_secret: std::env::var("REVERO_WEBHOOK_SECRET")
            .expect("REVERO_WEBHOOK_SECRET")
            .into(),
    };
    let app = Router::new()
        .route("/meetings", post(create_meeting))
        .route("/meetings/{meeting_id}/recordings", post(prepare_recording))
        .route("/recordings/{recording_id}/complete", post(complete_recording))
        .route("/webhooks/revero", post(webhook))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
