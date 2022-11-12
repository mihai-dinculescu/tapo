use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

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
    pub(crate) static ref COLOR_MAP: HashMap<Color, ColorConfig> = {
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
