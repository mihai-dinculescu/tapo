use serde::{Deserialize, Serialize};
use serde_with::{BoolFromInt, serde_as};

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, eq, eq_int))]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum SegmentEffectType {
    Circulating,
    Breathe,
    Chasing,
    Flicker,
    Bloom,
    Stacking,
    None,
}

/// Parameters for the `apply_segment_effect_rule` request.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentEffect {
    /// Brightness between 0 and 100.
    pub brightness: u8,
    /// Whether the effect is custom (serialized as 1/0).
    #[serde_as(as = "BoolFromInt")]
    #[serde(rename = "custom")]
    pub is_custom: bool,
    /// Device type string expected by the device, e.g. `strip`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deviceType")]
    pub device_type: Option<String>,
    /// Colors displayed in the app.
    pub display_colors: Vec<[u16; 4]>,
    /// Whether the effect is enabled (serialized as 1/0).
    #[serde_as(as = "BoolFromInt")]
    #[serde(rename = "enable")]
    pub enabled: bool,
    /// Effect identifier. Any non-empty string is accepted by the device.
    pub id: String,
    /// Effect name shown in the app.
    pub name: String,
    /// Segment list, typically the total segment count for the strip (required for custom effects).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<u8>>,
    /// Effect state list (required for custom effects).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<[u16; 4]>>,
    /// Effect type (required for custom effects).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub r#type: Option<SegmentEffectType>,
}

#[allow(missing_docs)]
impl SegmentEffect {
    /// Creates a new segment effect with the required fields.
    pub fn new(
        name: impl Into<String>,
        r#type: SegmentEffectType,
        is_custom: bool,
        enabled: bool,
        brightness: u8,
        display_colors: Vec<[u16; 4]>,
    ) -> Self {
        Self {
            brightness,
            is_custom,
            device_type: if is_custom {
                Some("strip".to_string())
            } else {
                None
            },
            display_colors,
            enabled,
            id: uuid::Uuid::new_v4().simple().to_string(),
            name: name.into(),
            segments: None,
            states: None,
            r#type: Some(r#type),
        }
    }

    fn preset(name: impl Into<String>) -> Self {
        Self {
            brightness: 100,
            is_custom: false,
            device_type: None,
            display_colors: Vec::new(),
            enabled: true,
            id: uuid::Uuid::new_v4().simple().to_string(),
            name: name.into(),
            segments: None,
            states: None,
            r#type: None,
        }
    }

    pub fn with_brightness(mut self, brightness: u8) -> Self {
        self.brightness = brightness;
        self
    }

    pub fn with_is_custom(mut self, is_custom: bool) -> Self {
        self.is_custom = is_custom;
        self
    }

    pub fn with_display_colors(mut self, display_colors: Vec<[u16; 4]>) -> Self {
        self.display_colors = display_colors;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_type(mut self, r#type: SegmentEffectType) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn with_device_type(mut self, device_type: impl Into<String>) -> Self {
        self.device_type = Some(device_type.into());
        self
    }

    pub fn with_segments(mut self, segments: Vec<u8>) -> Self {
        self.segments = Some(segments);
        self
    }

    pub fn with_states(mut self, states: Vec<[u16; 4]>) -> Self {
        self.states = Some(states);
        self
    }

    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.is_custom {
            let segments_missing = self.segments.as_ref().is_none_or(Vec::is_empty);
            if segments_missing {
                return Err(Error::Validation {
                    field: "segments".to_string(),
                    message: "Required for custom segment effects".to_string(),
                });
            }

            let states_missing = self.states.as_ref().is_none_or(Vec::is_empty);
            if states_missing {
                return Err(Error::Validation {
                    field: "states".to_string(),
                    message: "Required for custom segment effects".to_string(),
                });
            }

            if self.r#type.is_none() {
                return Err(Error::Validation {
                    field: "type".to_string(),
                    message: "Required for custom segment effects".to_string(),
                });
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, eq, eq_int))]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SegmentEffectPreset {
    Birthday,
    Blue,
    Bonfire,
    Candlelight,
    Carnival,
    Cyan,
    Dancing,
    Dating,
    Disco,
    Dreamland,
    ElectroDance,
    Energetic,
    Excited,
    Fall,
    Family,
    Fireworks,
    FlowerField,
    Forest,
    Game,
    Green,
    Halloween,
    Happy,
    Jazz,
    Lake,
    LightGreen,
    Lyric,
    Moonlight,
    Morning,
    Movie,
    NewYear,
    Night,
    Orange,
    Pink,
    Purple,
    Quiet,
    Red,
    Relaxed,
    Rock,
    Siren,
    Sleep,
    Snow,
    Star,
    Study,
    Summer,
    Sunny,
    Sweet,
    Tense,
    Thinking,
    Universe,
    Volcano,
    Warm,
    White,
    Winter,
    Work,
    Yellow,
}

