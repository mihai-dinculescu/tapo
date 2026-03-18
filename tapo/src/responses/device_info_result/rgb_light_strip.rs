use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, DefaultStateType, TapoResponseExt, decode_value};

/// Device info of Tapo L900.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DeviceInfoRgbLightStripResult {
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
    pub brightness: u8,
    pub color_temp_range: [u16; 2],
    pub color_temp: u16,
    /// The default state of a device to be used when internet connectivity is lost after a power cut.
    pub default_states: DefaultRgbLightStripState,
    pub device_on: bool,
    pub hue: Option<u16>,
    pub nickname: String,
    pub overheated: bool,
    pub saturation: Option<u16>,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DeviceInfoRgbLightStripResult);

impl TapoResponseExt for DeviceInfoRgbLightStripResult {}

impl DecodableResultExt for DeviceInfoRgbLightStripResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.ssid = decode_value(&self.ssid)?;
        self.nickname = decode_value(&self.nickname)?;

        Ok(self)
    }
}

/// RGB Light Strip Default State.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DefaultRgbLightStripState {
    pub r#type: DefaultStateType,
    pub state: RgbLightStripState,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DefaultRgbLightStripState);

/// RGB Light Strip State.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct RgbLightStripState {
    pub brightness: Option<u8>,
    pub hue: Option<u16>,
    pub saturation: Option<u16>,
    pub color_temp: Option<u16>,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(RgbLightStripState);
