use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum LightingEffectType {
    Sequence,
    Random,
    Pulse,
    Static,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct LightingEffect {
    // Mandatory
    pub brightness: u8,
    #[serde_as(as = "BoolFromInt")]
    #[serde(rename = "custom")]
    pub is_custom: bool,
    /// The colors that will be displayed in the Tapo app.
    pub display_colors: Vec<[u16; 3]>,
    #[serde_as(as = "BoolFromInt")]
    #[serde(rename = "enable")]
    pub enabled: bool,
    pub id: String,
    pub name: String,
    pub r#type: LightingEffectType,
    // Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backgrounds: Option<Vec<[u16; 3]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness_range: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expansion_strategy: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fadeoff: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue_range: Option<[u16; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_states: Option<Vec<[u16; 3]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_times: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturation_range: Option<[u8; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<Vec<[u16; 3]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_range: Option<[u16; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "trans_sequence")]
    pub transition_sequence: Option<Vec<u16>>,
}

impl LightingEffect {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        r#type: LightingEffectType,
        is_custom: bool,
        enabled: bool,
        brightness: u8,
        display_colors: Vec<[u16; 3]>,
    ) -> Self {
        Self {
            // Mandatory
            brightness,
            is_custom,
            display_colors,
            enabled,
            id: id.into(),
            name: name.into(),
            r#type,
            // Optional
            backgrounds: None,
            brightness_range: None,
            direction: None,
            duration: None,
            expansion_strategy: None,
            fadeoff: None,
            hue_range: None,
            init_states: None,
            random_seed: None,
            repeat_times: None,
            run_time: None,
            saturation_range: None,
            segment_length: None,
            segments: None,
            sequence: None,
            spread: None,
            transition_range: None,
            transition_sequence: None,
            transition: None,
        }
    }

    pub fn new_with_random_id(
        name: impl Into<String>,
        r#type: LightingEffectType,
        custom: bool,
        enable: bool,
        brightness: u8,
        display_colors: Vec<[u16; 3]>,
    ) -> Self {
        Self::new(
            uuid::Uuid::new_v4().simple().to_string(),
            name,
            r#type,
            custom,
            enable,
            brightness,
            display_colors,
        )
    }

    pub fn with_brightness(mut self, brightness: u8) -> Self {
        self.brightness = brightness;
        self
    }

    pub fn with_is_custom(mut self, is_custom: bool) -> Self {
        self.is_custom = is_custom;
        self
    }

    pub fn with_display_colors(mut self, display_colors: Vec<[u16; 3]>) -> Self {
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

    pub fn with_type(mut self, r#type: LightingEffectType) -> Self {
        self.r#type = r#type;
        self
    }

    pub fn with_backgrounds(mut self, backgrounds: Vec<[u16; 3]>) -> Self {
        self.backgrounds = Some(backgrounds);
        self
    }

    pub fn with_brightness_range(mut self, brightness_range: [u8; 2]) -> Self {
        self.brightness_range = Some(brightness_range.to_vec());
        self
    }

    pub fn with_init_states(mut self, init_states: Vec<[u16; 3]>) -> Self {
        self.init_states = Some(init_states);
        self
    }

    pub fn with_direction(mut self, direction: u8) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn with_duration(mut self, duration: u64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_expansion_strategy(mut self, expansion_strategy: u8) -> Self {
        self.expansion_strategy = Some(expansion_strategy);
        self
    }

    pub fn with_fadeoff(mut self, fadeoff: u16) -> Self {
        self.fadeoff = Some(fadeoff);
        self
    }

    pub fn with_hue_range(mut self, hue_range: [u16; 2]) -> Self {
        self.hue_range = Some(hue_range);
        self
    }

    pub fn with_random_seed(mut self, random_seed: u64) -> Self {
        self.random_seed = Some(random_seed);
        self
    }

    pub fn with_repeat_times(mut self, repeat_times: u8) -> Self {
        self.repeat_times = Some(repeat_times);
        self
    }

    pub fn with_run_time(mut self, run_time: u64) -> Self {
        self.run_time = Some(run_time);
        self
    }

    pub fn with_saturation_range(mut self, saturation_range: [u8; 2]) -> Self {
        self.saturation_range = Some(saturation_range);
        self
    }

    pub fn with_segment_length(mut self, segment_length: u8) -> Self {
        self.segment_length = Some(segment_length);
        self
    }

    pub fn with_segments(mut self, segments: Vec<u8>) -> Self {
        self.segments = Some(segments);
        self
    }

    pub fn with_sequence(mut self, sequence: Vec<[u16; 3]>) -> Self {
        self.sequence = Some(sequence);
        self
    }

    pub fn with_spread(mut self, spread: u8) -> Self {
        self.spread = Some(spread);
        self
    }

    pub fn with_transition(mut self, transition: u16) -> Self {
        self.transition = Some(transition);
        self
    }

    pub fn with_transition_range(mut self, transition_range: [u16; 2]) -> Self {
        self.transition_range = Some(transition_range);
        self
    }

    pub fn with_transition_sequence(mut self, transition_sequence: Vec<u16>) -> Self {
        self.transition_sequence = Some(transition_sequence);
        self
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum LightingEffectPreset {
    Aurora,
    BubblingCauldron,
    CandyCane,
    Christmas,
    Flicker,
    GrandmasChristmasLights,
    Hanukkah,
    HauntedMansion,
    Icicle,
    Lightning,
    Ocean,
    Rainbow,
    Raindrop,
    Spring,
    Sunrise,
    Sunset,
    Valentines,
}

impl From<LightingEffectPreset> for LightingEffect {
    fn from(val: LightingEffectPreset) -> Self {
        match val {
            LightingEffectPreset::Aurora => val.aurora(),
            LightingEffectPreset::BubblingCauldron => val.bubbling_cauldron(),
            LightingEffectPreset::CandyCane => val.candy_cane(),
            LightingEffectPreset::Christmas => val.christmas(),
            LightingEffectPreset::Flicker => val.flicker(),
            LightingEffectPreset::GrandmasChristmasLights => val.grandmas_christmas_lights(),
            LightingEffectPreset::Hanukkah => val.hanukkah(),
            LightingEffectPreset::HauntedMansion => val.haunted_mansion(),
            LightingEffectPreset::Icicle => val.icicle(),
            LightingEffectPreset::Lightning => val.lightning(),
            LightingEffectPreset::Ocean => val.ocean(),
            LightingEffectPreset::Rainbow => val.rainbow(),
            LightingEffectPreset::Raindrop => val.raindrop(),
            LightingEffectPreset::Spring => val.spring(),
            LightingEffectPreset::Sunrise => val.sunrise(),
            LightingEffectPreset::Sunset => val.sunset(),
            LightingEffectPreset::Valentines => val.valentines(),
        }
    }
}

impl LightingEffectPreset {
    // def aurora() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_1MClvV18i15Jq3bvJVf0eP', name='Aurora', brightness=100,
    //                                display_colors=[[120, 100, 100], [240, 100, 100], [260, 100, 100], [280, 100, 100]],
    //                                enable=1, bAdjusted=None, brightness_range=[], backgrounds=[], custom=0, direction=4,
    //                                duration=0, expansion_strategy=1, fadeoff=None, hue_range=None, init_states=[],
    //                                random_seed=None, repeat_times=0, saturation_range=None, segment_length=None,
    //                                segments=[0],
    //                                sequence=[[120, 100, 100], [240, 100, 100], [260, 100, 100], [280, 100, 100]],
    //                                spread=7, transition=1500, transition_range=None, type='sequence',
    //                                trans_sequence=None, run_time=None)
    //     )
    fn aurora(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_1MClvV18i15Jq3bvJVf0eP",
            "Aurora",
            LightingEffectType::Sequence,
            false,
            true,
            100,
            vec![
                [120, 100, 100],
                [240, 100, 100],
                [260, 100, 100],
                [280, 100, 100],
            ],
        )
        .with_direction(4)
        .with_duration(0)
        .with_expansion_strategy(1)
        .with_repeat_times(0)
        .with_segments(vec![0])
        .with_sequence(vec![
            [120, 100, 100],
            [240, 100, 100],
            [260, 100, 100],
            [280, 100, 100],
        ])
        .with_spread(7)
        .with_transition(1500)
    }

    // def bubbling_calderon() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_6DlumDwO2NdfHppy50vJtu', name='Bubbling Cauldron', brightness=100,
    //                                display_colors=[[100, 100, 100], [270, 100, 100]], enable=1, bAdjusted=None,
    //                                brightness_range=[50, 100], backgrounds=[[270, 40, 50]], custom=0, direction=None,
    //                                duration=0, expansion_strategy=1, fadeoff=1000, hue_range=[100, 270],
    //                                init_states=[[270, 100, 100]], random_seed=24, repeat_times=None,
    //                                saturation_range=[80, 100], segment_length=None, segments=[0], sequence=None,
    //                                spread=None, transition=200, transition_range=None, type='random',
    //                                trans_sequence=None, run_time=None)
    //     )
    fn bubbling_cauldron(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_6DlumDwO2NdfHppy50vJtu",
            "Bubbling Cauldron",
            LightingEffectType::Random,
            false,
            true,
            100,
            vec![[100, 100, 100], [270, 100, 100]],
        )
        .with_backgrounds(vec![[270, 40, 50]])
        .with_brightness_range([50, 100])
        .with_init_states(vec![[270, 100, 100]])
        .with_duration(0)
        .with_expansion_strategy(1)
        .with_fadeoff(1000)
        .with_hue_range([100, 270])
        .with_random_seed(24)
        .with_saturation_range([80, 100])
        .with_segments(vec![0])
        .with_transition(200)
    }

    // def candy_cane() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_6Dy0Nc45vlhFPEzG021Pe9', name='Candy Cane', brightness=100,
    //                                display_colors=[[0, 0, 100], [360, 81, 100]], enable=1, bAdjusted=None,
    //                                brightness_range=[], backgrounds=[], custom=0, direction=1, duration=700,
    //                                expansion_strategy=1, fadeoff=None, hue_range=None, init_states=[], random_seed=None,
    //                                repeat_times=0, saturation_range=None, segment_length=None,
    //                                segments=[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    //                                sequence=[[0, 0, 100], [0, 0, 100], [360, 81, 100], [0, 0, 100], [0, 0, 100],
    //                                          [360, 81, 100], [360, 81, 100], [0, 0, 100], [0, 0, 100], [360, 81, 100],
    //                                          [360, 81, 100], [360, 81, 100], [360, 81, 100], [0, 0, 100], [0, 0, 100],
    //                                          [360, 81, 100]], spread=1, transition=500, transition_range=None,
    //                                type='sequence')
    //     )
    fn candy_cane(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_6Dy0Nc45vlhFPEzG021Pe9",
            "Candy Cane",
            LightingEffectType::Sequence,
            false,
            true,
            100,
            vec![[0, 0, 100], [360, 81, 100]],
        )
        .with_direction(1)
        .with_duration(700)
        .with_expansion_strategy(1)
        .with_repeat_times(0)
        .with_segments(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
        .with_sequence(vec![
            [0, 0, 100],
            [0, 0, 100],
            [360, 81, 100],
            [0, 0, 100],
            [0, 0, 100],
            [360, 81, 100],
            [360, 81, 100],
            [0, 0, 100],
            [0, 0, 100],
            [360, 81, 100],
            [360, 81, 100],
            [360, 81, 100],
            [360, 81, 100],
            [0, 0, 100],
            [0, 0, 100],
            [360, 81, 100],
        ])
        .with_spread(1)
        .with_transition(500)
    }

    // def christmas() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_5zkiG6avJ1IbhjiZbRlWvh', name='Christmas', brightness=100,
    //                                display_colors=[[136, 98, 100], [350, 97, 100]], enable=1, bAdjusted=None,
    //                                brightness_range=[50, 100],
    //                                backgrounds=[[136, 98, 75], [136, 0, 0], [350, 0, 100], [350, 97, 94]], custom=0,
    //                                direction=None, duration=5000, expansion_strategy=1, fadeoff=2000,
    //                                hue_range=[136, 146], init_states=[[136, 0, 100]], random_seed=100,
    //                                repeat_times=None, saturation_range=[90, 100], segment_length=None, segments=[0],
    //                                sequence=None, spread=None, transition=0, transition_range=None, type='random')
    //     )
    fn christmas(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_5zkiG6avJ1IbhjiZbRlWvh",
            "Christmas",
            LightingEffectType::Random,
            false,
            true,
            100,
            vec![[136, 98, 100], [350, 97, 100]],
        )
        .with_backgrounds(vec![
            [136, 98, 75],
            [136, 0, 0],
            [350, 0, 100],
            [350, 97, 94],
        ])
        .with_brightness_range([50, 100])
        .with_init_states(vec![[136, 0, 100]])
        .with_duration(5000)
        .with_expansion_strategy(1)
        .with_fadeoff(2000)
        .with_hue_range([136, 146])
        .with_random_seed(100)
        .with_saturation_range([90, 100])
        .with_segments(vec![0])
        .with_transition(0)
    }

    // def flicker() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_4HVKmMc6vEzjm36jXaGwMs', name='Flicker', brightness=100,
    //                                display_colors=[[30, 81, 100], [40, 100, 100]], enable=1, bAdjusted=None,
    //                                brightness_range=[50, 100], backgrounds=[], custom=0, direction=None, duration=0,
    //                                expansion_strategy=1, fadeoff=None, hue_range=[30, 40], init_states=[[30, 81, 80]],
    //                                random_seed=None, repeat_times=None, saturation_range=[100, 100],
    //                                segment_length=None, segments=[1], sequence=None, spread=None, transition=0,
    //                                transition_range=[375, 500], type='random')
    //     )
    fn flicker(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_4HVKmMc6vEzjm36jXaGwMs",
            "Flicker",
            LightingEffectType::Random,
            false,
            true,
            100,
            vec![[30, 81, 100], [40, 100, 100]],
        )
        .with_brightness_range([50, 100])
        .with_init_states(vec![[30, 81, 80]])
        .with_duration(0)
        .with_expansion_strategy(1)
        .with_hue_range([30, 40])
        .with_saturation_range([100, 100])
        .with_segments(vec![1])
        .with_transition(0)
        .with_transition_range([375, 500])
    }

    // def christmas_light() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_3Gk6CmXOXbjCiwz9iD543C', name="Grandma's Christmas Lights",
    //                                brightness=100,
    //                                display_colors=[[30, 100, 100], [240, 100, 100], [130, 100, 100], [0, 100, 100]],
    //                                enable=1, bAdjusted=None, brightness_range=[], backgrounds=[], custom=0, direction=1,
    //                                duration=5000, expansion_strategy=1, fadeoff=None, hue_range=None, init_states=[],
    //                                random_seed=None, repeat_times=0, saturation_range=None, segment_length=None,
    //                                segments=[0],
    //                                sequence=[[30, 100, 100], [30, 0, 0], [30, 0, 0], [240, 100, 100], [240, 0, 0],
    //                                          [240, 0, 0], [240, 0, 100], [240, 0, 0], [240, 0, 0], [130, 100, 100],
    //                                          [130, 0, 0], [130, 0, 0], [0, 100, 100], [0, 0, 0], [0, 0, 0]], spread=1,
    //                                transition=100, transition_range=None, type='sequence')
    //     )
    fn grandmas_christmas_lights(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_3Gk6CmXOXbjCiwz9iD543C",
            "Grandma's Christmas Lights",
            LightingEffectType::Sequence,
            false,
            true,
            100,
            vec![
                [30, 100, 100],
                [240, 100, 100],
                [130, 100, 100],
                [0, 100, 100],
            ],
        )
        .with_direction(1)
        .with_duration(5000)
        .with_expansion_strategy(1)
        .with_repeat_times(0)
        .with_segments(vec![0])
        .with_sequence(vec![
            [30, 100, 100],
            [30, 0, 0],
            [30, 0, 0],
            [240, 100, 100],
            [240, 0, 0],
            [240, 0, 0],
            [240, 0, 100],
            [240, 0, 0],
            [240, 0, 0],
            [130, 100, 100],
            [130, 0, 0],
            [130, 0, 0],
            [0, 100, 100],
            [0, 0, 0],
            [0, 0, 0],
        ])
        .with_spread(1)
        .with_transition(100)
    }

    // def hanukkah() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_2YTk4wramLKv5XZ9KFDVYm', name='Hanukkah', brightness=100,
    //                                display_colors=[[200, 100, 100]], enable=1, bAdjusted=None,
    //                                brightness_range=[50, 100], backgrounds=[], custom=0, direction=None, duration=1500,
    //                                expansion_strategy=1, fadeoff=None, hue_range=[200, 210], init_states=[[35, 81, 80]],
    //                                random_seed=None, repeat_times=None, saturation_range=[0, 100], segment_length=None,
    //                                segments=[1], sequence=None, spread=None, transition=0, transition_range=[400, 500],
    //                                type='random')
    //     )
    fn hanukkah(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_2YTk4wramLKv5XZ9KFDVYm",
            "Hanukkah",
            LightingEffectType::Random,
            false,
            true,
            100,
            vec![[200, 100, 100]],
        )
        .with_brightness_range([50, 100])
        .with_init_states(vec![[35, 81, 80]])
        .with_duration(1500)
        .with_expansion_strategy(1)
        .with_hue_range([200, 210])
        .with_saturation_range([0, 100])
        .with_segments(vec![1])
        .with_transition(0)
        .with_transition_range([400, 500])
    }

    // def haunted_mansion() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_4rJ6JwC7I9st3tQ8j4lwlI', name='Haunted Mansion', brightness=100,
    //                                display_colors=[[45, 10, 100]], enable=1, bAdjusted=None, brightness_range=[0, 80],
    //                                backgrounds=[[45, 10, 100]], custom=0, direction=None, duration=0,
    //                                expansion_strategy=2, fadeoff=200, hue_range=[45, 45], init_states=[[45, 10, 100]],
    //                                random_seed=1, repeat_times=None, saturation_range=[10, 10], segment_length=None,
    //                                segments=[80], sequence=None, spread=None, transition=0, transition_range=[50, 1500],
    //                                type='random')
    //     )
    fn haunted_mansion(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_4rJ6JwC7I9st3tQ8j4lwlI",
            "Haunted Mansion",
            LightingEffectType::Random,
            false,
            true,
            100,
            vec![[45, 10, 100]],
        )
        .with_backgrounds(vec![[45, 10, 100]])
        .with_brightness_range([0, 80])
        .with_init_states(vec![[45, 10, 100]])
        .with_duration(0)
        .with_expansion_strategy(2)
        .with_fadeoff(200)
        .with_hue_range([45, 45])
        .with_random_seed(1)
        .with_saturation_range([10, 10])
        .with_segments(vec![80])
        .with_transition(0)
        .with_transition_range([50, 1500])
    }

    // def icicle() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_7UcYLeJbiaxVIXCxr21tpx', name='Icicle', brightness=100,
    //                                display_colors=[[190, 100, 100]], enable=1, bAdjusted=None, brightness_range=[],
    //                                backgrounds=[], custom=0, direction=4, duration=0, expansion_strategy=1,
    //                                fadeoff=None, hue_range=None, init_states=[], random_seed=None, repeat_times=0,
    //                                saturation_range=None, segment_length=None, segments=[0],
    //                                sequence=[[190, 100, 70], [190, 100, 70], [190, 30, 50], [190, 100, 70],
    //                                          [190, 100, 70]], spread=3, transition=400, transition_range=None,
    //                                type='sequence')
    //     )
    fn icicle(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_7UcYLeJbiaxVIXCxr21tpx",
            "Icicle",
            LightingEffectType::Sequence,
            false,
            true,
            100,
            vec![[190, 100, 100]],
        )
        .with_direction(4)
        .with_duration(0)
        .with_expansion_strategy(1)
        .with_repeat_times(0)
        .with_segments(vec![0])
        .with_sequence(vec![
            [190, 100, 70],
            [190, 100, 70],
            [190, 30, 50],
            [190, 100, 70],
            [190, 100, 70],
        ])
        .with_spread(3)
        .with_transition(400)
    }

    // def lightning() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_7OGzfSfnOdhoO2ri4gOHWn', name='Lightning', brightness=100,
    //                                display_colors=[[210, 10, 100], [200, 50, 100], [200, 100, 100]], enable=1,
    //                                bAdjusted=None, brightness_range=[90, 100],
    //                                backgrounds=[[200, 100, 100], [200, 50, 10], [210, 10, 50], [240, 10, 0]], custom=0,
    //                                direction=None, duration=0, expansion_strategy=1, fadeoff=150, hue_range=[240, 240],
    //                                init_states=[[240, 30, 100]], random_seed=600, repeat_times=None,
    //                                saturation_range=[10, 11], segment_length=None,
    //                                segments=[7, 20, 23, 32, 34, 35, 49, 65, 66, 74, 80], sequence=None, spread=None,
    //                                transition=50, transition_range=None, type='random')
    //     )
    fn lightning(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_7OGzfSfnOdhoO2ri4gOHWn",
            "Lightning",
            LightingEffectType::Random,
            false,
            true,
            100,
            vec![[210, 10, 100], [200, 50, 100], [200, 100, 100]],
        )
        .with_backgrounds(vec![
            [200, 100, 100],
            [200, 50, 10],
            [210, 10, 50],
            [240, 10, 0],
        ])
        .with_brightness_range([90, 100])
        .with_init_states(vec![[240, 30, 100]])
        .with_duration(0)
        .with_expansion_strategy(1)
        .with_fadeoff(150)
        .with_hue_range([240, 240])
        .with_random_seed(600)
        .with_saturation_range([10, 11])
        .with_segments(vec![7, 20, 23, 32, 34, 35, 49, 65, 66, 74, 80])
        .with_transition(50)
    }

    // def ocean() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_0fOleCdwSgR0nfjkReeYfw', name='Ocean', brightness=100,
    //                                display_colors=[[198, 84, 100]], enable=1, bAdjusted=None, brightness_range=[],
    //                                backgrounds=[], custom=0, direction=3, duration=0, expansion_strategy=1,
    //                                fadeoff=None, hue_range=None, init_states=[], random_seed=None, repeat_times=0,
    //                                saturation_range=None, segment_length=None, segments=[0],
    //                                sequence=[[198, 84, 30], [198, 70, 30], [198, 10, 30]], spread=16, transition=2000,
    //                                transition_range=None, type='sequence')
    //     )
    fn ocean(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_0fOleCdwSgR0nfjkReeYfw",
            "Ocean",
            LightingEffectType::Sequence,
            false,
            true,
            100,
            vec![[198, 84, 100]],
        )
        .with_direction(3)
        .with_duration(0)
        .with_expansion_strategy(1)
        .with_repeat_times(0)
        .with_segments(vec![0])
        .with_sequence(vec![[198, 84, 30], [198, 70, 30], [198, 10, 30]])
        .with_spread(16)
        .with_transition(2000)
    }

    // def rainbow() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_7CC5y4lsL8pETYvmz7UOpQ', name='Rainbow', brightness=100,
    //                                display_colors=[[0, 100, 100], [100, 100, 100], [200, 100, 100], [300, 100, 100]],
    //                                enable=1, bAdjusted=None, brightness_range=[], backgrounds=[], custom=0, direction=1,
    //                                duration=0, expansion_strategy=1, fadeoff=None, hue_range=None, init_states=[],
    //                                random_seed=None, repeat_times=0, saturation_range=None, segment_length=None,
    //                                segments=[0],
    //                                sequence=[[0, 100, 100], [100, 100, 100], [200, 100, 100], [300, 100, 100]],
    //                                spread=12, transition=1500, transition_range=None, type='sequence')
    //     )
    fn rainbow(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_7CC5y4lsL8pETYvmz7UOpQ",
            "Rainbow",
            LightingEffectType::Sequence,
            false,
            true,
            100,
            vec![
                [0, 100, 100],
                [100, 100, 100],
                [200, 100, 100],
                [300, 100, 100],
            ],
        )
        .with_direction(1)
        .with_duration(0)
        .with_expansion_strategy(1)
        .with_repeat_times(0)
        .with_segments(vec![0])
        .with_sequence(vec![
            [0, 100, 100],
            [100, 100, 100],
            [200, 100, 100],
            [300, 100, 100],
        ])
        .with_spread(12)
        .with_transition(1500)
    }

    // def raindrop() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_1t2nWlTBkV8KXBZ0TWvBjs', name='Raindrop', brightness=100,
    //                                display_colors=[[200, 10, 100], [200, 20, 100]], enable=1, bAdjusted=None,
    //                                brightness_range=[10, 30], backgrounds=[[200, 40, 0]], custom=0, direction=None,
    //                                duration=0, expansion_strategy=1, fadeoff=1000, hue_range=[200, 200],
    //                                init_states=[[200, 40, 100]], random_seed=24, repeat_times=None,
    //                                saturation_range=[10, 20], segment_length=None, segments=[0], sequence=None,
    //                                spread=None, transition=1000, transition_range=None, type='random')
    //     )
    fn raindrop(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_1t2nWlTBkV8KXBZ0TWvBjs",
            "Raindrop",
            LightingEffectType::Random,
            false,
            true,
            100,
            vec![[200, 10, 100], [200, 20, 100]],
        )
        .with_backgrounds(vec![[200, 40, 0]])
        .with_brightness_range([10, 30])
        .with_init_states(vec![[200, 40, 100]])
        .with_duration(0)
        .with_expansion_strategy(1)
        .with_fadeoff(1000)
        .with_hue_range([200, 200])
        .with_random_seed(24)
        .with_saturation_range([10, 20])
        .with_segments(vec![0])
        .with_transition(1000)
    }

    // def spring() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_1nL6GqZ5soOxj71YDJOlZL', name='Spring', brightness=100,
    //                                display_colors=[[0, 30, 100], [130, 100, 100]], enable=1, bAdjusted=None,
    //                                brightness_range=[90, 100], backgrounds=[[130, 100, 40]], custom=0, direction=None,
    //                                duration=600, expansion_strategy=1, fadeoff=1000, hue_range=[0, 90],
    //                                init_states=[[80, 30, 100]], random_seed=20, repeat_times=None,
    //                                saturation_range=[30, 100], segment_length=None, segments=[0], sequence=None,
    //                                spread=None, transition=0, transition_range=[2000, 6000], type='random')
    //     )
    fn spring(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_1nL6GqZ5soOxj71YDJOlZL",
            "Spring",
            LightingEffectType::Random,
            false,
            true,
            100,
            vec![[0, 30, 100], [130, 100, 100]],
        )
        .with_backgrounds(vec![[130, 100, 40]])
        .with_brightness_range([90, 100])
        .with_init_states(vec![[80, 30, 100]])
        .with_duration(600)
        .with_expansion_strategy(1)
        .with_fadeoff(1000)
        .with_hue_range([0, 90])
        .with_random_seed(20)
        .with_saturation_range([30, 100])
        .with_segments(vec![0])
        .with_transition(0)
        .with_transition_range([2000, 6000])
    }

    // def sunrise() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_1OVSyXIsDxrt4j7OxyRvqi', name='Sunrise', brightness=100,
    //                                display_colors=[[30, 0, 100], [30, 95, 100], [0, 100, 100]], enable=1,
    //                                bAdjusted=None, brightness_range=[], backgrounds=[], custom=0, direction=1,
    //                                duration=600, expansion_strategy=2, fadeoff=None, hue_range=None, init_states=[],
    //                                random_seed=None, repeat_times=1, saturation_range=None, segment_length=None,
    //                                segments=[0],
    //                                sequence=[[0, 100, 5], [0, 100, 5], [10, 100, 6], [15, 100, 7], [20, 100, 8],
    //                                          [20, 100, 10], [30, 100, 12], [30, 95, 15], [30, 90, 20], [30, 80, 25],
    //                                          [30, 75, 30], [30, 70, 40], [30, 60, 50], [30, 50, 60], [30, 20, 70],
    //                                          [30, 0, 100]], spread=1, transition=60000, transition_range=None,
    //                                type='pulse', trans_sequence=[], run_time=0)
    //     )
    fn sunrise(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_1OVSyXIsDxrt4j7OxyRvqi",
            "Sunrise",
            LightingEffectType::Pulse,
            false,
            true,
            100,
            vec![[30, 0, 100], [30, 95, 100], [0, 100, 100]],
        )
        .with_direction(1)
        .with_duration(600)
        .with_expansion_strategy(2)
        .with_repeat_times(1)
        .with_segments(vec![0])
        .with_sequence(vec![
            [0, 100, 5],
            [0, 100, 5],
            [10, 100, 6],
            [15, 100, 7],
            [20, 100, 8],
            [20, 100, 10],
            [30, 100, 12],
            [30, 95, 15],
            [30, 90, 20],
            [30, 80, 25],
            [30, 75, 30],
            [30, 70, 40],
            [30, 60, 50],
            [30, 50, 60],
            [30, 20, 70],
            [30, 0, 100],
        ])
        .with_spread(1)
        .with_transition(60000)
        .with_run_time(0)
    }

    // def sunset() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_5NiN0Y8GAUD78p4neKk9EL', name='Sunset', brightness=100,
    //                                display_colors=[[0, 100, 100], [30, 95, 100], [30, 0, 100]], enable=1,
    //                                bAdjusted=None, brightness_range=[], backgrounds=[], custom=0, direction=1,
    //                                duration=600, expansion_strategy=2, fadeoff=None, hue_range=None, init_states=[],
    //                                random_seed=None, repeat_times=1, saturation_range=None, segment_length=None,
    //                                segments=[0],
    //                                sequence=[[30, 0, 100], [30, 20, 100], [30, 50, 99], [30, 60, 98], [30, 70, 97],
    //                                          [30, 75, 95], [30, 80, 93], [30, 90, 90], [30, 95, 85], [30, 100, 80],
    //                                          [20, 100, 70], [20, 100, 60], [15, 100, 50], [10, 100, 40], [0, 100, 30],
    //                                          [0, 100, 0]], spread=1, transition=60000, transition_range=None,
    //                                type='pulse', trans_sequence=[], run_time=0)
    //     )
    fn sunset(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_5NiN0Y8GAUD78p4neKk9EL",
            "Sunset",
            LightingEffectType::Pulse,
            false,
            true,
            100,
            vec![[0, 100, 100], [30, 95, 100], [30, 0, 100]],
        )
        .with_direction(1)
        .with_duration(600)
        .with_expansion_strategy(2)
        .with_repeat_times(1)
        .with_segments(vec![0])
        .with_sequence(vec![
            [30, 0, 100],
            [30, 20, 100],
            [30, 50, 99],
            [30, 60, 98],
            [30, 70, 97],
            [30, 75, 95],
            [30, 80, 93],
            [30, 90, 90],
            [30, 95, 85],
            [30, 100, 80],
            [20, 100, 70],
            [20, 100, 60],
            [15, 100, 50],
            [10, 100, 40],
            [0, 100, 30],
            [0, 100, 0],
        ])
        .with_spread(1)
        .with_transition(60000)
        .with_run_time(0)
    }

    // def valentines() -> 'LightEffectPreset':
    //     return LightEffectPreset(
    //         effect=LightEffectData(id='TapoStrip_2q1Vio9sSjHmaC7JS9d30l', name='Valentines', brightness=100,
    //                                display_colors=[[340, 20, 100], [20, 50, 100], [0, 100, 100], [340, 40, 100]],
    //                                enable=1, bAdjusted=None, brightness_range=[90, 100],
    //                                backgrounds=[[340, 20, 50], [20, 50, 50], [0, 100, 50]], custom=0, direction=None,
    //                                duration=600, expansion_strategy=1, fadeoff=3000, hue_range=[340, 340],
    //                                init_states=[[340, 30, 100]], random_seed=100, repeat_times=None,
    //                                saturation_range=[30, 40], segment_length=None, segments=[0], sequence=None,
    //                                spread=None, transition=2000, transition_range=None, type='random',
    //                                trans_sequence=None, run_time=None)
    //     )
    fn valentines(self) -> LightingEffect {
        LightingEffect::new(
            "TapoStrip_2q1Vio9sSjHmaC7JS9d30l",
            "Valentines",
            LightingEffectType::Random,
            false,
            true,
            100,
            vec![[340, 20, 100], [20, 50, 100], [0, 100, 100], [340, 40, 100]],
        )
        .with_backgrounds(vec![[340, 20, 50], [20, 50, 50], [0, 100, 50]])
        .with_brightness_range([90, 100])
        .with_init_states(vec![[340, 30, 100]])
        .with_duration(600)
        .with_expansion_strategy(1)
        .with_fadeoff(3000)
        .with_hue_range([340, 340])
        .with_random_seed(100)
        .with_saturation_range([30, 40])
        .with_segments(vec![0])
        .with_transition(2000)
    }
}
