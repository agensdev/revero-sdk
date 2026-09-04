pub use crate::prelude::*;

/// OAuth authorization-server metadata for this service issuer.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ClientCredentialsMetadataResponse {
    /// Issuer that creates and verifies service tokens.
    #[serde(default)]
    pub issuer: String,
    /// Endpoint for Client Credentials exchange.
    #[serde(default)]
    pub token_endpoint: String,
    /// OAuth grants supported by this issuer.
    #[serde(default)]
    pub grant_types_supported: Vec<ClientCredentialsMetadataResponseGrantTypesSupportedItem>,
    /// Client authentication methods accepted by the token endpoint.
    #[serde(default)]
    pub token_endpoint_auth_methods_supported:
        Vec<ClientCredentialsMetadataResponseTokenEndpointAuthMethodsSupportedItem>,
    /// OAuth scopes this issuer can grant.
    #[serde(default)]
    pub scopes_supported: Vec<String>,
}

impl ClientCredentialsMetadataResponse {
    pub fn builder() -> ClientCredentialsMetadataResponseBuilder {
        <ClientCredentialsMetadataResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientCredentialsMetadataResponseBuilder {
    issuer: Option<String>,
    token_endpoint: Option<String>,
    grant_types_supported: Option<Vec<ClientCredentialsMetadataResponseGrantTypesSupportedItem>>,
    token_endpoint_auth_methods_supported:
        Option<Vec<ClientCredentialsMetadataResponseTokenEndpointAuthMethodsSupportedItem>>,
    scopes_supported: Option<Vec<String>>,
}

impl ClientCredentialsMetadataResponseBuilder {
    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn token_endpoint(mut self, value: impl Into<String>) -> Self {
        self.token_endpoint = Some(value.into());
        self
    }

    pub fn grant_types_supported(
        mut self,
        value: Vec<ClientCredentialsMetadataResponseGrantTypesSupportedItem>,
    ) -> Self {
        self.grant_types_supported = Some(value);
        self
    }

    pub fn token_endpoint_auth_methods_supported(
        mut self,
        value: Vec<ClientCredentialsMetadataResponseTokenEndpointAuthMethodsSupportedItem>,
    ) -> Self {
        self.token_endpoint_auth_methods_supported = Some(value);
        self
    }

    pub fn scopes_supported(mut self, value: Vec<String>) -> Self {
        self.scopes_supported = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientCredentialsMetadataResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`issuer`](ClientCredentialsMetadataResponseBuilder::issuer)
    /// - [`token_endpoint`](ClientCredentialsMetadataResponseBuilder::token_endpoint)
    /// - [`grant_types_supported`](ClientCredentialsMetadataResponseBuilder::grant_types_supported)
    /// - [`token_endpoint_auth_methods_supported`](ClientCredentialsMetadataResponseBuilder::token_endpoint_auth_methods_supported)
    /// - [`scopes_supported`](ClientCredentialsMetadataResponseBuilder::scopes_supported)
    pub fn build(self) -> Result<ClientCredentialsMetadataResponse, BuildError> {
        Ok(ClientCredentialsMetadataResponse {
            issuer: self
                .issuer
                .ok_or_else(|| BuildError::missing_field("issuer"))?,
            token_endpoint: self
                .token_endpoint
                .ok_or_else(|| BuildError::missing_field("token_endpoint"))?,
            grant_types_supported: self
                .grant_types_supported
                .ok_or_else(|| BuildError::missing_field("grant_types_supported"))?,
            token_endpoint_auth_methods_supported: self
                .token_endpoint_auth_methods_supported
                .ok_or_else(|| {
                    BuildError::missing_field("token_endpoint_auth_methods_supported")
                })?,
            scopes_supported: self
                .scopes_supported
                .ok_or_else(|| BuildError::missing_field("scopes_supported"))?,
        })
    }
}
