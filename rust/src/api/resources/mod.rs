//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Oauth**
//! - **PublicApi**

use crate::{ApiError, ClientConfig};

pub mod oauth;
pub mod public_api;
pub struct GeneratedReveroClient {
    pub config: ClientConfig,
    pub oauth: OauthClient,
    pub public_api: PublicApiClient,
}

impl GeneratedReveroClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            oauth: OauthClient::new(config.clone())?,
            public_api: PublicApiClient::new(config.clone())?,
        })
    }
}

pub use oauth::OauthClient;
pub use public_api::PublicApiClient;
