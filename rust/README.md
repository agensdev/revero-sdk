# `revero`

The official Rust SDK for Revero's Public API. Fern generates endpoint methods,
wire models and typed API errors from the checked-in OpenAPI contract. A small,
separately tested layer adds the production Client Credentials exchange,
presigned upload, bounded operation polling and webhook verification.

The client is backend-only. Never compile a Revero client ID, client secret or
service access token into browser code. A browser receives only the short-lived
`PresignedUpload` returned for one recording.

```rust,no_run
use std::time::Duration;

use revero::{
    MeetingCreate, PublicOperationStatus, PublicOutputStatus, RecordingCreate,
    ReveroClient, ReveroClientOptions, Scope, WaitOptions,
};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let revero = ReveroClient::new(ReveroClientOptions::production(
    std::env::var("REVERO_CLIENT_ID")?,
    std::env::var("REVERO_CLIENT_SECRET")?,
    vec![
        Scope::MeetingsRead,
        Scope::MeetingsWrite,
        Scope::RecordingsWrite,
    ],
))?;
// `ReveroClientOptions::for_environment(Environment::Staging, ..)` targets
// staging, and `ReveroClientOptions::new(base_url, ..)` any other host.

let meeting = revero
    .create_meeting(&MeetingCreate {
        external_id: "your-meeting-2026-09-04".into(),
        language: Some("en".into()),
        output_profiles: Some(vec!["transcript".into(), "minutes".into()]),
        locked_context: Some(serde_json::from_value(serde_json::json!({
            "case_number": "ABC-123"
        }))?),
    })
    .await?;

let prepared = revero
    .prepare_recording(
        &meeting.id,
        &RecordingCreate {
            content_type: Some("audio/webm".into()),
        },
    )
    .await?;

// A browser normally performs this step with the exact upload instruction.
revero
    .upload_bytes(&prepared.upload, bytes::Bytes::from_static(b"audio bytes"))
    .await?;

let operation = revero
    .complete_recording(&prepared.id, "complete:your-meeting-2026-09-04")
    .await?;
let settled = revero
    .wait_for_operation(
        &operation.id,
        WaitOptions::new(Duration::from_secs(10 * 60)),
    )
    .await?;

if settled.status == PublicOperationStatus::Completed {
    for output in revero.list_outputs(&meeting.id).await? {
        if output.status == PublicOutputStatus::Completed {
            let document = revero.get_output(&output.id).await?;
            // Persist `document` in your own long-term storage.
        }
    }
}
# Ok(())
# }
```

See <https://docs.revero.no/welcome> for the complete Axum/React integration,
replay rules, stable errors and webhook reconciliation. Generated symbol
reference is published at <https://docs.rs/revero>.

## Generation

The private Flyt monorepo is the authoring source. Run
`make generate-api-revero-public` followed by
`scripts/generate-revero-rust-sdk.sh`. `.fernignore` preserves only the
hand-written README, license and `src/extensions`; the post-generation script
reconnects those helpers and their dependencies. CI reruns the same process and
fails on any diff.

Licensed under Apache-2.0.
