pub use crate::prelude::*;

/// Authorization scheme used when calling the Public API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ClientCredentialsTokenResponseTokenType {
    Bearer,
}
impl fmt::Display for ClientCredentialsTokenResponseTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Bearer => "Bearer",
        };
        write!(f, "{}", s)
    }
}
