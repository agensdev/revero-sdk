use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct OauthClient {
    pub http_client: HttpClient,
}

impl OauthClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// # Examples
    ///
    /// ```no_run
    /// use revero::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         username: Some("<username>".to_string()),
    ///         password: Some("<password>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    ///     client
    ///         .oauth
    ///         .get_oauth_authorization_server_metadata(None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_oauth_authorization_server_metadata(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ClientCredentialsMetadataResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                ".well-known/oauth-authorization-server/oauth/partner",
                None,
                None,
                options,
            )
            .await
    }

    /// Authenticate with HTTP Basic using the integration client ID and secret. The form must request the exact advertised resource and an explicit space-separated scope set. Client secrets must only be used by a backend.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use revero::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         username: Some("<username>".to_string()),
    ///         password: Some("<password>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    ///     client
    ///         .oauth
    ///         .exchange_client_credentials(
    ///             &ExchangeClientCredentialsRequest {
    ///                 grant_type: ExchangeClientCredentialsRequestGrantType::ClientCredentials,
    ///                 resource: "https://referat-api.flyt.test/public/v1".to_string(),
    ///                 scope: "meetings:read meetings:write recordings:write".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn exchange_client_credentials(
        &self,
        request: &ExchangeClientCredentialsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ClientCredentialsTokenResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "oauth/partner/token",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
