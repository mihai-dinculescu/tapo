use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DecodableResultExt, DefaultState, TapoResponseExt};

/// Device info of Tapo P100, P105, P110 and P115. Superset of [`crate::responses::GenericDeviceInfoResult`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PlugDeviceInfoResult {
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
    /// The default state of a device to be used when internet connectivity is lost after a power cut.
    pub default_states: DefaultState<PlugStateWrapper>,
}

impl TapoResponseExt for PlugDeviceInfoResult {}

impl DecodableResultExt for PlugDeviceInfoResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.ssid = decode_value(&self.ssid)?;
        self.nickname = decode_value(&self.nickname)?;

        Ok(self)
    }
}

/// Plug State wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PlugStateWrapper {
    pub state: PlugState,
}

/// Plug State.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PlugState {
    pub on: Option<bool>,
}
