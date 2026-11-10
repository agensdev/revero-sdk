//! Verify and decode Revero's terminal webhook notifications.
//!
//! Revero follows Standard Webhooks (<https://www.standardwebhooks.com>):
//! `webhook-id`, `webhook-timestamp` and `webhook-signature` headers, signed
//! with the endpoint's `whsec_` secret over `{id}.{timestamp}.{body}`.

use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub use crate::{
    PublicWebhookData as WebhookData, PublicWebhookEnvelope as WebhookEnvelope,
    PublicWebhookEventType as WebhookEventType, PublicWebhookLinks as WebhookLinks,
};

pub const ID_HEADER: &str = "webhook-id";
pub const TIMESTAMP_HEADER: &str = "webhook-timestamp";
pub const SIGNATURE_HEADER: &str = "webhook-signature";
pub const SECRET_PREFIX: &str = "whsec_";
pub const DEFAULT_TOLERANCE_SECONDS: i64 = 300;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, thiserror::Error)]
pub enum WebhookError {
    #[error("the signing secret is not a whsec_ secret")]
    InvalidSecret,
    #[error("a webhook-id, webhook-timestamp or webhook-signature header is missing or malformed")]
    InvalidHeaders,
    #[error("the webhook timestamp is outside the accepted window")]
    TimestampOutsideTolerance,
    #[error("no v1 signature matches the supplied secret")]
    SignatureMismatch,
    #[error("the verified webhook body is not a valid Revero envelope: {0}")]
    InvalidEnvelope(#[from] serde_json::Error),
}

/// The three headers of one delivery, as the receiver read them.
#[derive(Debug, Clone, Copy)]
pub struct WebhookHeaders<'a> {
    pub id: &'a str,
    pub timestamp: &'a str,
    pub signature: &'a str,
}

/// The key behind a `whsec_<base64>` secret.
pub fn parse_secret(secret: &str) -> Result<Vec<u8>, WebhookError> {
    let encoded = secret
        .strip_prefix(SECRET_PREFIX)
        .ok_or(WebhookError::InvalidSecret)?;
    STANDARD
        .decode(encoded)
        .map_err(|_| WebhookError::InvalidSecret)
}

/// Verify one delivery's headers against its raw body with a `whsec_` secret.
///
/// Every `v1,` signature in `webhook-signature` is tried, so a delivery made
/// during a secret rotation verifies with either secret.
pub fn verify(
    headers: WebhookHeaders<'_>,
    secret: &str,
    body: &[u8],
    now: i64,
    tolerance_seconds: i64,
) -> Result<(), WebhookError> {
    let key = parse_secret(secret)?;
    if headers.id.is_empty() || headers.id.contains('.') {
        return Err(WebhookError::InvalidHeaders);
    }
    let timestamp: i64 = headers
        .timestamp
        .parse()
        .map_err(|_| WebhookError::InvalidHeaders)?;
    if now.abs_diff(timestamp) > tolerance_seconds.unsigned_abs() {
        return Err(WebhookError::TimestampOutsideTolerance);
    }
    let mut material = format!("{}.{}.", headers.id, timestamp).into_bytes();
    material.extend_from_slice(body);

    let mut saw_v1 = false;
    for entry in headers.signature.split_whitespace() {
        let Some((version, value)) = entry.split_once(',') else {
            return Err(WebhookError::InvalidHeaders);
        };
        if version != "v1" {
            continue;
        }
        saw_v1 = true;
        let Ok(expected) = STANDARD.decode(value) else {
            continue;
        };
        let Ok(mut mac) = HmacSha256::new_from_slice(&key) else {
            return Err(WebhookError::InvalidSecret);
        };
        mac.update(&material);
        if mac.verify_slice(&expected).is_ok() {
            return Ok(());
        }
    }
    if saw_v1 {
        Err(WebhookError::SignatureMismatch)
    } else {
        Err(WebhookError::InvalidHeaders)
    }
}

