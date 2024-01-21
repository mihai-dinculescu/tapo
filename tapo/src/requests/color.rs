use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

/// List of preset colors as defined in the Google Home app.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
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

type ColorConfig = (Option<u16>, Option<u8>, Option<u16>);

lazy_static! {
    pub(crate) static ref COLOR_MAP: HashMap<Color, ColorConfig> = {
        let mut map = HashMap::new();
        map.insert(Color::CoolWhite, (Some(0), Some(100), Some(4000)));
        map.insert(Color::Daylight, (Some(0), Some(100), Some(5000)));
        map.insert(Color::Ivory, (Some(0), Some(100), Some(6000)));
        map.insert(Color::WarmWhite, (Some(0), Some(100), Some(3000)));
        map.insert(Color::Incandescent, (Some(0), Some(100), Some(2700)));
        map.insert(Color::Candlelight, (Some(0), Some(100), Some(2500)));
        map.insert(Color::Snow, (Some(0), Some(100), Some(6500)));
        map.insert(Color::GhostWhite, (Some(0), Some(100), Some(6500)));
        map.insert(Color::AliceBlue, (Some(208), Some(5), Some(0)));
        map.insert(Color::LightGoldenrod, (Some(54), Some(28), Some(0)));
        map.insert(Color::LemonChiffon, (Some(54), Some(19), Some(0)));
        map.insert(Color::AntiqueWhite, (Some(0), Some(100), Some(5500)));
        map.insert(Color::Gold, (Some(50), Some(100), Some(0)));
        map.insert(Color::Peru, (Some(29), Some(69), Some(0)));
        map.insert(Color::Chocolate, (Some(30), Some(100), Some(0)));
        map.insert(Color::SandyBrown, (Some(27), Some(60), Some(0)));
        map.insert(Color::Coral, (Some(16), Some(68), Some(0)));
        map.insert(Color::Pumpkin, (Some(24), Some(90), Some(0)));
        map.insert(Color::Tomato, (Some(9), Some(72), Some(0)));
        map.insert(Color::Vermilion, (Some(4), Some(77), Some(0)));
        map.insert(Color::OrangeRed, (Some(16), Some(100), Some(0)));
        map.insert(Color::Pink, (Some(349), Some(24), Some(0)));
        map.insert(Color::Crimson, (Some(348), Some(90), Some(0)));
        map.insert(Color::DarkRed, (Some(0), Some(100), Some(0)));
        map.insert(Color::HotPink, (Some(330), Some(58), Some(0)));
        map.insert(Color::Smitten, (Some(329), Some(67), Some(0)));
        map.insert(Color::MediumPurple, (Some(259), Some(48), Some(0)));
        map.insert(Color::BlueViolet, (Some(271), Some(80), Some(0)));
        map.insert(Color::Indigo, (Some(274), Some(100), Some(0)));
        map.insert(Color::LightSkyBlue, (Some(202), Some(46), Some(0)));
        map.insert(Color::CornflowerBlue, (Some(218), Some(57), Some(0)));
        map.insert(Color::Ultramarine, (Some(254), Some(100), Some(0)));
        map.insert(Color::DeepSkyBlue, (Some(195), Some(100), Some(0)));
        map.insert(Color::Azure, (Some(210), Some(100), Some(0)));
        map.insert(Color::NavyBlue, (Some(240), Some(100), Some(0)));
        map.insert(Color::LightTurquoise, (Some(180), Some(26), Some(0)));
        map.insert(Color::Aquamarine, (Some(159), Some(50), Some(0)));
        map.insert(Color::Turquoise, (Some(174), Some(71), Some(0)));
        map.insert(Color::LightGreen, (Some(120), Some(39), Some(0)));
        map.insert(Color::Lime, (Some(75), Some(100), Some(0)));
        map.insert(Color::ForestGreen, (Some(120), Some(75), Some(0)));
        map
    };
}
