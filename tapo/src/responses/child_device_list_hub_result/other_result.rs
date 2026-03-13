use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, decode_value};

/// Catch-all result for unsupported hub child devices.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct OtherResult {
    pub device_id: String,
    pub model: String,
    pub nickname: String,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(OtherResult);

impl DecodableResultExt for OtherResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}
