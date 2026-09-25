/// Response Error from the Tapo API.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum TapoResponseError {
    #[error("Device error {code}: {kind}")]
    DeviceError { code: i64, kind: &'static str },
    #[error("Unexpected empty result")]
    EmptyResult,
    #[error("HTTP error {status_code}: {description}")]
    HttpError {
        status_code: u16,
        description: String,
    },
    #[error("Response error: {description}")]
    ResponseError { description: String },
    #[error("Unauthorized: {kind}: {description}")]
    Unauthorized {
        kind: &'static str,
        description: String,
    },
}

impl TapoResponseError {
    pub(crate) fn session_expired(kind: &'static str) -> Self {
        Self::Unauthorized {
            kind,
            description: "Session has expired. Re-authentication is required.".to_string(),
        }
    }

    pub(crate) fn hash_mismatch() -> Self {
        Self::Unauthorized {
            kind: "HASH_MISMATCH",
            description: "The device response did not match the challenge issued by the library. Make sure that your email and password are correct -— both are case-sensitive. Before adding a new device, disconnect any existing TP-Link/Tapo devices on the network. The TP-Link Simple Setup (TSS) protocol, which shares credentials from previously configured devices, may interfere with authentication. If the problem continues, perform a factory reset on the new device and add it again with no other TP-Link devices active during setup.".to_string(),
        }
    }
}

/// Tapo API Client Error.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Response Error from the Tapo API.
    #[error(transparent)]
    Tapo(TapoResponseError),
    /// Validation Error of a provided field.
    #[error("Validation: {field} {message}")]
    Validation {
        /// The field that failed validation.
        field: String,
        /// The validation error message.
        message: String,
    },
    /// Serialization/Deserialization Error.
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    /// HTTP Error.
    #[error(transparent)]
    Http(reqwest::Error),
    /// Device not found
    #[error("Device not found")]
    DeviceNotFound,
    /// Other Error. This is a catch-all for errors that don't fit into the other categories.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<reqwest::Error> for Error {
    /// Redacts the session token from the error's URL, because reqwest prints the full URL.
    fn from(mut err: reqwest::Error) -> Self {
        if let Some(url) = err.url_mut() {
            redact_session_token(url);
        }
        Self::Http(err)
    }
}

/// Redacts the session token that the AES SSL protocol puts in the `stok=` path segment
/// and the AES protocol puts in the `token` query parameter.
fn redact_session_token(url: &mut reqwest::Url) {
    let path = url
        .path()
        .split('/')
        .map(|segment| {
            if segment.starts_with("stok=") {
                "stok=REDACTED"
            } else {
                segment
            }
        })
        .collect::<Vec<_>>()
        .join("/");
    url.set_path(&path);

    if url.query().is_some() {
        let pairs = url
            .query_pairs()
            .map(|(key, value)| {
                let value = if key == "token" {
                    "REDACTED".to_string()
                } else {
                    value.into_owned()
                };
                (key.into_owned(), value)
            })
            .collect::<Vec<_>>();
        url.query_pairs_mut().clear().extend_pairs(pairs);
    }
}

#[cfg(feature = "python")]
impl From<Error> for pyo3::PyErr {
    fn from(err: Error) -> pyo3::PyErr {
        pyo3::exceptions::PyException::new_err(format!("{:?}", err))
    }
}

/// Discovery Error. Wraps an error that occurred while discovering a specific device.
#[derive(thiserror::Error, Debug)]
#[error("Failed to discover device at {ip}: {source}")]
pub struct DiscoveryError {
    /// The IP address of the device that failed to be discovered.
    pub ip: String,
    /// The underlying error.
    pub source: Error,
}

#[cfg(feature = "python")]
impl From<DiscoveryError> for pyo3::PyErr {
    fn from(err: DiscoveryError) -> pyo3::PyErr {
        pyo3::exceptions::PyException::new_err(format!("{:?}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn http_error_redacts_the_session_token() {
        let err: Error = reqwest::Client::new()
            .post("https://127.0.0.1:0/stok=secret-token/ds")
            .send()
            .await
            .unwrap_err()
            .into();

        let display = err.to_string();
        assert!(
            display.contains("https://127.0.0.1:0/stok=REDACTED/ds"),
            "{display}"
        );
        assert!(!format!("{err:?}").contains("secret-token"));
    }

    #[test]
    fn redact_session_token_in_query() {
        let mut url = reqwest::Url::parse("http://127.0.0.1/app?token=secret-token").unwrap();
        redact_session_token(&mut url);
        assert_eq!(url.as_str(), "http://127.0.0.1/app?token=REDACTED");
    }

    #[test]
    fn redact_session_token_keeps_other_urls() {
        let mut url = reqwest::Url::parse("http://127.0.0.1/app/request?seq=123").unwrap();
        redact_session_token(&mut url);
        assert_eq!(url.as_str(), "http://127.0.0.1/app/request?seq=123");
    }
}
