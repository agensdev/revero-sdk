//! Verify and decode Revero's signed, terminal webhook notifications.

use hmac::{Hmac, Mac};
use sha2::Sha256;

pub use crate::{
    PublicWebhookData as WebhookData, PublicWebhookEnvelope as WebhookEnvelope,
    PublicWebhookEventType as WebhookEventType, PublicWebhookLinks as WebhookLinks,
};

pub const SIGNATURE_HEADER: &str = "Flyt-Signature";
pub const EVENT_ID_HEADER: &str = "Flyt-Event-Id";
pub const EVENT_TYPE_HEADER: &str = "Flyt-Event-Type";
pub const DEFAULT_TOLERANCE_SECONDS: i64 = 300;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, thiserror::Error)]
pub enum WebhookError {
    #[error("the Flyt-Signature header is missing or malformed")]
    InvalidSignature,
    #[error("the webhook signature timestamp is outside the accepted window")]
    TimestampOutsideTolerance,
    #[error("no v1 signature matches the supplied secret")]
    SignatureMismatch,
    #[error("the verified webhook body is not a valid Revero envelope: {0}")]
    InvalidEnvelope(#[from] serde_json::Error),
}

fn signed_material(timestamp: i64, body: &[u8]) -> Vec<u8> {
    let mut material = timestamp.to_string().into_bytes();
    material.push(b'.');
    material.extend_from_slice(body);
    material
}

pub fn parse_signature(header: &str) -> Result<(i64, Vec<&str>), WebhookError> {
    let mut timestamp = None;
    let mut signatures = Vec::new();
    for chunk in header.split(',') {
        let Some((key, value)) = chunk.trim().split_once('=') else {
            continue;
        };
        match key {
            "t" => {
                timestamp = Some(
                    value
                        .parse::<i64>()
                        .map_err(|_| WebhookError::InvalidSignature)?,
                );
            }
            "v1" if !value.is_empty() => signatures.push(value),
            _ => {}
        }
    }
    match (timestamp, signatures.is_empty()) {
        (Some(timestamp), false) => Ok((timestamp, signatures)),
        _ => Err(WebhookError::InvalidSignature),
    }
}

fn decode_hex(value: &str) -> Option<Vec<u8>> {
    if !value.len().is_multiple_of(2) {
        return None;
    }
    (0..value.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&value[index..index + 2], 16).ok())
        .collect()
}

pub fn verify(
    header: &str,
    secret: &[u8],
    body: &[u8],
    now: i64,
    tolerance_seconds: i64,
) -> Result<(), WebhookError> {
    let (timestamp, signatures) = parse_signature(header)?;
    if now.abs_diff(timestamp) > tolerance_seconds.unsigned_abs() {
        return Err(WebhookError::TimestampOutsideTolerance);
    }
    let material = signed_material(timestamp, body);
    let matches = signatures.iter().any(|candidate| {
        let Some(expected) = decode_hex(candidate) else {
            return false;
        };
        let Ok(mut mac) = HmacSha256::new_from_slice(secret) else {
            return false;
        };
        mac.update(&material);
        mac.verify_slice(&expected).is_ok()
    });
    if matches {
        Ok(())
    } else {
        Err(WebhookError::SignatureMismatch)
    }
}

/// Verify the raw body before parsing it, then return the typed envelope.
pub fn verify_and_parse(
    header: &str,
    secret: &[u8],
    body: &[u8],
    now: i64,
    tolerance_seconds: i64,
) -> Result<WebhookEnvelope, WebhookError> {
    verify(header, secret, body, now, tolerance_seconds)?;
    Ok(serde_json::from_slice(body)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: i64 = 1_756_684_800;
    const CURRENT: &[u8] = b"whsec_current_secret_value";
    const PREVIOUS: &[u8] = b"whsec_previous_secret_value";
    const ROTATION_HEADER: &str = "t=1756684800,v1=dff8ae8f3b0c2f4a5faf33e67c0ee2c7927ca618dabca50754ca2d0634b05117,v1=aaadf09884d91dc99146fb056be464dcb22c58ba94c1194f22978fd2d9480a86";
    const SIGNED_BODY: &[u8] =
        br#"{"id":"9f0c2b0e-5f7a-4d2e-9d0a-1b2c3d4e5f60","type":"operation.completed"}"#;

    #[test]
    fn rotation_accepts_each_live_secret_and_rejects_tampering() {
        assert!(verify(
            ROTATION_HEADER,
            CURRENT,
            SIGNED_BODY,
            NOW,
            DEFAULT_TOLERANCE_SECONDS
        )
        .is_ok());
        assert!(verify(
            ROTATION_HEADER,
            PREVIOUS,
            SIGNED_BODY,
            NOW,
            DEFAULT_TOLERANCE_SECONDS
        )
        .is_ok());
        assert!(matches!(
            verify(
                ROTATION_HEADER,
                CURRENT,
                b"tampered",
                NOW,
                DEFAULT_TOLERANCE_SECONDS
            ),
            Err(WebhookError::SignatureMismatch)
        ));
    }

    #[test]
    fn verified_body_decodes_to_the_typed_envelope() {
        let body = br#"{"schema_version":"2026-09-01","id":"event-1","type":"operation.completed","created_at":"2026-09-01T09:15:00+00:00","data":{"meeting_id":"meeting-1","external_id":"customer-1","operation_id":"operation-1","operation_status":"completed"},"links":{"meeting":"https://api.revero.no/public/v1/meetings/meeting-1","operation":"https://api.revero.no/public/v1/operations/operation-1"}}"#;
        let mut mac = HmacSha256::new_from_slice(CURRENT).unwrap();
        mac.update(&signed_material(NOW, body));
        let header = format!("t={NOW},v1={}", hex(&mac.finalize().into_bytes()));

        let envelope =
            verify_and_parse(&header, CURRENT, body, NOW, DEFAULT_TOLERANCE_SECONDS).unwrap();

        assert_eq!(envelope.r#type, WebhookEventType::OperationCompleted);
        assert_eq!(
            envelope.data.operation_status,
            Some(crate::PublicOperationStatus::Completed)
        );
        assert_eq!(
            envelope.links.operation.as_deref(),
            Some("https://api.revero.no/public/v1/operations/operation-1")
        );
    }

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}
