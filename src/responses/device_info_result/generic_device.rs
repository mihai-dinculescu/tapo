use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DeviceInfoResultExt, TapoResponseExt};

/// Device info of [`crate::ApiClient<GenericDevice>`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericDeviceInfoResult {
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
    pub overheated: bool,
    pub nickname: String,
    pub avatar: String,
    pub has_set_location_info: bool,
    /// The time in seconds this device has been ON since the last state change (ON/OFF)
    pub on_time: Option<u64>,
    pub region: Option<String>,
    pub longitude: Option<i64>,
    pub latitude: Option<i64>,
    pub time_diff: Option<i64>,
}

impl TapoResponseExt for GenericDeviceInfoResult {}

impl DeviceInfoResultExt for GenericDeviceInfoResult {
    fn decode(&self) -> Result<Self, Error> {
        Ok(Self {
            ssid: decode_value(&self.ssid)?,
            nickname: decode_value(&self.nickname)?,
            ..self.clone()
        })
    }
}
