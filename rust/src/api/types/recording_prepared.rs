pub use crate::prelude::*;

/// A recording plus the one-time grant to upload its audio.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordingPrepared {
    /// Revero's identifier for this recording.
    #[serde(default)]
    pub id: String,
    /// The meeting this recording belongs to.
    #[serde(default)]
    pub meeting_id: String,
    /// Ingest state. Terminal values will not change again.
    pub status: PublicRecordingStatus,
    /// The MIME type declared when the upload was prepared.
    #[serde(default)]
    pub content_type: String,
    /// Size of the uploaded object as Revero verified it. Null until the upload has been completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    /// When Revero verified the object in storage — not when your browser finished sending it. Null until completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<DateTime<FixedOffset>>,
    /// When the upload was prepared.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When this resource last changed.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// This is the only part of the response safe to give browser code.
    #[serde(default)]
    pub upload: PresignedUpload,
}

impl RecordingPrepared {
    pub fn builder() -> RecordingPreparedBuilder {
        <RecordingPreparedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordingPreparedBuilder {
    id: Option<String>,
    meeting_id: Option<String>,
    status: Option<PublicRecordingStatus>,
    content_type: Option<String>,
    size_bytes: Option<i64>,
    uploaded_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    upload: Option<PresignedUpload>,
}

impl RecordingPreparedBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn meeting_id(mut self, value: impl Into<String>) -> Self {
        self.meeting_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: PublicRecordingStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    pub fn uploaded_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.uploaded_at = Some(value);
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

    pub fn upload(mut self, value: PresignedUpload) -> Self {
        self.upload = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordingPrepared`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RecordingPreparedBuilder::id)
    /// - [`meeting_id`](RecordingPreparedBuilder::meeting_id)
    /// - [`status`](RecordingPreparedBuilder::status)
    /// - [`content_type`](RecordingPreparedBuilder::content_type)
    /// - [`created_at`](RecordingPreparedBuilder::created_at)
    /// - [`updated_at`](RecordingPreparedBuilder::updated_at)
    /// - [`upload`](RecordingPreparedBuilder::upload)
    pub fn build(self) -> Result<RecordingPrepared, BuildError> {
        Ok(RecordingPrepared {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            meeting_id: self
                .meeting_id
                .ok_or_else(|| BuildError::missing_field("meeting_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            content_type: self
                .content_type
                .ok_or_else(|| BuildError::missing_field("content_type"))?,
            size_bytes: self.size_bytes,
            uploaded_at: self.uploaded_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            upload: self
                .upload
                .ok_or_else(|| BuildError::missing_field("upload"))?,
        })
    }
}
