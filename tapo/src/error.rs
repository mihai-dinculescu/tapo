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
    Http(#[from] reqwest::Error),
    /// Device not found
    #[error("Device not found")]
    DeviceNotFound,
    /// Other Error. This is a catch-all for errors that don't fit into the other categories.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
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
