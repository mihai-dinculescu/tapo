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

    pub(crate) fn forbidden() -> Self {
        Self::Unauthorized {
            kind: "FORBIDDEN",
            description: "Make sure Third-Party Compatibility is turned on in the Tapo app. If it's already enabled, try switching it off and then back on again. You can find this option by navigating to Me > Third-Party Services in the app.".to_string(),
        }
    }

    pub(crate) fn hash_mismatch() -> Self {
        Self::Unauthorized {
            kind: "HASH_MISMATCH",
            description: "The device response did not match the challenge issued by the library. Make sure that your email and password are correct - both are case-sensitive. Before adding a new device, disconnect any existing TP-Link/Tapo devices on the network. The TP-Link Simple Setup (TSS) protocol, which shares credentials from previously configured devices, may interfere with authentication. If the problem continues, perform a factory reset on the new device and add it again with no other TP-Link devices active during setup.".to_string(),
        }
    }

    pub(crate) fn tpap_hash_mismatch() -> Self {
        Self::Unauthorized {
            kind: "TPAP_HASH_MISMATCH",
            description: "The device accepted the login but its response did not match the challenge issued by the library, so the device could not prove that it knows the password as well.".to_string(),
        }
    }

    pub(crate) fn tpap_credentials() -> Self {
        Self::Unauthorized {
            kind: "TPAP_CREDENTIALS",
            description: "Please verify that your password is correct - it is case-sensitive. The device locks itself after too many failed logins, so do not retry in a loop.".to_string(),
        }
    }

    pub(crate) fn tpap_auth_attempts_limit() -> Self {
        Self::Unauthorized {
            kind: "TPAP_AUTH_ATTEMPTS_LIMIT",
            description: "The device has locked itself after too many failed logins and refuses even a correct password. Verify that your email and password are those of the TP-Link account the device is registered to, and wait a while before trying again. Retrying keeps the device locked.".to_string(),
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
    /// The device speaks a protocol that the library does not support.
    #[error("Unsupported protocol: {protocol}: {description}")]
    UnsupportedProtocol {
        /// The protocol the device speaks.
        protocol: &'static str,
        /// Why the protocol is not supported and what to do about it.
        description: String,
    },
    /// Other Error. This is a catch-all for errors that don't fit into the other categories.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub(crate) fn unsupported_aes_protocol() -> Self {
        Self::UnsupportedProtocol {
            protocol: "AES",
            description: "The device uses the legacy AES protocol, which is no longer supported. Updating the firmware in the Tapo app switches the device to KLAP, which is supported.".to_string(),
        }
    }

    /// A device in TPAP mode that wants to be logged in to in a way the
    /// library cannot do yet. `detail` is what the device announced.
    pub(crate) fn unsupported_tpap(detail: String) -> Self {
        Self::UnsupportedProtocol {
            protocol: "TPAP",
            description: format!(
                "The device uses a variant of the TPAP protocol that is not supported yet ({detail}). On some devices, turning on Third-Party Compatibility in the Tapo app (Me > Third-Party Services) switches the device to a protocol that is supported. If that does not help, please open an issue with this message and the device model."
            ),
        }
    }
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

/// Redacts the session token that the AES SSL and TPAP protocols put in the `stok=` path segment.
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
    fn redact_session_token_keeps_other_urls() {
        let mut url = reqwest::Url::parse("http://127.0.0.1/app/request?seq=123").unwrap();
        redact_session_token(&mut url);
        assert_eq!(url.as_str(), "http://127.0.0.1/app/request?seq=123");
    }
}
