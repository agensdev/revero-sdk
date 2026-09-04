pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ClientCredentialsMetadataResponseTokenEndpointAuthMethodsSupportedItem {
    #[serde(rename = "client_secret_basic")]
    ClientSecretBasic,
}
impl fmt::Display for ClientCredentialsMetadataResponseTokenEndpointAuthMethodsSupportedItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ClientSecretBasic => "client_secret_basic",
        };
        write!(f, "{}", s)
    }
}
