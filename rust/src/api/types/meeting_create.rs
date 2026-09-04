pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MeetingCreate {
    /// Your own identifier for this meeting. Creation is idempotent on it: re-sending an identical request returns the same meeting with 200 instead of 201. Re-using it for a DIFFERENT request (other documents, another language) is a 409 `external_id_conflict`.
    #[serde(default)]
    pub external_id: String,
    /// Spoken language, for transcription and generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Which documents this meeting should produce, by stable profile key. Each becomes one output resource you fetch separately. The profile VERSION is pinned now: publishing a new version later never changes what this meeting produces. Defaults to the transcript alone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_profiles: Option<Vec<String>>,
    /// Your authoritative facts about this meeting — a case number, participants, a date. Stored encrypted, returned as its own field, and never rewritten by generation: your system stays authoritative for legal and business facts. Captured ONCE, on the request that creates the meeting, so a later call with the same external_id cannot restate it under documents already generated from it. At most 16384 bytes serialized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked_context: Option<HashMap<String, serde_json::Value>>,
}

impl MeetingCreate {
    pub fn builder() -> MeetingCreateBuilder {
        <MeetingCreateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeetingCreateBuilder {
    external_id: Option<String>,
    language: Option<String>,
    output_profiles: Option<Vec<String>>,
    locked_context: Option<HashMap<String, serde_json::Value>>,
}

impl MeetingCreateBuilder {
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn output_profiles(mut self, value: Vec<String>) -> Self {
        self.output_profiles = Some(value);
        self
    }

    pub fn locked_context(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.locked_context = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeetingCreate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_id`](MeetingCreateBuilder::external_id)
    pub fn build(self) -> Result<MeetingCreate, BuildError> {
        Ok(MeetingCreate {
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            language: self.language,
            output_profiles: self.output_profiles,
            locked_context: self.locked_context,
        })
    }
}
