use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, TapoResponseExt, decode_value};
use crate::utils::bool_from_int_or_bool;

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
