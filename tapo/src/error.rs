/// Response Error from the Tapo API.
#[derive(Debug)]
#[non_exhaustive]
pub enum TapoResponseError {
    /// The Tapo API was expected to return a non-empty result.
    EmptyResult,
    /// The credentials provided were invalid.
    InvalidCredentials,
    /// Unknown Error. This is a catch-all for errors that don't fit into the other categories.
    /// In time, some of these might be added as their own variants.
    Unknown(i32),
}

/// Tapo API Client Error.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Response Error from the Tapo API.
    #[error("Tapo: {0:?}")]
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
    #[error("Serde: {0}")]
    Serde(#[from] serde_json::Error),
    /// HTTP Error.
    #[error("Http: {0}")]
    Http(#[from] isahc::Error),
    /// Other Error. This is a catch-all for errors that don't fit into the other categories.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