impl From<SegmentEffectPreset> for SegmentEffect {
    fn from(val: SegmentEffectPreset) -> Self {
        match val {
            SegmentEffectPreset::Birthday => val.birthday(),
            SegmentEffectPreset::Blue => val.blue(),
            SegmentEffectPreset::Bonfire => val.bonfire(),
            SegmentEffectPreset::Candlelight => val.candlelight(),
            SegmentEffectPreset::Carnival => val.carnival(),
            SegmentEffectPreset::Cyan => val.cyan(),
            SegmentEffectPreset::Dancing => val.dancing(),
            SegmentEffectPreset::Dating => val.dating(),
            SegmentEffectPreset::Disco => val.disco(),
            SegmentEffectPreset::Dreamland => val.dreamland(),
            SegmentEffectPreset::ElectroDance => val.electro_dance(),
            SegmentEffectPreset::Energetic => val.energetic(),
            SegmentEffectPreset::Excited => val.excited(),
            SegmentEffectPreset::Fall => val.fall(),
            SegmentEffectPreset::Family => val.family(),
            SegmentEffectPreset::Fireworks => val.fireworks(),
            SegmentEffectPreset::FlowerField => val.flower_field(),
            SegmentEffectPreset::Forest => val.forest(),
            SegmentEffectPreset::Game => val.game(),
            SegmentEffectPreset::Green => val.green(),
            SegmentEffectPreset::Halloween => val.halloween(),
            SegmentEffectPreset::Happy => val.happy(),
            SegmentEffectPreset::Jazz => val.jazz(),
            SegmentEffectPreset::Lake => val.lake(),
            SegmentEffectPreset::LightGreen => val.light_green(),
            SegmentEffectPreset::Lyric => val.lyric(),
            SegmentEffectPreset::Moonlight => val.moonlight(),
            SegmentEffectPreset::Morning => val.morning(),
            SegmentEffectPreset::Movie => val.movie(),
            SegmentEffectPreset::NewYear => val.new_year(),
            SegmentEffectPreset::Night => val.night(),
            SegmentEffectPreset::Orange => val.orange(),
            SegmentEffectPreset::Pink => val.pink(),
            SegmentEffectPreset::Purple => val.purple(),
            SegmentEffectPreset::Quiet => val.quiet(),
            SegmentEffectPreset::Red => val.red(),
            SegmentEffectPreset::Relaxed => val.relaxed(),
            SegmentEffectPreset::Rock => val.rock(),
            SegmentEffectPreset::Siren => val.siren(),
            SegmentEffectPreset::Sleep => val.sleep(),
            SegmentEffectPreset::Snow => val.snow(),
            SegmentEffectPreset::Star => val.star(),
            SegmentEffectPreset::Study => val.study(),
            SegmentEffectPreset::Summer => val.summer(),
            SegmentEffectPreset::Sunny => val.sunny(),
            SegmentEffectPreset::Sweet => val.sweet(),
            SegmentEffectPreset::Tense => val.tense(),
            SegmentEffectPreset::Thinking => val.thinking(),
            SegmentEffectPreset::Universe => val.universe(),
            SegmentEffectPreset::Volcano => val.volcano(),
            SegmentEffectPreset::Warm => val.warm(),
            SegmentEffectPreset::White => val.white(),
            SegmentEffectPreset::Winter => val.winter(),
            SegmentEffectPreset::Work => val.work(),
            SegmentEffectPreset::Yellow => val.yellow(),
        }
    }
}

