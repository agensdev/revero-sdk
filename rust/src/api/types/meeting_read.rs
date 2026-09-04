pub use crate::prelude::*;

/// The aggregate: one meeting, its recordings and its declared documents.
///
/// Safe to poll within the published rate limit — this read is metadata only
/// and is not audited per call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeetingRead {
    /// Revero's identifier for this meeting.
    #[serde(default)]
    pub id: String,
    /// Your identifier, exactly as you sent it.
    #[serde(default)]
    pub external_id: String,
    /// Coarse state of the meeting as a whole.
    pub status: PublicMeetingStatus,
    /// The language this meeting is processed in.
    #[serde(default)]
    pub language: String,
    /// Exactly the facts you supplied at creation. Returned beside the generated documents rather than inside them, and never derived from one — a model cannot promote its own guess into this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_context: Option<HashMap<String, serde_json::Value>>,
    /// When the meeting was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When this resource last changed.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Audio uploaded for this meeting, as metadata.
    #[serde(default)]
    pub recordings: Vec<RecordingRead>,
    /// The documents this meeting declared, as metadata. Fetch each one's content from its own resource.
    #[serde(default)]
    pub outputs: Vec<OutputRead>,
}

impl MeetingRead {
    pub fn builder() -> MeetingReadBuilder {
        <MeetingReadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeetingReadBuilder {
    id: Option<String>,
    external_id: Option<String>,
    status: Option<PublicMeetingStatus>,
    language: Option<String>,
    locked_context: Option<HashMap<String, serde_json::Value>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    recordings: Option<Vec<RecordingRead>>,
    outputs: Option<Vec<OutputRead>>,
}

impl MeetingReadBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: PublicMeetingStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn locked_context(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.locked_context = Some(value);
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

    pub fn recordings(mut self, value: Vec<RecordingRead>) -> Self {
        self.recordings = Some(value);
        self
    }

    pub fn outputs(mut self, value: Vec<OutputRead>) -> Self {
        self.outputs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeetingRead`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MeetingReadBuilder::id)
    /// - [`external_id`](MeetingReadBuilder::external_id)
    /// - [`status`](MeetingReadBuilder::status)
    /// - [`language`](MeetingReadBuilder::language)
    /// - [`created_at`](MeetingReadBuilder::created_at)
    /// - [`updated_at`](MeetingReadBuilder::updated_at)
    /// - [`recordings`](MeetingReadBuilder::recordings)
    /// - [`outputs`](MeetingReadBuilder::outputs)
    pub fn build(self) -> Result<MeetingRead, BuildError> {
        Ok(MeetingRead {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            language: self
                .language
                .ok_or_else(|| BuildError::missing_field("language"))?,
            locked_context: self.locked_context,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            recordings: self
                .recordings
                .ok_or_else(|| BuildError::missing_field("recordings"))?,
            outputs: self
                .outputs
                .ok_or_else(|| BuildError::missing_field("outputs"))?,
        })
    }
}
