pub use crate::prelude::*;

/// Every event that may leave an organisation over this surface.
///
/// The product contract ADR-0072 asked for, stated as a closed set. Terminal
/// only: there is no "started", no "progress" and no percentage, because a
/// partner reconciles from the authoritative GET and a notification only
/// accelerates that (ADR-0072, "Status and delivery").
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PublicWebhookEventType {
    OperationCompleted,
    OperationFailed,
    OutputCompleted,
    OutputFailed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PublicWebhookEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OperationCompleted => serializer.serialize_str("operation.completed"),
            Self::OperationFailed => serializer.serialize_str("operation.failed"),
            Self::OutputCompleted => serializer.serialize_str("output.completed"),
            Self::OutputFailed => serializer.serialize_str("output.failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PublicWebhookEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "operation.completed" => Ok(Self::OperationCompleted),
            "operation.failed" => Ok(Self::OperationFailed),
            "output.completed" => Ok(Self::OutputCompleted),
            "output.failed" => Ok(Self::OutputFailed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PublicWebhookEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OperationCompleted => write!(f, "operation.completed"),
            Self::OperationFailed => write!(f, "operation.failed"),
            Self::OutputCompleted => write!(f, "output.completed"),
            Self::OutputFailed => write!(f, "output.failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
