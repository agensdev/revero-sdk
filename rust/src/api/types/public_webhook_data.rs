pub use crate::prelude::*;

/// Identifiers and terminal state carried by a public webhook event.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PublicWebhookData {
    /// Meeting to reconcile with the Public API.
    #[serde(default)]
    pub meeting_id: String,
    /// Your identifier for the meeting.
    #[serde(default)]
    pub external_id: String,
    /// Terminal meeting state when the event describes an operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_status: Option<PublicMeetingStatus>,
    /// Operation associated with the event.
    #[serde(default)]
    pub operation_id: String,
    /// Terminal operation state for an operation event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<PublicOperationStatus>,
    /// Final informational phase for an operation event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_phase: Option<PublicOperationPhase>,
    /// Output associated with an output event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_id: Option<String>,
    /// Terminal output state for an output event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_status: Option<PublicOutputStatus>,
    /// Stable output profile key for an output event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_key: Option<String>,
    /// Pinned output profile version for an output event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_version: Option<i64>,
    /// Public content schema version for an output event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    /// Stable failure reason for a failed operation or output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<PublicFailureCode>,
    /// Whether a failed operation may succeed when retried.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retryable: Option<bool>,
    /// Authenticated resources whose current state is authoritative.
    #[serde(default)]
    pub links: PublicWebhookLinks,
}

impl PublicWebhookData {
    pub fn builder() -> PublicWebhookDataBuilder {
        <PublicWebhookDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicWebhookDataBuilder {
    meeting_id: Option<String>,
    external_id: Option<String>,
    meeting_status: Option<PublicMeetingStatus>,
    operation_id: Option<String>,
    operation_status: Option<PublicOperationStatus>,
    operation_phase: Option<PublicOperationPhase>,
    output_id: Option<String>,
    output_status: Option<PublicOutputStatus>,
    profile_key: Option<String>,
    profile_version: Option<i64>,
    schema_version: Option<String>,
    failure_code: Option<PublicFailureCode>,
    retryable: Option<bool>,
    links: Option<PublicWebhookLinks>,
}

impl PublicWebhookDataBuilder {
    pub fn meeting_id(mut self, value: impl Into<String>) -> Self {
        self.meeting_id = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn meeting_status(mut self, value: PublicMeetingStatus) -> Self {
        self.meeting_status = Some(value);
        self
    }

    pub fn operation_id(mut self, value: impl Into<String>) -> Self {
        self.operation_id = Some(value.into());
        self
    }

    pub fn operation_status(mut self, value: PublicOperationStatus) -> Self {
        self.operation_status = Some(value);
        self
    }

    pub fn operation_phase(mut self, value: PublicOperationPhase) -> Self {
        self.operation_phase = Some(value);
        self
    }

    pub fn output_id(mut self, value: impl Into<String>) -> Self {
        self.output_id = Some(value.into());
        self
    }

    pub fn output_status(mut self, value: PublicOutputStatus) -> Self {
        self.output_status = Some(value);
        self
    }

    pub fn profile_key(mut self, value: impl Into<String>) -> Self {
        self.profile_key = Some(value.into());
        self
    }

    pub fn profile_version(mut self, value: i64) -> Self {
        self.profile_version = Some(value);
        self
    }

    pub fn schema_version(mut self, value: impl Into<String>) -> Self {
        self.schema_version = Some(value.into());
        self
    }

    pub fn failure_code(mut self, value: PublicFailureCode) -> Self {
        self.failure_code = Some(value);
        self
    }

    pub fn retryable(mut self, value: bool) -> Self {
        self.retryable = Some(value);
        self
    }

    pub fn links(mut self, value: PublicWebhookLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicWebhookData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`meeting_id`](PublicWebhookDataBuilder::meeting_id)
    /// - [`external_id`](PublicWebhookDataBuilder::external_id)
    /// - [`operation_id`](PublicWebhookDataBuilder::operation_id)
    /// - [`links`](PublicWebhookDataBuilder::links)
    pub fn build(self) -> Result<PublicWebhookData, BuildError> {
        Ok(PublicWebhookData {
            meeting_id: self
                .meeting_id
                .ok_or_else(|| BuildError::missing_field("meeting_id"))?,
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            meeting_status: self.meeting_status,
            operation_id: self
                .operation_id
                .ok_or_else(|| BuildError::missing_field("operation_id"))?,
            operation_status: self.operation_status,
            operation_phase: self.operation_phase,
            output_id: self.output_id,
            output_status: self.output_status,
            profile_key: self.profile_key,
            profile_version: self.profile_version,
            schema_version: self.schema_version,
            failure_code: self.failure_code,
            retryable: self.retryable,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
