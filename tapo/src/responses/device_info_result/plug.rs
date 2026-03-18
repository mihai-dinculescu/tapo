use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, DefaultPlugState, TapoResponseExt, decode_value};

/// Device info of Tapo P100 and P105.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DeviceInfoPlugResult {
    //
    // Common properties
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
    pub time_diff: Option<i64>,
    pub r#type: String,
    //
    // Unique to this device
    //
    pub default_states: DefaultPlugState,
    pub device_on: bool,
    pub nickname: String,
    /// The time in seconds this device has been ON since the last state change (On/Off).
    pub on_time: u64,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DeviceInfoPlugResult);

impl TapoResponseExt for DeviceInfoPlugResult {}

impl DecodableResultExt for DeviceInfoPlugResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        self.ssid = decode_value(&self.ssid)?;

        Ok(self)
    }
}
