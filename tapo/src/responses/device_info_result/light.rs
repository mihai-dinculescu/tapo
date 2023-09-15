use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DecodableResultExt, DefaultStateType, TapoResponseExt};

/// Device info of Tapo L510 and L610. Superset of [`crate::responses::DeviceInfoGenericResult`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct DeviceInfoLightResult {
    //
    // Inherited from DeviceInfoGenericResult
    //
    pub device_id: String,
    pub r#type: String,
    pub model: String,
    pub hw_id: String,
    pub hw_ver: String,
    pub fw_id: String,
    pub fw_ver: String,
    pub oem_id: String,
    pub mac: String,
    pub ip: String,
    pub ssid: String,
    pub signal_level: u8,
    pub rssi: i16,
    pub specs: String,
    pub lang: String,
    pub device_on: bool,
    /// The time in seconds this device has been ON since the last state change (ON/OFF).
    /// On v2 hardware this is always None.
    pub on_time: Option<u64>,
    pub overheated: bool,
    pub nickname: String,
    pub avatar: String,
    pub has_set_location_info: bool,
    pub region: Option<String>,
    pub latitude: Option<i64>,
    pub longitude: Option<i64>,
    pub time_diff: Option<i64>,
    //
    // Unique to this device
    //
    pub brightness: u8,
    pub dynamic_light_effect_enable: bool,
    pub dynamic_light_effect_id: Option<String>,
    pub hue: Option<u16>,
    pub saturation: Option<u16>,
    pub color_temp: u16,
    /// The default state of a device to be used when internet connectivity is lost after a power cut.
    pub default_states: DefaultLightState,
}

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
#[allow(missing_docs)]
pub struct DefaultLightState {
    pub r#type: DefaultStateType,
    pub state: LightState,
}

/// Light State.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct LightState {
    pub brightness: u8,
}
