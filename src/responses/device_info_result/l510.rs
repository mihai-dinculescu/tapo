use serde::Deserialize;

use crate::responses::{DefaultState, DeviceInfoResultExt, TapoResponseExt};

/// Device info of [`crate::L510`]. Superset of [`crate::GenericDeviceInfoResult`].
#[derive(Debug, Clone, Deserialize)]
pub struct L510DeviceInfoResult {
    //
    // Inherited from GenericDeviceInfoResult
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
    pub on_time: u64,
    pub overheated: bool,
    pub nickname: String,
    pub avatar: String,
    pub has_set_location_info: bool,
    pub region: Option<String>,
    pub longitude: Option<i64>,
    pub latitude: Option<i64>,
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
    pub default_states: DefaultState<L510StateWrapper>,
}
impl TapoResponseExt for L510DeviceInfoResult {}

/// L510 State wrapper.
#[derive(Debug, Clone, Deserialize)]
pub struct L510StateWrapper {
    pub state: L510State,
}

/// L510 State.
#[derive(Debug, Clone, Deserialize)]
pub struct L510State {
    pub brightness: u8,
}

impl DeviceInfoResultExt for L510DeviceInfoResult {
    fn decode(&self) -> anyhow::Result<Self> {
        Ok(Self {
            ssid: std::str::from_utf8(&base64::decode(self.ssid.clone())?)?.to_string(),
            nickname: std::str::from_utf8(&base64::decode(self.nickname.clone())?)?.to_string(),
            ..self.clone()
        })
    }
}
