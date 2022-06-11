use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use anyhow::Context;

#[derive(Debug, Default, Serialize)]
pub struct L530SetDeviceInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brightness: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hue: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saturation: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "color_temp")]
    color_temperature: Option<u16>,
}

/// List of preset colors as defined in the Google Home app.
#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Color {
    CoolWhite,
    Daylight,
    Ivory,
    WarmWhite,
    Incandescent,
    Candlelight,
    Snow,
    GhostWhite,
    AliceBlue,
    LightGoldenrod,
    LemonChiffon,
    AntiqueWhite,
    Gold,
    Peru,
    Chocolate,
    SandyBrown,
    Coral,
    Pumpkin,
    Tomato,
    Vermilion,
    OrangeRed,
    Pink,
    Crimson,
    DarkRed,
    HotPink,
    Smitten,
    MediumPurple,
    BlueViolet,
    Indigo,
    LightSkyBlue,
    CornflowerBlue,
    Ultramarine,
    DeepSkyBlue,
    Azure,
    NavyBlue,
    LightTurquoise,
    Aquamarine,
    Turquoise,
    LightGreen,
    Lime,
    ForestGreen,
}

type ColorConfig = (Option<u16>, Option<u8>, Option<u16>);

lazy_static! {
    pub static ref COLOR_MAP: HashMap<Color, ColorConfig> = {
        let mut map = HashMap::new();
        map.insert(Color::CoolWhite, (None, None, Some(4000)));
        map.insert(Color::Daylight, (None, None, Some(5000)));
        map.insert(Color::Ivory, (None, None, Some(6000)));
        map.insert(Color::WarmWhite, (None, None, Some(3000)));
        map.insert(Color::Incandescent, (None, None, Some(2700)));
        map.insert(Color::Candlelight, (None, None, Some(2500)));
        map.insert(Color::Snow, (None, None, Some(6500)));
        map.insert(Color::GhostWhite, (None, None, Some(6500)));
        map.insert(Color::AliceBlue, (Some(208), Some(5), None));
        map.insert(Color::LightGoldenrod, (Some(54), Some(28), None));
        map.insert(Color::LemonChiffon, (Some(54), Some(19), None));
        map.insert(Color::AntiqueWhite, (None, None, Some(5500)));
        map.insert(Color::Gold, (Some(50), Some(100), None));
        map.insert(Color::Peru, (Some(29), Some(69), None));
        map.insert(Color::Chocolate, (Some(30), Some(100), None));
        map.insert(Color::SandyBrown, (Some(27), Some(60), None));
        map.insert(Color::Coral, (Some(16), Some(68), None));
        map.insert(Color::Pumpkin, (Some(24), Some(90), None));
        map.insert(Color::Tomato, (Some(9), Some(72), None));
        map.insert(Color::Vermilion, (Some(4), Some(77), None));
        map.insert(Color::OrangeRed, (Some(16), Some(100), None));
        map.insert(Color::Pink, (Some(349), Some(24), None));
        map.insert(Color::Crimson, (Some(348), Some(90), None));
        map.insert(Color::DarkRed, (Some(0), Some(100), None));
        map.insert(Color::HotPink, (Some(330), Some(58), None));
        map.insert(Color::Smitten, (Some(329), Some(67), None));
        map.insert(Color::MediumPurple, (Some(259), Some(48), None));
        map.insert(Color::BlueViolet, (Some(271), Some(80), None));
        map.insert(Color::Indigo, (Some(274), Some(100), None));
        map.insert(Color::LightSkyBlue, (Some(202), Some(46), None));
        map.insert(Color::CornflowerBlue, (Some(218), Some(57), None));
        map.insert(Color::Ultramarine, (Some(254), Some(100), None));
        map.insert(Color::DeepSkyBlue, (Some(195), Some(100), None));
        map.insert(Color::Azure, (Some(210), Some(100), None));
        map.insert(Color::NavyBlue, (Some(240), Some(100), None));
        map.insert(Color::LightTurquoise, (Some(180), Some(26), None));
        map.insert(Color::Aquamarine, (Some(159), Some(50), None));
        map.insert(Color::Turquoise, (Some(174), Some(71), None));
        map.insert(Color::LightGreen, (Some(120), Some(39), None));
        map.insert(Color::Lime, (Some(75), Some(100), None));
        map.insert(Color::ForestGreen, (Some(120), Some(75), None));
        map
    };
}

impl L530SetDeviceInfoParams {
    pub(crate) fn brightness(value: u8) -> anyhow::Result<Self> {
        Self {
            brightness: Some(value),
            ..Default::default()
        }
        .validate()
    }

    pub(crate) fn color(color: Color) -> anyhow::Result<Self> {
        let (hue, saturation, color_temperature) = *COLOR_MAP
            .get(&color)
            .context("failed to find the color properties")?;

        Self {
            hue,
            saturation,
            color_temperature,
            ..Default::default()
        }
        .validate()
    }

    pub(crate) fn hue_saturation(hue: u16, saturation: u8) -> anyhow::Result<Self> {
        Self {
            hue: Some(hue),
            saturation: Some(saturation),
            color_temperature: None,
            ..Default::default()
        }
        .validate()
    }

    pub(crate) fn color_temperature(value: u16) -> anyhow::Result<Self> {
        Self {
            hue: None,
            saturation: None,
            color_temperature: Some(value),
            ..Default::default()
        }
        .validate()
    }

    pub(crate) fn validate(self) -> anyhow::Result<Self> {
        if self.brightness.is_none()
            && self.hue.is_none()
            && self.saturation.is_none()
            && self.color_temperature.is_none()
        {
            return Err(anyhow::anyhow!(
                "DeviceInfoParams requires at least one property"
            ));
        }

        if self.color_temperature.is_some() && (self.hue.is_some() || self.saturation.is_some()) {
            return Err(anyhow::anyhow!(
                "'color_temperature' cannot be set together with 'hue' or 'saturation'"
            ));
        }

        if (self.hue.is_some() && self.saturation.is_none())
            || (self.hue.is_none() && self.saturation.is_some())
        {
            return Err(anyhow::anyhow!(
                "'hue' and 'saturation' must be both set or unset"
            ));
        }

        if let Some(brightness) = self.brightness {
            if !(1..=100).contains(&brightness) {
                return Err(anyhow::anyhow!("'brightness' must be between 1 and 100"));
            }
        }

        if let Some(hue) = self.hue {
            if !(1..=360).contains(&hue) {
                return Err(anyhow::anyhow!("'hue' must be between 1 and 360"));
            }
        }

        if let Some(saturation) = self.saturation {
            if !(1..=100).contains(&saturation) {
                return Err(anyhow::anyhow!("'saturation' must be between 1 and 100"));
            }
        }

        if let Some(color_temp) = self.color_temperature {
            if !(2500..=6500).contains(&color_temp) {
                return Err(anyhow::anyhow!(
                    "'color_temperature' must be between 2500 and 6500"
                ));
            }
        }

        Ok(self)
    }
}
