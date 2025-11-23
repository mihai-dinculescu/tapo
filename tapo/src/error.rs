/// Response Error from the Tapo API.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum TapoResponseError {
    /// Unexpected empty result.
    #[error("Unexpected empty result")]
    EmptyResult,
    /// Forbidden access.
    #[error("Forbidden: code {code}, description {description}")]
    Forbidden {
        /// Error code.
        code: String,
        /// Error description.
        description: String,
    },
    /// Parameters were invalid.
    #[error("Invalid parameters")]
    InvalidParameters,
    /// Invalid public key.
    #[error("Invalid public key")]
    InvalidPublicKey,
    /// Invalid request.
    #[error("Invalid request")]
    InvalidRequest,
    /// Invalid response.
    #[error("Invalid response")]
    InvalidResponse,
    /// Malformed request.
    #[error("Malformed request")]
    MalformedRequest,
    /// Session timeout.
    #[error("Session timeout")]
    SessionTimeout,
    /// Unauthorized access.
    #[error("Unauthorized: code {code}, description {description}")]
    Unauthorized {
        /// Error code.
        code: String,
        /// Error description.
        description: String,
    },
    /// Unknown Error. This is a catch-all for errors that don't fit into the other categories.
    /// In time, some of these might be added as their own variants.
    #[error("Unknown error: {0}")]
    Unknown(i32),
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
