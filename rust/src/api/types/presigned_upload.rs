pub use crate::prelude::*;

/// A single-object upload grant. The only thing here safe to give a browser.
///
/// It carries no Revero credential and authorises writing one object, once,
/// until it expires. Never send your client secret or bearer token to browser
/// code.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PresignedUpload {
    /// Use this HTTP method exactly as returned. Do not hard-code or reinterpret it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Upload the audio bytes here, directly to object storage.
    #[serde(default)]
    pub url: String,
    /// Send these verbatim on the upload. `Content-Type` must match the type you declared, or completion refuses the object.
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// After this the grant is dead — prepare a new recording rather than retrying the upload.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
}

impl PresignedUpload {
    pub fn builder() -> PresignedUploadBuilder {
        <PresignedUploadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PresignedUploadBuilder {
    method: Option<String>,
    url: Option<String>,
    headers: Option<HashMap<String, String>>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl PresignedUploadBuilder {
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn headers(mut self, value: HashMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PresignedUpload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](PresignedUploadBuilder::url)
    /// - [`headers`](PresignedUploadBuilder::headers)
    /// - [`expires_at`](PresignedUploadBuilder::expires_at)
    pub fn build(self) -> Result<PresignedUpload, BuildError> {
        Ok(PresignedUpload {
            method: self.method,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            headers: self
                .headers
                .ok_or_else(|| BuildError::missing_field("headers"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
