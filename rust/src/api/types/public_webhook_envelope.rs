pub use crate::prelude::*;

/// A Standard Webhooks body: a terminal public resource state, signed and
/// delivered at least once. The event ID is the `webhook-id` header.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicWebhookEnvelope {
    /// Terminal event type.
    pub r#type: PublicWebhookEventType,
    /// When the event happened, UTC.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Identifiers, terminal state and authenticated links.
    #[serde(default)]
    pub data: PublicWebhookData,
}

impl PublicWebhookEnvelope {
    pub fn builder() -> PublicWebhookEnvelopeBuilder {
        <PublicWebhookEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicWebhookEnvelopeBuilder {
    r#type: Option<PublicWebhookEventType>,
    timestamp: Option<DateTime<FixedOffset>>,
    data: Option<PublicWebhookData>,
}

impl PublicWebhookEnvelopeBuilder {
    pub fn r#type(mut self, value: PublicWebhookEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn data(mut self, value: PublicWebhookData) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicWebhookEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PublicWebhookEnvelopeBuilder::r#type)
    /// - [`timestamp`](PublicWebhookEnvelopeBuilder::timestamp)
    /// - [`data`](PublicWebhookEnvelopeBuilder::data)
    pub fn build(self) -> Result<PublicWebhookEnvelope, BuildError> {
        Ok(PublicWebhookEnvelope {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
