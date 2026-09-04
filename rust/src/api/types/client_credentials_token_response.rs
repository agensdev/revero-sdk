pub use crate::prelude::*;

/// Successful OAuth Client Credentials response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ClientCredentialsTokenResponse {
    /// Opaque bearer token. Keep it on the integrating backend.
    #[serde(default)]
    pub access_token: String,
    /// Authorization scheme used when calling the Public API.
    pub token_type: ClientCredentialsTokenResponseTokenType,
    /// Lifetime of the access token in seconds.
    #[serde(default)]
    pub expires_in: i64,
    /// Granted OAuth scopes, separated by spaces.
    #[serde(default)]
    pub scope: String,
}

impl ClientCredentialsTokenResponse {
    pub fn builder() -> ClientCredentialsTokenResponseBuilder {
        <ClientCredentialsTokenResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientCredentialsTokenResponseBuilder {
    access_token: Option<String>,
    token_type: Option<ClientCredentialsTokenResponseTokenType>,
    expires_in: Option<i64>,
    scope: Option<String>,
}

impl ClientCredentialsTokenResponseBuilder {
    pub fn access_token(mut self, value: impl Into<String>) -> Self {
        self.access_token = Some(value.into());
        self
    }

    pub fn token_type(mut self, value: ClientCredentialsTokenResponseTokenType) -> Self {
        self.token_type = Some(value);
        self
    }

    pub fn expires_in(mut self, value: i64) -> Self {
        self.expires_in = Some(value);
        self
    }

    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClientCredentialsTokenResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_token`](ClientCredentialsTokenResponseBuilder::access_token)
    /// - [`token_type`](ClientCredentialsTokenResponseBuilder::token_type)
    /// - [`expires_in`](ClientCredentialsTokenResponseBuilder::expires_in)
    /// - [`scope`](ClientCredentialsTokenResponseBuilder::scope)
    pub fn build(self) -> Result<ClientCredentialsTokenResponse, BuildError> {
        Ok(ClientCredentialsTokenResponse {
            access_token: self
                .access_token
                .ok_or_else(|| BuildError::missing_field("access_token"))?,
            token_type: self
                .token_type
                .ok_or_else(|| BuildError::missing_field("token_type"))?,
            expires_in: self
                .expires_in
                .ok_or_else(|| BuildError::missing_field("expires_in"))?,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
        })
    }
}
