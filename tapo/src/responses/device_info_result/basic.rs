use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, TapoResponseExt, decode_value};

/// Basic device info of a Tapo device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DeviceInfoBasicResult {
    pub avatar: String,
    pub device_id: String,
    pub device_on: Option<bool>,
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
    pub nickname: String,
    pub oem_id: String,
    /// The time in seconds this device has been ON since the last state change (On/Off).
    pub on_time: Option<u64>,
    pub region: Option<String>,
    pub rssi: i16,
    pub signal_level: u8,
    pub specs: String,
    pub ssid: String,
    pub time_diff: Option<i64>,
    pub r#type: String,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DeviceInfoBasicResult);

impl TapoResponseExt for DeviceInfoBasicResult {}

impl DecodableResultExt for DeviceInfoBasicResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.ssid = decode_value(&self.ssid)?;
        self.nickname = decode_value(&self.nickname)?;

        Ok(self)
    }
}