impl SegmentEffectPreset {
    fn birthday(self) -> SegmentEffect {
        SegmentEffect::preset("birthday")
            .with_id("TapoStrip_6UV91PSxvC1LRPXIibdcuJ")
            .with_brightness(50)
            .with_display_colors(vec![
                [0, 87, 100, 0],
                [298, 58, 100, 0],
                [182, 94, 100, 0],
                [135, 80, 100, 0],
            ])
    }
    fn blue(self) -> SegmentEffect {
        SegmentEffect::preset("blue")
            .with_id("TapoStrip_3d5ofcJ90anniQnMTjzUqb")
            .with_brightness(50)
            .with_display_colors(vec![[202, 42, 100, 0]])
    }
    fn bonfire(self) -> SegmentEffect {
        SegmentEffect::preset("bonfire")
            .with_id("TapoStrip_4Gwp1kPB6EKSuf8YL9PxJT")
            .with_brightness(50)
            .with_display_colors(vec![[39, 86, 100, 0], [30, 14, 100, 0], [20, 87, 100, 0]])
    }
    fn candlelight(self) -> SegmentEffect {
        SegmentEffect::preset("candlelight")
            .with_id("TapoStrip_5MD0lPVZ6rf7B0gezimR3s")
            .with_brightness(50)
            .with_display_colors(vec![[298, 58, 100, 0], [20, 87, 100, 0]])
    }
    fn carnival(self) -> SegmentEffect {
        SegmentEffect::preset("carnival")
            .with_id("TapoStrip_3NbAjdKvkZdD9FGvsTJ6MJ")
            .with_brightness(50)
            .with_display_colors(vec![[0, 87, 100, 0], [0, 0, 100, 0], [135, 80, 100, 0]])
    }
    fn cyan(self) -> SegmentEffect {
        SegmentEffect::preset("cyan")
            .with_id("TapoStrip_2vKcyzPtr3aizuRMuYdAMW")
            .with_brightness(50)
            .with_display_colors(vec![[186, 50, 100, 0]])
    }
    fn dancing(self) -> SegmentEffect {
        SegmentEffect::preset("dancing")
            .with_id("TapoStrip_5Tt1eZzXRdj4YWGgGdqK3y")
            .with_brightness(50)
            .with_display_colors(vec![
                [298, 58, 100, 0],
                [20, 87, 100, 0],
                [48, 87, 100, 0],
                [192, 78, 100, 0],
            ])
    }
    fn dating(self) -> SegmentEffect {
        SegmentEffect::preset("dating")
            .with_id("TapoStrip_2tkyPdByKhKe4eVqE5XphX")
            .with_brightness(50)
            .with_display_colors(vec![[298, 58, 100, 0], [298, 58, 100, 0]])
    }
    fn disco(self) -> SegmentEffect {
        SegmentEffect::preset("disco")
            .with_id("TapoStrip_5pAN2kMxzsh8HpsBpfwRXP")
            .with_brightness(50)
            .with_display_colors(vec![
                [20, 87, 100, 0],
                [35, 8, 100, 0],
                [298, 58, 100, 0],
                [192, 78, 100, 0],
            ])
    }
    fn dreamland(self) -> SegmentEffect {
        SegmentEffect::preset("dreamland")
            .with_id("TapoStrip_4wHSAFyVTCtgxSw0wUeXGV")
            .with_brightness(50)
            .with_display_colors(vec![
                [298, 58, 100, 0],
                [48, 87, 100, 0],
                [216, 49, 100, 0],
                [182, 94, 100, 0],
            ])
    }
    fn electro_dance(self) -> SegmentEffect {
        SegmentEffect::preset("electro_dance")
            .with_id("TapoStrip_2s22PUbeRbRROnSuJTw8uN")
            .with_brightness(50)
            .with_display_colors(vec![
                [20, 87, 100, 0],
                [132, 67, 100, 0],
                [298, 58, 100, 0],
                [51, 58, 100, 0],
            ])
    }
    fn energetic(self) -> SegmentEffect {
        SegmentEffect::preset("energetic")
            .with_id("TapoStrip_4EPVg5TXRM5OW8fkRq4uMy")
            .with_brightness(50)
            .with_display_colors(vec![[298, 58, 100, 0], [0, 0, 100, 0], [182, 94, 100, 0]])
    }
    fn excited(self) -> SegmentEffect {
        SegmentEffect::preset("excited")
            .with_id("TapoStrip_4CVvw7fPwbDgpjdyTdwSir")
            .with_brightness(50)
            .with_display_colors(vec![
                [298, 58, 100, 0],
                [48, 87, 100, 0],
                [182, 94, 100, 0],
                [135, 80, 100, 0],
            ])
    }
    fn fall(self) -> SegmentEffect {
        SegmentEffect::preset("fall")
            .with_id("TapoStrip_4g6E1iZQyB2HGa8kIPULRk")
            .with_brightness(50)
            .with_display_colors(vec![
                [44, 86, 100, 0],
                [20, 87, 100, 0],
                [20, 87, 100, 0],
                [307, 64, 100, 0],
            ])
    }
    fn family(self) -> SegmentEffect {
        SegmentEffect::preset("family")
            .with_id("TapoStrip_4mCB0GdvVxYQUK6rK3Ny0w")
            .with_brightness(50)
            .with_display_colors(vec![
                [20, 87, 100, 0],
                [298, 58, 100, 0],
                [298, 58, 100, 0],
                [216, 46, 100, 0],
            ])
    }
    fn fireworks(self) -> SegmentEffect {
        SegmentEffect::preset("fireworks")
            .with_id("TapoStrip_0xZnAlXp2sYlXpPGsfOguo")
            .with_brightness(50)
            .with_display_colors(vec![
                [20, 87, 100, 0],
                [192, 78, 100, 0],
                [298, 58, 100, 0],
                [216, 46, 100, 0],
            ])
    }
    fn flower_field(self) -> SegmentEffect {
        SegmentEffect::preset("flower_field")
            .with_id("TapoStrip_5PNM7bkM9AsT8RTjxQrpPs")
            .with_brightness(50)
            .with_display_colors(vec![
                [48, 87, 100, 0],
                [306, 72, 100, 0],
                [306, 72, 100, 0],
                [0, 87, 100, 0],
            ])
    }
    fn forest(self) -> SegmentEffect {
        SegmentEffect::preset("forest")
            .with_id("TapoStrip_4Q8f2MgkLJOUCVYs4C3S6m")
            .with_brightness(50)
            .with_display_colors(vec![
                [182, 96, 100, 0],
                [135, 100, 100, 0],
                [135, 100, 100, 0],
                [48, 87, 100, 0],
            ])
    }
    fn game(self) -> SegmentEffect {
        SegmentEffect::preset("game")
            .with_id("TapoStrip_4RVTMKq1uKlB3pexBaHgNE")
            .with_brightness(50)
            .with_display_colors(vec![
                [135, 80, 100, 0],
                [48, 87, 100, 0],
                [48, 87, 100, 0],
                [199, 94, 100, 0],
            ])
    }
    fn green(self) -> SegmentEffect {
        SegmentEffect::preset("green")
            .with_id("TapoStrip_7597pq6KQwsfV4HVf2vsor")
            .with_brightness(50)
            .with_display_colors(vec![[137, 57, 100, 0]])
    }
    fn halloween(self) -> SegmentEffect {
        SegmentEffect::preset("halloween")
            .with_id("TapoStrip_3DSdleFU3ty3QP4Ud3ZXMD")
            .with_brightness(50)
            .with_display_colors(vec![[20, 87, 100, 0], [20, 87, 100, 0], [20, 87, 100, 0]])
    }
    fn happy(self) -> SegmentEffect {
        SegmentEffect::preset("happy")
            .with_id("TapoStrip_73Svjln3dFbgu75rL1UrKh")
            .with_brightness(50)
            .with_display_colors(vec![
                [298, 58, 100, 0],
                [216, 49, 100, 0],
                [182, 94, 100, 0],
            ])
    }
    fn jazz(self) -> SegmentEffect {
        SegmentEffect::preset("jazz")
            .with_id("TapoStrip_47WNHiq9BbSNpE822vU6RP")
            .with_brightness(50)
            .with_display_colors(vec![
                [20, 87, 100, 0],
                [298, 58, 100, 0],
                [34, 20, 100, 0],
                [48, 87, 100, 0],
            ])
    }
    fn lake(self) -> SegmentEffect {
        SegmentEffect::preset("lake")
            .with_id("TapoStrip_03Dhq9acJ7TeK5p55zNPlO")
            .with_brightness(50)
            .with_display_colors(vec![
                [188, 100, 100, 0],
                [188, 100, 100, 0],
                [49, 61, 100, 0],
            ])
    }
    fn light_green(self) -> SegmentEffect {
        SegmentEffect::preset("light_green")
            .with_id("TapoStrip_1IgXu8L5zqIRbFqhcEwk5V")
            .with_brightness(50)
            .with_display_colors(vec![[92, 49, 100, 0]])
    }
    fn lyric(self) -> SegmentEffect {
        SegmentEffect::preset("lyric")
            .with_id("TapoStrip_7L4YEbUn65YY8Lex6wHj52")
            .with_brightness(50)
            .with_display_colors(vec![
                [20, 87, 100, 0],
                [35, 8, 100, 0],
                [298, 58, 100, 0],
                [192, 78, 100, 0],
            ])
    }
    fn moonlight(self) -> SegmentEffect {
        SegmentEffect::preset("moonlight")
            .with_id("TapoStrip_2ufTEngo3cOhJvSZrvSKdV")
            .with_brightness(25)
            .with_display_colors(vec![[260, 68, 100, 0], [0, 0, 100, 0], [260, 68, 100, 0]])
    }
    fn morning(self) -> SegmentEffect {
        SegmentEffect::preset("morning")
            .with_id("TapoStrip_7B11L46nAjBGWSOQ2O2VLv")
            .with_brightness(50)
            .with_display_colors(vec![
                [182, 94, 100, 0],
                [182, 94, 100, 0],
                [182, 94, 100, 0],
            ])
    }
    fn movie(self) -> SegmentEffect {
        SegmentEffect::preset("movie")
            .with_id("TapoStrip_2I5ccWSquBU8j3Vnq8lAyd")
            .with_brightness(50)
            .with_display_colors(vec![[183, 56, 100, 0]])
    }
    fn new_year(self) -> SegmentEffect {
        SegmentEffect::preset("new_year")
            .with_id("TapoStrip_0SPC7sLfhBFFpjPin9EjWi")
            .with_brightness(50)
            .with_display_colors(vec![
                [0, 87, 100, 0],
                [298, 58, 100, 0],
                [182, 94, 100, 0],
                [135, 80, 100, 0],
            ])
    }
    fn night(self) -> SegmentEffect {
        SegmentEffect::preset("night")
            .with_id("TapoStrip_0je4XrPck2GUabCqqqtgjV")
            .with_brightness(50)
            .with_display_colors(vec![[285, 58, 100, 0], [216, 49, 100, 0]])
    }
    fn orange(self) -> SegmentEffect {
        SegmentEffect::preset("orange")
            .with_id("TapoStrip_7fld2P3UXtjklt0zeOkybx")
            .with_brightness(50)
            .with_display_colors(vec![[39, 52, 100, 0]])
    }
    fn pink(self) -> SegmentEffect {
        SegmentEffect::preset("pink")
            .with_id("TapoStrip_0ns4qSp5U0iw23YDakyR3I")
            .with_brightness(50)
            .with_display_colors(vec![[329, 30, 100, 0]])
    }
    fn purple(self) -> SegmentEffect {
        SegmentEffect::preset("purple")
            .with_id("TapoStrip_29591QOxwUNV20Pqfx720l")
            .with_brightness(50)
            .with_display_colors(vec![[281, 51, 100, 0]])
    }
    fn quiet(self) -> SegmentEffect {
        SegmentEffect::preset("quiet")
            .with_id("TapoStrip_1wCEVdKuooI1CJofw25Axw")
            .with_brightness(50)
            .with_display_colors(vec![[48, 87, 100, 0], [135, 53, 100, 0]])
    }
    fn red(self) -> SegmentEffect {
        SegmentEffect::preset("red")
            .with_id("TapoStrip_4BL9bCCMh2g1zYysCl9hlJ")
            .with_brightness(50)
            .with_display_colors(vec![[0, 51, 100, 0]])
    }
    fn relaxed(self) -> SegmentEffect {
        SegmentEffect::preset("relaxed")
            .with_id("TapoStrip_4CMF6Zyq33hi0dVCw4DEZk")
            .with_brightness(50)
            .with_display_colors(vec![[48, 87, 100, 0], [135, 80, 100, 0], [182, 94, 100, 0]])
    }
    fn rock(self) -> SegmentEffect {
        SegmentEffect::preset("rock")
            .with_id("TapoStrip_09nXMfB9vlqMRnaZtUNvNv")
            .with_brightness(50)
            .with_display_colors(vec![[12, 87, 100, 0], [49, 10, 100, 0], [192, 78, 100, 0]])
    }
    fn siren(self) -> SegmentEffect {
        SegmentEffect::preset("siren")
            .with_id("TapoStrip_45PsgBVIPAS8eAeejFKLaq")
            .with_brightness(50)
            .with_display_colors(vec![[298, 58, 100, 0], [20, 87, 100, 0], [182, 94, 100, 0]])
    }
    fn sleep(self) -> SegmentEffect {
        SegmentEffect::preset("sleep")
            .with_id("TapoStrip_5EcqkgKhof7H4VZHv6GF8W")
            .with_brightness(50)
            .with_display_colors(vec![[48, 67, 100, 0], [20, 63, 100, 0], [48, 67, 100, 0]])
    }
    fn snow(self) -> SegmentEffect {
        SegmentEffect::preset("snow")
            .with_id("TapoStrip_6J1Wz1BcYAIZ0Hr0hrLtHz")
            .with_brightness(50)
            .with_display_colors(vec![
                [0, 0, 100, 0],
                [0, 0, 100, 0],
                [0, 0, 100, 0],
                [0, 0, 100, 0],
            ])
    }
    fn star(self) -> SegmentEffect {
        SegmentEffect::preset("star")
            .with_id("TapoStrip_5BUE2IZxWBlvTyndwlGWxZ")
            .with_brightness(50)
            .with_display_colors(vec![
                [0, 0, 100, 0],
                [188, 61, 100, 0],
                [0, 0, 100, 0],
                [188, 61, 100, 0],
            ])
    }
    fn study(self) -> SegmentEffect {
        SegmentEffect::preset("study")
            .with_id("TapoStrip_1LMVpsdYvcqOvj7B49Zgsv")
            .with_brightness(50)
            .with_display_colors(vec![[0, 0, 100, 0]])
    }
    fn summer(self) -> SegmentEffect {
        SegmentEffect::preset("summer")
            .with_id("TapoStrip_3EpKcvOXOg7JOzI2CEGHfV")
            .with_brightness(50)
            .with_display_colors(vec![
                [298, 58, 100, 0],
                [182, 94, 100, 0],
                [182, 94, 100, 0],
                [298, 58, 100, 0],
            ])
    }
    fn sunny(self) -> SegmentEffect {
        SegmentEffect::preset("sunny")
            .with_id("TapoStrip_7iphEteHrkOB6RxyaHB7lq")
            .with_brightness(50)
            .with_display_colors(vec![[0, 0, 100, 0], [182, 94, 100, 0], [221, 65, 100, 0]])
    }
    fn sweet(self) -> SegmentEffect {
        SegmentEffect::preset("sweet")
            .with_id("TapoStrip_6HWuKPlEFY5Tu8iZECj8Cu")
            .with_brightness(50)
            .with_display_colors(vec![[298, 58, 100, 0], [298, 58, 100, 0]])
    }
    fn tense(self) -> SegmentEffect {
        SegmentEffect::preset("tense")
            .with_id("TapoStrip_6lFm0gKWSdV0EZkMBrEdbs")
            .with_brightness(50)
            .with_display_colors(vec![[20, 87, 100, 0], [20, 87, 100, 0], [20, 87, 100, 0]])
    }
    fn thinking(self) -> SegmentEffect {
        SegmentEffect::preset("thinking")
            .with_id("TapoStrip_0X0JpxWy3bXrg9UYIjOOHE")
            .with_brightness(50)
            .with_display_colors(vec![
                [182, 94, 100, 0],
                [182, 94, 100, 0],
                [182, 94, 100, 0],
            ])
    }
    fn universe(self) -> SegmentEffect {
        SegmentEffect::preset("universe")
            .with_id("TapoStrip_0VS5wowo18K0pg5eVlQJjo")
            .with_brightness(50)
            .with_display_colors(vec![[182, 100, 100, 0], [225, 87, 100, 0]])
    }
    fn volcano(self) -> SegmentEffect {
        SegmentEffect::preset("volcano")
            .with_id("TapoStrip_6dJUyTqdQb69WMTtYfmhXp")
            .with_brightness(50)
            .with_display_colors(vec![[20, 87, 100, 0], [20, 87, 100, 0], [20, 87, 100, 0]])
    }
    fn warm(self) -> SegmentEffect {
        SegmentEffect::preset("warm")
            .with_id("TapoStrip_6lcSZhR1fFfEK67cU35bgk")
            .with_brightness(50)
            .with_display_colors(vec![[20, 87, 100, 0], [20, 87, 100, 0]])
    }
    fn white(self) -> SegmentEffect {
        SegmentEffect::preset("white")
            .with_id("TapoStrip_3WQHPF6iH8vT8ZUVnTymzr")
            .with_brightness(50)
            .with_display_colors(vec![[0, 0, 100, 0]])
    }
    fn winter(self) -> SegmentEffect {
        SegmentEffect::preset("winter")
            .with_id("TapoStrip_0PpgOmXVg0WWdF3710KxbA")
            .with_brightness(50)
            .with_display_colors(vec![
                [182, 94, 100, 0],
                [0, 0, 100, 0],
                [0, 0, 100, 0],
                [182, 94, 100, 0],
            ])
    }
    fn work(self) -> SegmentEffect {
        SegmentEffect::preset("work")
            .with_id("TapoStrip_2Xeosp9nHkJIPL2491cbtQ")
            .with_brightness(50)
            .with_display_colors(vec![
                [187, 41, 100, 0],
                [0, 0, 100, 0],
                [0, 0, 100, 0],
                [187, 41, 100, 0],
            ])
    }
    fn yellow(self) -> SegmentEffect {
        SegmentEffect::preset("yellow")
            .with_id("TapoStrip_0m6KyfHdw3GK3ge199nxJC")
            .with_brightness(50)
            .with_display_colors(vec![[55, 75, 100, 0]])
    }
}
