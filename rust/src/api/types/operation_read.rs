pub use crate::prelude::*;

/// The durable receipt for one accepted command — poll this.
///
/// After a dropped connection, a closed browser or a missed notification,
/// reading this resource is enough to learn what happened.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OperationRead {
    /// Revero's identifier for this operation.
    #[serde(default)]
    pub id: String,
    /// The meeting the command acted on.
    #[serde(default)]
    pub meeting_id: String,
    /// Which command this receipt is for.
    #[serde(default)]
    pub command: String,
    /// One of five coarse states. `queued` and `processing` mean work is outstanding; the other three are terminal and will not change.
    pub status: PublicOperationStatus,
    /// Which stage the work has reached. Informational, for showing progress — do not branch on it; branch on `status`.
    pub phase: PublicOperationPhase,
    /// Why the operation failed, or null. A closed set of stable values — branch on this rather than on the HTTP status or the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<PublicFailureCode>,
    /// Whether re-sending could succeed at all. Null until the operation fails. False means a retry would fail identically, but the reason differs by `failure_code`, and only some need an administrator, so branch on the code rather than on this alone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retryable: Option<bool>,
    /// When the command was accepted.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When this resource last changed.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// When it reached a terminal state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<FixedOffset>>,
}

impl OperationRead {
    pub fn builder() -> OperationReadBuilder {
        <OperationReadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OperationReadBuilder {
    id: Option<String>,
    meeting_id: Option<String>,
    command: Option<String>,
    status: Option<PublicOperationStatus>,
    phase: Option<PublicOperationPhase>,
    failure_code: Option<PublicFailureCode>,
    retryable: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    completed_at: Option<DateTime<FixedOffset>>,
}

impl OperationReadBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn meeting_id(mut self, value: impl Into<String>) -> Self {
        self.meeting_id = Some(value.into());
        self
    }

    pub fn command(mut self, value: impl Into<String>) -> Self {
        self.command = Some(value.into());
        self
    }

    pub fn status(mut self, value: PublicOperationStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn phase(mut self, value: PublicOperationPhase) -> Self {
        self.phase = Some(value);
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OperationRead`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OperationReadBuilder::id)
    /// - [`meeting_id`](OperationReadBuilder::meeting_id)
    /// - [`command`](OperationReadBuilder::command)
    /// - [`status`](OperationReadBuilder::status)
    /// - [`phase`](OperationReadBuilder::phase)
    /// - [`created_at`](OperationReadBuilder::created_at)
    /// - [`updated_at`](OperationReadBuilder::updated_at)
    pub fn build(self) -> Result<OperationRead, BuildError> {
        Ok(OperationRead {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            meeting_id: self
                .meeting_id
                .ok_or_else(|| BuildError::missing_field("meeting_id"))?,
            command: self
                .command
                .ok_or_else(|| BuildError::missing_field("command"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            phase: self
                .phase
                .ok_or_else(|| BuildError::missing_field("phase"))?,
            failure_code: self.failure_code,
            retryable: self.retryable,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            completed_at: self.completed_at,
        })
    }
}
