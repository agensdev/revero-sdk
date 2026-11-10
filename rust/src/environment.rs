use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "production")]
    Production,
    #[serde(rename = "staging")]
    Staging,
}
impl Environment {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Production => "https://api.revero.no",
            Self::Staging => "https://api.staging.revero.no",
        }
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::Production
    }
}
