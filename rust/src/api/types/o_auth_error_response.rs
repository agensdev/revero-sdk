pub use crate::prelude::*;

/// RFC 6749 error returned by the token endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OAuthErrorResponse {
    /// Machine-readable OAuth error code.
    #[serde(default)]
    pub error: String,
    /// Safe description of why the token request failed.
    #[serde(default)]
    pub error_description: String,
}

impl OAuthErrorResponse {
    pub fn builder() -> OAuthErrorResponseBuilder {
        <OAuthErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OAuthErrorResponseBuilder {
    error: Option<String>,
    error_description: Option<String>,
}

impl OAuthErrorResponseBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn error_description(mut self, value: impl Into<String>) -> Self {
        self.error_description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OAuthErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](OAuthErrorResponseBuilder::error)
    /// - [`error_description`](OAuthErrorResponseBuilder::error_description)
    pub fn build(self) -> Result<OAuthErrorResponse, BuildError> {
        Ok(OAuthErrorResponse {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
            error_description: self
                .error_description
                .ok_or_else(|| BuildError::missing_field("error_description"))?,
        })
    }
}
