use base64::{engine::general_purpose, Engine as _};

use crate::error::Error;

/// Implemented by all Device Info Result variations.
pub trait DeviceInfoResultExt
where
    Self: Sized,
{
    fn decode(&self) -> Result<Self, Error>;
}

pub(crate) fn decode_value(value: &str) -> anyhow::Result<String> {
    let decoded_bytes = general_purpose::STANDARD.decode(value)?;
    Ok(std::str::from_utf8(&decoded_bytes)?.to_string())
}
