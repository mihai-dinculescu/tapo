use base64::{Engine as _, engine::general_purpose};

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

/// Encodes a string as base64, the way the device takes text such as names.
pub(crate) fn encode_value(value: &str) -> String {
    general_purpose::STANDARD.encode(value)
}

pub(crate) fn decode_value(value: &str) -> anyhow::Result<String> {
    let decoded_bytes = general_purpose::STANDARD.decode(value)?;
    Ok(std::str::from_utf8(&decoded_bytes)?.to_string())
}
