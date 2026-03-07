use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, TapoResponseExt, decode_value};

/// Device info of Tapo P300, P304M, P306 and P316M. Superset of [`crate::responses::DeviceInfoGenericResult`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DeviceInfoPowerStripResult {
    //
    // Inherited from DeviceInfoGenericResult
    //
    pub avatar: String,
    pub device_id: String,
    pub fw_id: String,
    pub fw_ver: String,
    pub has_set_location_info: bool,
    pub hw_id: String,
    pub hw_ver: String,
    pub ip: String,
    pub lang: String,
    pub latitude: Option<i64>,
    pub longitude: Option<i64>,
    pub mac: String,
    pub model: String,
    pub oem_id: String,
    pub region: Option<String>,
    pub rssi: i16,
    pub signal_level: u8,
    pub specs: String,
    pub ssid: String,
    pub time_diff: i64,
    pub r#type: String,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DeviceInfoPowerStripResult);

impl TapoResponseExt for DeviceInfoPowerStripResult {}

impl DecodableResultExt for DeviceInfoPowerStripResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.ssid = decode_value(&self.ssid)?;

        Ok(self)
    }
}
