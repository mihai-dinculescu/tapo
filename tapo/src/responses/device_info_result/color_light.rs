use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, DefaultStateType, TapoResponseExt, decode_value};

/// Device info of Tapo L530, L535 and L630.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DeviceInfoColorLightResult {
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
    pub color_temp: u16,
    /// The default state of a device to be used when internet connectivity is lost after a power cut.
    pub default_states: DefaultColorLightState,
    pub device_on: bool,
    /// Some L530/L535/L630 firmware builds omit this field; default to `false`.
    #[serde(default)]
    pub dynamic_light_effect_enable: bool,
    pub dynamic_light_effect_id: Option<String>,
    pub hue: Option<u16>,
    pub nickname: String,
    /// The time in seconds this device has been ON since the last state change (On/Off).
    /// On v2 hardware this is always None.
    pub on_time: Option<u64>,
    pub overheated: bool,
    pub saturation: Option<u16>,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DeviceInfoColorLightResult);

impl TapoResponseExt for DeviceInfoColorLightResult {}

impl DecodableResultExt for DeviceInfoColorLightResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.ssid = decode_value(&self.ssid)?;
        self.nickname = decode_value(&self.nickname)?;

        Ok(self)
    }
}

/// Color Light Default State.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DefaultColorLightState {
    pub r#type: DefaultStateType,
    pub state: ColorLightState,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DefaultColorLightState);

/// Color Light State.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct ColorLightState {
    pub brightness: u8,
    pub hue: Option<u16>,
    pub saturation: Option<u16>,
    pub color_temp: u16,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(ColorLightState);
#[cfg(test)]
mod tests {
    use super::*;

    fn color_light_info_json(with_dynamic_effect: bool) -> String {
        let dynamic = if with_dynamic_effect {
            "\"dynamic_light_effect_enable\": true, \"dynamic_light_effect_id\": \"L1\","
        } else {
            // some firmware builds omit both fields
            ""
        };
        format!(
            r#"{{{dynamic}
            "avatar": "bulb",
            "device_id": "8000000000000000000000000000000000000535",
            "device_on": true,
            "fw_id": "00000000000000000000000000000000",
            "fw_ver": "1.4.5 Build 260923 Rel.174333",
            "has_set_location_info": true,
            "hw_id": "ABCDEF0123456789ABCDEF0123456789",
            "hw_ver": "3.0",
            "ip": "192.168.0.157",
            "lang": "en_US",
            "latitude": 373702,
            "longitude": -1219667,
            "mac": "AA-BB-CC-DD-EE-57",
            "model": "L535",
            "oem_id": "9876543210FEDCBA9876543210FEDCBA",
            "brightness": 55,
            "color_temp": 0,
            "default_states": {{"type": "last_states", "state": {{"brightness": 55, "color_temp": 0, "hue": 29, "saturation": 69}}}},
            "hue": 29,
            "nickname": "T2ZmaWNlIE9wZW5DbGF3IExpZ2h0",
            "overheated": false,
            "saturation": 69,
            "region": "America/Los_Angeles",
            "rssi": -39,
            "signal_level": 3,
            "specs": "",
            "ssid": "T3BlbkNsYXc=",
            "time_diff": -480,
            "type": "SMART.TAPOBULB"
        }}"#
        )
    }

    #[test]
    fn device_info_parses_when_dynamic_light_effect_enable_is_absent() {
        // L535 1.4.5 and some L530/L630 firmware builds omit the field.
        let info: DeviceInfoColorLightResult =
            serde_json::from_str(&color_light_info_json(false)).expect("should parse");
        assert!(!info.dynamic_light_effect_enable);
        assert_eq!(info.model, "L535");
    }

    #[test]
    fn device_info_parses_when_dynamic_light_effect_enable_is_present() {
        let info: DeviceInfoColorLightResult =
            serde_json::from_str(&color_light_info_json(true)).expect("should parse");
        assert!(info.dynamic_light_effect_enable);
        assert_eq!(info.dynamic_light_effect_id.as_deref(), Some("L1"));
    }
}
