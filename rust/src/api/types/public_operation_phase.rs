pub use crate::prelude::*;

/// Informational work stage. Never use it to decide whether work is terminal.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PublicOperationPhase {
    Upload,
    Transcription,
    Generation,
    Delivery,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PublicOperationPhase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Upload => serializer.serialize_str("upload"),
            Self::Transcription => serializer.serialize_str("transcription"),
            Self::Generation => serializer.serialize_str("generation"),
            Self::Delivery => serializer.serialize_str("delivery"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PublicOperationPhase {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "upload" => Ok(Self::Upload),
            "transcription" => Ok(Self::Transcription),
            "generation" => Ok(Self::Generation),
            "delivery" => Ok(Self::Delivery),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PublicOperationPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Upload => write!(f, "upload"),
            Self::Transcription => write!(f, "transcription"),
            Self::Generation => write!(f, "generation"),
            Self::Delivery => write!(f, "delivery"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
