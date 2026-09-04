pub use crate::prelude::*;

/// Signed, at-least-once notification of a terminal public resource state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicWebhookEnvelope {
    /// Version of this webhook envelope contract.
    #[serde(default)]
    pub schema_version: String,
    /// Stable event ID. Deduplicate repeated delivery on this value.
    #[serde(default)]
    pub id: String,
    /// Terminal event type.
    pub r#type: PublicWebhookEventType,
    /// When Revero created the event.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Identifiers and terminal state.
    #[serde(default)]
    pub data: PublicWebhookData,
    /// Authenticated resources whose current state is authoritative.
    #[serde(default)]
    pub links: PublicWebhookLinks,
}

impl PublicWebhookEnvelope {
    pub fn builder() -> PublicWebhookEnvelopeBuilder {
        <PublicWebhookEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicWebhookEnvelopeBuilder {
    schema_version: Option<String>,
    id: Option<String>,
    r#type: Option<PublicWebhookEventType>,
    created_at: Option<DateTime<FixedOffset>>,
    data: Option<PublicWebhookData>,
    links: Option<PublicWebhookLinks>,
}

impl PublicWebhookEnvelopeBuilder {
    pub fn schema_version(mut self, value: impl Into<String>) -> Self {
        self.schema_version = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PublicWebhookEventType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn data(mut self, value: PublicWebhookData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn links(mut self, value: PublicWebhookLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicWebhookEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schema_version`](PublicWebhookEnvelopeBuilder::schema_version)
    /// - [`id`](PublicWebhookEnvelopeBuilder::id)
    /// - [`r#type`](PublicWebhookEnvelopeBuilder::r#type)
    /// - [`created_at`](PublicWebhookEnvelopeBuilder::created_at)
    /// - [`data`](PublicWebhookEnvelopeBuilder::data)
    /// - [`links`](PublicWebhookEnvelopeBuilder::links)
    pub fn build(self) -> Result<PublicWebhookEnvelope, BuildError> {
        Ok(PublicWebhookEnvelope {
            schema_version: self
                .schema_version
                .ok_or_else(|| BuildError::missing_field("schema_version"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
