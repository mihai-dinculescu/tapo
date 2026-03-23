use serde::{Deserialize, Deserializer, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, TapoResponseExt, decode_value};

/// Basic device info of a Tapo device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DeviceInfoBasicResult {
    pub avatar: String,
    #[serde(alias = "dev_id")]
    pub device_id: String,
    #[serde(alias = "sw_version")]
    pub fw_ver: String,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub has_set_location_info: bool,
    #[serde(alias = "hw_version")]
    pub hw_ver: String,
    pub latitude: Option<i64>,
    pub longitude: Option<i64>,
    pub mac: String,
    #[serde(alias = "device_model")]
    pub model: String,
    #[serde(alias = "device_alias")]
    pub nickname: Option<String>,
    pub oem_id: String,
    pub region: Option<String>,
    #[serde(alias = "device_type")]
    pub r#type: String,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DeviceInfoBasicResult);

impl TapoResponseExt for DeviceInfoBasicResult {}

impl DecodableResultExt for DeviceInfoBasicResult {
    fn decode(mut self) -> Result<Self, Error> {
        if let Some(nickname) = &self.nickname {
            self.nickname = Some(decode_value(nickname)?);
        }

        Ok(self)
    }
}

/// Deserialize a boolean from either a JSON bool or an integer (0/1).
/// Camera devices return some boolean fields as integers.
fn bool_from_int_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Bool(b) => Ok(b),
        serde_json::Value::Number(n) => Ok(n.as_i64().unwrap_or(0) != 0),
        _ => Err(serde::de::Error::custom("expected bool or integer")),
    }
}
