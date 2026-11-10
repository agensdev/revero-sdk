pub use crate::prelude::*;

/// One document your organisation may ask a meeting to produce.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OutputProfileRead {
    /// The stable key to send in `output_profiles`. Never renamed.
    #[serde(default)]
    pub key: String,
    /// The profile version a meeting created now would pin. Bumped when what this key produces changes meaningfully; existing meetings keep the version they pinned.
    #[serde(default)]
    pub version: i64,
    /// The shape of the output resource this profile yields, as reported on `OutputRead.schema_version`.
    #[serde(default)]
    pub schema_version: String,
    /// What the document contains, in one line.
    #[serde(default)]
    pub description: String,
    /// Whether producing it needs a note-generation run. A profile that does not is derived from the transcript alone and survives a generation outage.
    #[serde(default)]
    pub requires_generation: bool,
}

impl OutputProfileRead {
    pub fn builder() -> OutputProfileReadBuilder {
        <OutputProfileReadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutputProfileReadBuilder {
    key: Option<String>,
    version: Option<i64>,
    schema_version: Option<String>,
    description: Option<String>,
    requires_generation: Option<bool>,
}

impl OutputProfileReadBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn schema_version(mut self, value: impl Into<String>) -> Self {
        self.schema_version = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn requires_generation(mut self, value: bool) -> Self {
        self.requires_generation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OutputProfileRead`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](OutputProfileReadBuilder::key)
    /// - [`version`](OutputProfileReadBuilder::version)
    /// - [`schema_version`](OutputProfileReadBuilder::schema_version)
    /// - [`description`](OutputProfileReadBuilder::description)
    /// - [`requires_generation`](OutputProfileReadBuilder::requires_generation)
    pub fn build(self) -> Result<OutputProfileRead, BuildError> {
        Ok(OutputProfileRead {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            schema_version: self
                .schema_version
                .ok_or_else(|| BuildError::missing_field("schema_version"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            requires_generation: self
                .requires_generation
                .ok_or_else(|| BuildError::missing_field("requires_generation"))?,
        })
    }
}
