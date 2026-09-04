pub use crate::prelude::*;

/// Authenticated Public API resources used to reconcile an event.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PublicWebhookLinks {
    /// Authoritative meeting resource URL.
    #[serde(default)]
    pub meeting: String,
    /// Authoritative operation resource URL when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// Authoritative output resource URL when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

impl PublicWebhookLinks {
    pub fn builder() -> PublicWebhookLinksBuilder {
        <PublicWebhookLinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicWebhookLinksBuilder {
    meeting: Option<String>,
    operation: Option<String>,
    output: Option<String>,
}

impl PublicWebhookLinksBuilder {
    pub fn meeting(mut self, value: impl Into<String>) -> Self {
        self.meeting = Some(value.into());
        self
    }

    pub fn operation(mut self, value: impl Into<String>) -> Self {
        self.operation = Some(value.into());
        self
    }

    pub fn output(mut self, value: impl Into<String>) -> Self {
        self.output = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PublicWebhookLinks`].
    /// This method will fail if any of the following fields are not set:
    /// - [`meeting`](PublicWebhookLinksBuilder::meeting)
    pub fn build(self) -> Result<PublicWebhookLinks, BuildError> {
        Ok(PublicWebhookLinks {
            meeting: self
                .meeting
                .ok_or_else(|| BuildError::missing_field("meeting"))?,
            operation: self.operation,
            output: self.output,
        })
    }
}
