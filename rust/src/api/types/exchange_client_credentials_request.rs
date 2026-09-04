pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExchangeClientCredentialsRequest {
    pub grant_type: ExchangeClientCredentialsRequestGrantType,
    #[serde(default)]
    pub resource: String,
    /// Space-separated OAuth scopes
    #[serde(default)]
    pub scope: String,
}

impl ExchangeClientCredentialsRequest {
    pub fn builder() -> ExchangeClientCredentialsRequestBuilder {
        <ExchangeClientCredentialsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExchangeClientCredentialsRequestBuilder {
    grant_type: Option<ExchangeClientCredentialsRequestGrantType>,
    resource: Option<String>,
    scope: Option<String>,
}

impl ExchangeClientCredentialsRequestBuilder {
    pub fn grant_type(mut self, value: ExchangeClientCredentialsRequestGrantType) -> Self {
        self.grant_type = Some(value);
        self
    }

    pub fn resource(mut self, value: impl Into<String>) -> Self {
        self.resource = Some(value.into());
        self
    }

    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExchangeClientCredentialsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`grant_type`](ExchangeClientCredentialsRequestBuilder::grant_type)
    /// - [`resource`](ExchangeClientCredentialsRequestBuilder::resource)
    /// - [`scope`](ExchangeClientCredentialsRequestBuilder::scope)
    pub fn build(self) -> Result<ExchangeClientCredentialsRequest, BuildError> {
        Ok(ExchangeClientCredentialsRequest {
            grant_type: self
                .grant_type
                .ok_or_else(|| BuildError::missing_field("grant_type"))?,
            resource: self
                .resource
                .ok_or_else(|| BuildError::missing_field("resource"))?,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
        })
    }
}
