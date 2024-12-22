use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

/// List of preset colors as defined in the Google Home app.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, eq, eq_int))]
#[allow(missing_docs)]
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

#[cfg_attr(feature = "python", pyo3::pymethods)]
impl Color {
    /// Get the [`crate::requests::ColorConfig`] of the color.
    pub fn get_color_config(&self) -> ColorConfig {
        COLOR_MAP
            .get(self)
            .cloned()
            .unwrap_or_else(|| panic!("Failed to find the color definition of {self:?}"))
    }
}

/// Triple-tuple containing the `hue`, `saturation`, and `color_temperature` of a color.
pub type ColorConfig = (u16, u8, u16);

lazy_static! {
    static ref COLOR_MAP: HashMap<Color, ColorConfig> = {
        let mut map = HashMap::<Color, ColorConfig>::new();
        map.insert(Color::CoolWhite, (0, 100, 4000));
        map.insert(Color::Daylight, (0, 100, 5000));
        map.insert(Color::Ivory, (0, 100, 6000));
        map.insert(Color::WarmWhite, (0, 100, 3000));
        map.insert(Color::Incandescent, (0, 100, 2700));
        map.insert(Color::Candlelight, (0, 100, 2500));
        map.insert(Color::Snow, (0, 100, 6500));
        map.insert(Color::GhostWhite, (0, 100, 6500));
        map.insert(Color::AliceBlue, (208, 5, 0));
        map.insert(Color::LightGoldenrod, (54, 28, 0));
        map.insert(Color::LemonChiffon, (54, 19, 0));
        map.insert(Color::AntiqueWhite, (0, 100, 5500));
        map.insert(Color::Gold, (50, 100, 0));
        map.insert(Color::Peru, (29, 69, 0));
        map.insert(Color::Chocolate, (30, 100, 0));
        map.insert(Color::SandyBrown, (27, 60, 0));
        map.insert(Color::Coral, (16, 68, 0));
        map.insert(Color::Pumpkin, (24, 90, 0));
        map.insert(Color::Tomato, (9, 72, 0));
        map.insert(Color::Vermilion, (4, 77, 0));
        map.insert(Color::OrangeRed, (16, 100, 0));
        map.insert(Color::Pink, (349, 24, 0));
        map.insert(Color::Crimson, (348, 90, 0));
        map.insert(Color::DarkRed, (0, 100, 0));
        map.insert(Color::HotPink, (330, 58, 0));
        map.insert(Color::Smitten, (329, 67, 0));
        map.insert(Color::MediumPurple, (259, 48, 0));
        map.insert(Color::BlueViolet, (271, 80, 0));
        map.insert(Color::Indigo, (274, 100, 0));
        map.insert(Color::LightSkyBlue, (202, 46, 0));
        map.insert(Color::CornflowerBlue, (218, 57, 0));
        map.insert(Color::Ultramarine, (254, 100, 0));
        map.insert(Color::DeepSkyBlue, (195, 100, 0));
        map.insert(Color::Azure, (210, 100, 0));
        map.insert(Color::NavyBlue, (240, 100, 0));
        map.insert(Color::LightTurquoise, (180, 26, 0));
        map.insert(Color::Aquamarine, (159, 50, 0));
        map.insert(Color::Turquoise, (174, 71, 0));
        map.insert(Color::LightGreen, (120, 39, 0));
        map.insert(Color::Lime, (75, 100, 0));
        map.insert(Color::ForestGreen, (120, 75, 0));
        map
    };
}
