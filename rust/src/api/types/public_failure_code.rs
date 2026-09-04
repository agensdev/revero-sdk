pub use crate::prelude::*;

/// Stable, machine-readable reasons a public operation ended badly.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PublicFailureCode {
    UploadUnreadable,
    TranscriptionFailed,
    TranscriptionEmpty,
    GenerationFailed,
    ProcessingNotEntitled,
    OutputProfileUnavailable,
    OrganisationKeyUnavailable,
    JurisdictionUnavailable,
    InternalError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PublicFailureCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UploadUnreadable => serializer.serialize_str("upload_unreadable"),
            Self::TranscriptionFailed => serializer.serialize_str("transcription_failed"),
            Self::TranscriptionEmpty => serializer.serialize_str("transcription_empty"),
            Self::GenerationFailed => serializer.serialize_str("generation_failed"),
            Self::ProcessingNotEntitled => serializer.serialize_str("processing_not_entitled"),
            Self::OutputProfileUnavailable => {
                serializer.serialize_str("output_profile_unavailable")
            }
            Self::OrganisationKeyUnavailable => {
                serializer.serialize_str("organisation_key_unavailable")
            }
            Self::JurisdictionUnavailable => serializer.serialize_str("jurisdiction_unavailable"),
            Self::InternalError => serializer.serialize_str("internal_error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PublicFailureCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "upload_unreadable" => Ok(Self::UploadUnreadable),
            "transcription_failed" => Ok(Self::TranscriptionFailed),
            "transcription_empty" => Ok(Self::TranscriptionEmpty),
            "generation_failed" => Ok(Self::GenerationFailed),
            "processing_not_entitled" => Ok(Self::ProcessingNotEntitled),
            "output_profile_unavailable" => Ok(Self::OutputProfileUnavailable),
            "organisation_key_unavailable" => Ok(Self::OrganisationKeyUnavailable),
            "jurisdiction_unavailable" => Ok(Self::JurisdictionUnavailable),
            "internal_error" => Ok(Self::InternalError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PublicFailureCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UploadUnreadable => write!(f, "upload_unreadable"),
            Self::TranscriptionFailed => write!(f, "transcription_failed"),
            Self::TranscriptionEmpty => write!(f, "transcription_empty"),
            Self::GenerationFailed => write!(f, "generation_failed"),
            Self::ProcessingNotEntitled => write!(f, "processing_not_entitled"),
            Self::OutputProfileUnavailable => write!(f, "output_profile_unavailable"),
            Self::OrganisationKeyUnavailable => write!(f, "organisation_key_unavailable"),
            Self::JurisdictionUnavailable => write!(f, "jurisdiction_unavailable"),
            Self::InternalError => write!(f, "internal_error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
