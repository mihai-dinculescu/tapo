use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

/// List of preset colors as defined in the Google Home app.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, eq, hash, frozen)
)]
pub enum Color {
    /// White light at 4000 K.
    CoolWhite,
    /// White light at 5000 K.
    Daylight,
    /// White light at 6000 K.
    Ivory,
    /// White light at 3000 K.
    WarmWhite,
    /// White light at 2700 K.
    Incandescent,
    /// White light at 2500 K.
    Candlelight,
    /// White light at 6500 K.
    Snow,
    /// White light at 6500 K.
    GhostWhite,
    /// Hue 208°, saturation 5%.
    AliceBlue,
    /// Hue 54°, saturation 28%.
    LightGoldenrod,
    /// Hue 54°, saturation 19%.
    LemonChiffon,
    /// White light at 5500 K.
    AntiqueWhite,
    /// Hue 50°, saturation 100%.
    Gold,
    /// Hue 29°, saturation 69%.
    Peru,
    /// Hue 30°, saturation 100%.
    Chocolate,
    /// Hue 27°, saturation 60%.
    SandyBrown,
    /// Hue 16°, saturation 68%.
    Coral,
    /// Hue 24°, saturation 90%.
    Pumpkin,
    /// Hue 9°, saturation 72%.
    Tomato,
    /// Hue 4°, saturation 77%.
    Vermilion,
    /// Hue 16°, saturation 100%.
    OrangeRed,
    /// Hue 349°, saturation 24%.
    Pink,
    /// Hue 348°, saturation 90%.
    Crimson,
    /// Hue 0°, saturation 100%.
    DarkRed,
    /// Hue 330°, saturation 58%.
    HotPink,
    /// Hue 329°, saturation 67%.
    Smitten,
    /// Hue 259°, saturation 48%.
    MediumPurple,
    /// Hue 271°, saturation 80%.
    BlueViolet,
    /// Hue 274°, saturation 100%.
    Indigo,
    /// Hue 202°, saturation 46%.
    LightSkyBlue,
    /// Hue 218°, saturation 57%.
    CornflowerBlue,
    /// Hue 254°, saturation 100%.
    Ultramarine,
    /// Hue 195°, saturation 100%.
    DeepSkyBlue,
    /// Hue 210°, saturation 100%.
    Azure,
    /// Hue 240°, saturation 100%.
    NavyBlue,
    /// Hue 180°, saturation 26%.
    LightTurquoise,
    /// Hue 159°, saturation 50%.
    Aquamarine,
    /// Hue 174°, saturation 71%.
    Turquoise,
    /// Hue 120°, saturation 39%.
    LightGreen,
    /// Hue 75°, saturation 100%.
    Lime,
    /// Hue 120°, saturation 75%.
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