/// Verify the raw body before parsing it, then return the typed envelope.
pub fn verify_and_parse(
    headers: WebhookHeaders<'_>,
    secret: &str,
    body: &[u8],
    now: i64,
    tolerance_seconds: i64,
) -> Result<WebhookEnvelope, WebhookError> {
    verify(headers, secret, body, now, tolerance_seconds)?;
    Ok(serde_json::from_slice(body)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The Standard Webhooks reference libraries' own test vector.
    const SPEC_SECRET: &str = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";
    const SPEC_HEADERS: WebhookHeaders<'static> = WebhookHeaders {
        id: "msg_p5jXN8AQM9LWM0D4loKWxJek",
        timestamp: "1614265330",
        signature: "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE=",
    };
    const SPEC_BODY: &[u8] = br#"{"test": 2432232314}"#;
    const SPEC_NOW: i64 = 1_614_265_330;

    // Signed by the PyPI `standardwebhooks` package over a Revero envelope.
    const SECRET: &str = "whsec_cnJycnJycnJycnJycnJycnJycnJycnJycnJycnJycnI=";
    const OTHER_SECRET: &str = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";
    const NOW: i64 = 1_756_684_800;
    const BODY: &[u8] = br#"{"data":{"external_id":"customer-1","links":{"meeting":"https://api.revero.no/public/v1/meetings/meeting-1","operation":"https://api.revero.no/public/v1/operations/operation-1"},"meeting_id":"meeting-1","operation_id":"operation-1","operation_status":"completed"},"timestamp":"2026-09-01T09:15:00Z","type":"operation.completed"}"#;
    const SIGNATURE: &str = "v1,ZTQj2mQ88RBOrmVHpQ7eVxBUnW/KOWNDZD/Jnl2u9Ew=";

    fn headers(signature: &str) -> WebhookHeaders<'_> {
        WebhookHeaders {
            id: "event-1",
            timestamp: "1756684800",
            signature,
        }
    }

    #[test]
    fn the_spec_vector_verifies() {
        verify(
            SPEC_HEADERS,
            SPEC_SECRET,
            SPEC_BODY,
            SPEC_NOW,
            DEFAULT_TOLERANCE_SECONDS,
        )
        .unwrap();
    }

    #[test]
    fn a_changed_body_does_not_verify() {
        assert!(matches!(
            verify(
                SPEC_HEADERS,
                SPEC_SECRET,
                br#"{"test": 2432232315}"#,
                SPEC_NOW,
                DEFAULT_TOLERANCE_SECONDS
            ),
            Err(WebhookError::SignatureMismatch)
        ));
    }

    #[test]
    fn rotation_accepts_any_listed_signature() {
        let rotated = format!("v1,AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA= {SIGNATURE}");
        verify(
            headers(&rotated),
            SECRET,
            BODY,
            NOW,
            DEFAULT_TOLERANCE_SECONDS,
        )
        .unwrap();
        assert!(matches!(
            verify(
                headers(SIGNATURE),
                OTHER_SECRET,
                BODY,
                NOW,
                DEFAULT_TOLERANCE_SECONDS
            ),
            Err(WebhookError::SignatureMismatch)
        ));
    }

    #[test]
    fn a_stale_or_future_timestamp_is_refused() {
        for now in [NOW + 301, NOW - 301] {
            assert!(matches!(
                verify(
                    headers(SIGNATURE),
                    SECRET,
                    BODY,
                    now,
                    DEFAULT_TOLERANCE_SECONDS
                ),
                Err(WebhookError::TimestampOutsideTolerance)
            ));
        }
    }

    #[test]
    fn malformed_input_is_refused() {
        assert!(matches!(
            verify(headers(SIGNATURE), "not-a-secret", BODY, NOW, 300),
            Err(WebhookError::InvalidSecret)
        ));
        assert!(matches!(
            verify(headers(""), SECRET, BODY, NOW, 300),
            Err(WebhookError::InvalidHeaders)
        ));
        assert!(matches!(
            verify(headers("v1a,xyz"), SECRET, BODY, NOW, 300),
            Err(WebhookError::InvalidHeaders)
        ));
    }

    #[test]
    fn a_verified_body_decodes_to_the_typed_envelope() {
        let envelope = verify_and_parse(
            headers(SIGNATURE),
            SECRET,
            BODY,
            NOW,
            DEFAULT_TOLERANCE_SECONDS,
        )
        .unwrap();
        assert_eq!(envelope.r#type, WebhookEventType::OperationCompleted);
        assert_eq!(
            envelope.data.operation_status,
            Some(crate::PublicOperationStatus::Completed)
        );
        assert_eq!(
            envelope.data.links.operation.as_deref(),
            Some("https://api.revero.no/public/v1/operations/operation-1")
        );
    }
}
