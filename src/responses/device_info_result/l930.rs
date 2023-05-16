use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::requests::LightingEffect;
use crate::responses::{decode_value, DefaultState, DeviceInfoResultExt, TapoResponseExt};

/// Device info of Tapo L930. Superset of [`crate::responses::GenericDeviceInfoResult`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L930DeviceInfoResult {
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
    pub hue: Option<u16>,
    pub saturation: Option<u16>,
    pub color_temp: u16,
    pub color_temp_range: [u16; 2],
    /// The default state of a device to be used when internet connectivity is lost after a power cut.
    pub default_states: DefaultState<L930StateWrapper>,
}

impl TapoResponseExt for L930DeviceInfoResult {}

impl DeviceInfoResultExt for L930DeviceInfoResult {
    fn decode(&self) -> Result<Self, Error> {
        Ok(Self {
            ssid: decode_value(&self.ssid)?,
            nickname: decode_value(&self.nickname)?,
            ..self.clone()
        })
    }
}

/// L930 State wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L930StateWrapper {
    pub state: L930State,
}

/// L930 State.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L930State {
    pub brightness: Option<u8>,
    pub hue: Option<u16>,
    pub saturation: Option<u16>,
    pub color_temp: Option<u16>,
    pub lighting_effect: Option<LightingEffect>,
}
