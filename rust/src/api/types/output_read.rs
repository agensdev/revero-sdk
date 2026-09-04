pub use crate::prelude::*;

/// One declared document, as metadata. Fetch its content separately.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OutputRead {
    /// Fetch this document's content from `/outputs/{id}`.
    #[serde(default)]
    pub id: String,
    /// The meeting this document belongs to.
    #[serde(default)]
    pub meeting_id: String,
    /// Which document this is, as named when the meeting was created.
    #[serde(default)]
    pub profile_key: String,
    /// Pinned when the meeting was created. A newer version published since then does not apply to this document.
    #[serde(default)]
    pub profile_version: i64,
    /// Version of the public payload shape used by `content` and `structured_data` for this output.
    #[serde(default)]
    pub schema_version: String,
    /// Where this document stands. Content is readable at `completed`.
    pub status: PublicOutputStatus,
    /// Why this document failed, or null. Branch on this rather than on prose; see the operation's `retryable` for whether re-sending could help.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<PublicFailureCode>,
    /// When the document was declared.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When this resource last changed.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// When it became readable. Null until then.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<FixedOffset>>,
}

impl OutputRead {
    pub fn builder() -> OutputReadBuilder {
        <OutputReadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutputReadBuilder {
    id: Option<String>,
    meeting_id: Option<String>,
    profile_key: Option<String>,
    profile_version: Option<i64>,
    schema_version: Option<String>,
    status: Option<PublicOutputStatus>,
    failure_code: Option<PublicFailureCode>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    completed_at: Option<DateTime<FixedOffset>>,
}

impl OutputReadBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn meeting_id(mut self, value: impl Into<String>) -> Self {
        self.meeting_id = Some(value.into());
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

    pub fn status(mut self, value: PublicOutputStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn failure_code(mut self, value: PublicFailureCode) -> Self {
        self.failure_code = Some(value);
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

    /// Consumes the builder and constructs a [`OutputRead`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OutputReadBuilder::id)
    /// - [`meeting_id`](OutputReadBuilder::meeting_id)
    /// - [`profile_key`](OutputReadBuilder::profile_key)
    /// - [`profile_version`](OutputReadBuilder::profile_version)
    /// - [`schema_version`](OutputReadBuilder::schema_version)
    /// - [`status`](OutputReadBuilder::status)
    /// - [`created_at`](OutputReadBuilder::created_at)
    /// - [`updated_at`](OutputReadBuilder::updated_at)
    pub fn build(self) -> Result<OutputRead, BuildError> {
        Ok(OutputRead {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            meeting_id: self
                .meeting_id
                .ok_or_else(|| BuildError::missing_field("meeting_id"))?,
            profile_key: self
                .profile_key
                .ok_or_else(|| BuildError::missing_field("profile_key"))?,
            profile_version: self
                .profile_version
                .ok_or_else(|| BuildError::missing_field("profile_version"))?,
            schema_version: self
                .schema_version
                .ok_or_else(|| BuildError::missing_field("schema_version"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            failure_code: self.failure_code,
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
