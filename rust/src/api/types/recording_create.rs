pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecordingCreate {
    /// MIME type of the audio. The presigned request is issued for exactly this type, and completion refuses an object whose stored type differs — send the same value as the `Content-Type` header on your upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl RecordingCreate {
    pub fn builder() -> RecordingCreateBuilder {
        <RecordingCreateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordingCreateBuilder {
    content_type: Option<String>,
}

impl RecordingCreateBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecordingCreate`].
    pub fn build(self) -> Result<RecordingCreate, BuildError> {
        Ok(RecordingCreate {
            content_type: self.content_type,
        })
    }
}
