use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{
    DecodableResultExt, DefaultBrightnessState, DefaultPowerType, TapoResponseExt, decode_value,
};

/// Device info of Tapo L510, L520 and L610.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DeviceInfoLightResult {
    //
    // Common properties
    //
    pub avatar: String,
    pub device_id: String,
    pub device_on: bool,
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
    /// On v2 hardware this is always None.
    pub on_time: Option<u64>,
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
    pub brightness: u8,
    /// The default state of a device to be used when internet connectivity is lost after a power cut.
    pub default_states: DefaultLightState,
    pub overheated: bool,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DeviceInfoLightResult);

impl TapoResponseExt for DeviceInfoLightResult {}

impl DecodableResultExt for DeviceInfoLightResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.ssid = decode_value(&self.ssid)?;
        self.nickname = decode_value(&self.nickname)?;

        Ok(self)
    }
}

/// Light Default State.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DefaultLightState {
    pub brightness: DefaultBrightnessState,
    pub re_power_type: Option<DefaultPowerType>,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DefaultLightState);
