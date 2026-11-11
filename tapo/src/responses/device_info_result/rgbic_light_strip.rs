use serde::{Deserialize, Deserializer, Serialize};

use crate::error::Error;
use crate::requests::LightingEffect;
use crate::responses::{DecodableResultExt, DefaultStateType, TapoResponseExt, decode_value};
use crate::utils::bool_from_int_or_bool;

/// Device info of Tapo L920 and L930.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DeviceInfoRgbicLightStripResult {
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
    pub default_states: DefaultRgbicLightStripState,
    pub device_on: bool,
    pub hue: Option<u16>,
    /// The lighting effect that the device ran last, which is running now when
    /// its `enabled` is `true`. It is `None` when the device has not run one yet.
    #[serde(deserialize_with = "lighting_effect_or_none")]
    pub lighting_effect: Option<LightingEffectSummary>,
    pub nickname: String,
    pub overheated: bool,
    pub saturation: Option<u16>,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DeviceInfoRgbicLightStripResult);

impl TapoResponseExt for DeviceInfoRgbicLightStripResult {}

impl DecodableResultExt for DeviceInfoRgbicLightStripResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.ssid = decode_value(&self.ssid)?;
        self.nickname = decode_value(&self.nickname)?;

        Ok(self)
    }
}

/// The lighting effect that an L920 or L930 ran last, as its device info reports it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct LightingEffectSummary {
    pub brightness: u8,
    /// The colors that are displayed in the Tapo app.
    pub display_colors: Vec<[u16; 3]>,
    /// Whether the lighting effect is running. It is `false` while the device
    /// is off, and once a color or another kind of effect has replaced it.
    #[serde(alias = "enable", deserialize_with = "bool_from_int_or_bool")]
    pub enabled: bool,
    pub id: String,
    /// Whether the lighting effect was created in the Tapo app, as opposed to
    /// being one of its presets.
    #[serde(alias = "custom", deserialize_with = "bool_from_int_or_bool")]
    pub is_custom: bool,
    pub name: String,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(LightingEffectSummary);

/// A strip that has not run a lighting effect yet reports a placeholder with an
/// empty `id`, named after its Wi-Fi state (`softAP` during setup, `station`
/// afterwards). Until the strip reboots, the placeholder's `enable` is 1 even
/// while the strip is off (L930, firmware 1.4.3), so it is dropped.
fn lighting_effect_or_none<'de, D>(
    deserializer: D,
) -> Result<Option<LightingEffectSummary>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<LightingEffectSummary>::deserialize(deserializer)?
        .filter(|lighting_effect| !lighting_effect.id.is_empty()))
}

/// RGB IC Light Strip Default State.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct DefaultRgbicLightStripState {
    pub r#type: DefaultStateType,
    pub state: RgbicLightStripState,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(DefaultRgbicLightStripState);

/// RGB IC Light Strip State.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct RgbicLightStripState {
    pub brightness: Option<u8>,
    pub hue: Option<u16>,
    pub saturation: Option<u16>,
    pub color_temp: Option<u16>,
    pub lighting_effect: Option<LightingEffect>,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(RgbicLightStripState);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lighting_effect_summary_parse() {
        let summary: LightingEffectSummary = serde_json::from_str(
            r#"{
                "brightness": 100,
                "custom": 0,
                "display_colors": [[120, 100, 100], [240, 100, 100]],
                "enable": 1,
                "id": "TapoStrip_1MClvV18i15Jq3bvJVf0eP",
                "name": "Aurora"
            }"#,
        )
        .unwrap();

        assert_eq!(summary.name, "Aurora");
        assert_eq!(summary.brightness, 100);
        assert!(summary.enabled);
        assert!(!summary.is_custom);
        assert_eq!(summary.display_colors, [[120, 100, 100], [240, 100, 100]]);
    }

    #[test]
    fn test_lighting_effect_placeholder_is_none() {
        let mut deserializer = serde_json::Deserializer::from_str(
            r#"{
                "brightness": 50,
                "custom": 0,
                "display_colors": [],
                "enable": 1,
                "id": "",
                "name": "station"
            }"#,
        );

        assert!(
            lighting_effect_or_none(&mut deserializer)
                .unwrap()
                .is_none()
        );
    }
}
