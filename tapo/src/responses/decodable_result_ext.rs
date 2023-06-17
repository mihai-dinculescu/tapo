use base64::{engine::general_purpose, Engine as _};

use crate::error::Error;

/// Implemented by all Device Info Result variations.
pub(crate) trait DecodableResultExt
where
    Self: Sized,
{
    /// Decodes a base64 encoded string from the result.
    fn decode(self) -> Result<Self, Error>;
}

impl DecodableResultExt for serde_json::Value {
    fn decode(self) -> Result<Self, Error> {
        Ok(self)
    }
}

pub(crate) fn decode_value(value: &str) -> anyhow::Result<String> {
    let decoded_bytes = general_purpose::STANDARD.decode(value)?;
    Ok(std::str::from_utf8(&decoded_bytes)?.to_string())
}
