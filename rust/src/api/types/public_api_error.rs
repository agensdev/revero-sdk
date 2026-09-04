pub use crate::prelude::*;

/// One refusal, as a stable code plus prose a human reads.
///
/// Every `/public/v1` error uses this flat shape. The code is the stable
/// protocol; the message is explanatory prose and may be reworded.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PublicApiError {
    /// Stable, machine-readable reason for the refusal
    #[serde(default)]
    pub code: String,
    /// Human-readable explanation. Never parse this.
    #[serde(default)]
    pub message: String,
}

impl PublicApiError {
    pub fn builder() -> PublicApiErrorBuilder {
        <PublicApiErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicApiErrorBuilder {
    code: Option<String>,
    message: Option<String>,
}

impl PublicApiErrorBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PublicApiError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PublicApiErrorBuilder::code)
    /// - [`message`](PublicApiErrorBuilder::message)
    pub fn build(self) -> Result<PublicApiError, BuildError> {
        Ok(PublicApiError {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
