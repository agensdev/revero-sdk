pub use crate::prelude::*;

/// Recording ingest lifecycle; all states after transcribed are terminal.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PublicRecordingStatus {
    AwaitingUpload,
    Uploaded,
    Processing,
    Transcribed,
    Failed,
    Cancelled,
    Purged,
    Unrecoverable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PublicRecordingStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AwaitingUpload => serializer.serialize_str("awaiting_upload"),
            Self::Uploaded => serializer.serialize_str("uploaded"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Transcribed => serializer.serialize_str("transcribed"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::Purged => serializer.serialize_str("purged"),
            Self::Unrecoverable => serializer.serialize_str("unrecoverable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PublicRecordingStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "awaiting_upload" => Ok(Self::AwaitingUpload),
            "uploaded" => Ok(Self::Uploaded),
            "processing" => Ok(Self::Processing),
            "transcribed" => Ok(Self::Transcribed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            "purged" => Ok(Self::Purged),
            "unrecoverable" => Ok(Self::Unrecoverable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PublicRecordingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AwaitingUpload => write!(f, "awaiting_upload"),
            Self::Uploaded => write!(f, "uploaded"),
            Self::Processing => write!(f, "processing"),
            Self::Transcribed => write!(f, "transcribed"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Purged => write!(f, "purged"),
            Self::Unrecoverable => write!(f, "unrecoverable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
