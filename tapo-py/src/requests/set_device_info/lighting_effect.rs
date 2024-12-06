use pyo3::prelude::*;
use tapo::requests::{LightingEffect, LightingEffectType};

#[derive(Clone)]
#[pyclass(name = "LightingEffect")]
pub struct PyLightingEffect {
    inner: LightingEffect,
}

#[pymethods]
impl PyLightingEffect {
    #[new]
    fn new(
        name: String,
        r#type: LightingEffectType,
        is_custom: bool,
        enabled: bool,
        brightness: u8,
        display_colors: Vec<[u16; 3]>,
    ) -> Self {
        Self {
            inner: LightingEffect::new(
                name,
                r#type,
                is_custom,
                enabled,
                brightness,
                display_colors,
            ),
        }
    }

    pub fn with_brightness(mut slf: PyRefMut<'_, Self>, brightness: u8) -> PyRefMut<'_, Self> {
        (*slf).inner.brightness = brightness;
        slf
    }

    pub fn with_is_custom(mut slf: PyRefMut<'_, Self>, is_custom: bool) -> PyRefMut<'_, Self> {
        (*slf).inner.is_custom = is_custom;
        slf
    }

    pub fn with_display_colors(
        mut slf: PyRefMut<'_, Self>,
        display_colors: Vec<[u16; 3]>,
    ) -> PyRefMut<'_, Self> {
        (*slf).inner.display_colors = display_colors;
        slf
    }

    pub fn with_enabled(mut slf: PyRefMut<'_, Self>, enabled: bool) -> PyRefMut<'_, Self> {
        (*slf).inner.enabled = enabled;
        slf
    }

    pub fn with_id(mut slf: PyRefMut<'_, Self>, id: String) -> PyRefMut<'_, Self> {
        (*slf).inner.id = id;
        slf
    }

    pub fn with_name(mut slf: PyRefMut<'_, Self>, name: String) -> PyRefMut<'_, Self> {
        (*slf).inner.name = name;
        slf
    }

    pub fn with_type(
        mut slf: PyRefMut<'_, Self>,
        r#type: LightingEffectType,
    ) -> PyRefMut<'_, Self> {
        (*slf).inner.r#type = r#type;
        slf
    }

    pub fn with_backgrounds(
        mut slf: PyRefMut<'_, Self>,
        backgrounds: Vec<[u16; 3]>,
    ) -> PyRefMut<'_, Self> {
        (*slf).inner.backgrounds = Some(backgrounds);
        slf
    }

    pub fn with_brightness_range(
        mut slf: PyRefMut<'_, Self>,
        brightness_range: [u8; 2],
    ) -> PyRefMut<'_, Self> {
        (*slf).inner.brightness_range = Some(brightness_range.to_vec());
        slf
    }

    pub fn with_direction(mut slf: PyRefMut<'_, Self>, direction: u8) -> PyRefMut<'_, Self> {
        (*slf).inner.direction = Some(direction);
        slf
    }

    pub fn with_duration(mut slf: PyRefMut<'_, Self>, duration: u64) -> PyRefMut<'_, Self> {
        (*slf).inner.duration = Some(duration);
        slf
    }

    pub fn with_expansion_strategy(
        mut slf: PyRefMut<'_, Self>,
        expansion_strategy: u8,
    ) -> PyRefMut<'_, Self> {
        (*slf).inner.expansion_strategy = Some(expansion_strategy);
        slf
    }

    pub fn with_fade_off(mut slf: PyRefMut<'_, Self>, fade_off: u16) -> PyRefMut<'_, Self> {
        (*slf).inner.fade_off = Some(fade_off);
        slf
    }

    pub fn with_hue_range(mut slf: PyRefMut<'_, Self>, hue_range: [u16; 2]) -> PyRefMut<'_, Self> {
        (*slf).inner.hue_range = Some(hue_range);
        slf
    }

    pub fn with_init_states(
        mut slf: PyRefMut<'_, Self>,
        init_states: Vec<[u16; 3]>,
    ) -> PyRefMut<'_, Self> {
        (*slf).inner.init_states = Some(init_states);
        slf
    }

    pub fn with_random_seed(mut slf: PyRefMut<'_, Self>, random_seed: u64) -> PyRefMut<'_, Self> {
        (*slf).inner.random_seed = Some(random_seed);
        slf
    }

    pub fn with_repeat_times(mut slf: PyRefMut<'_, Self>, repeat_times: u8) -> PyRefMut<'_, Self> {
        (*slf).inner.repeat_times = Some(repeat_times);
        slf
    }

    pub fn with_run_time(mut slf: PyRefMut<'_, Self>, run_time: u64) -> PyRefMut<'_, Self> {
        (*slf).inner.run_time = Some(run_time);
        slf
    }

    pub fn with_saturation_range(
        mut slf: PyRefMut<'_, Self>,
        saturation_range: [u8; 2],
    ) -> PyRefMut<'_, Self> {
        (*slf).inner.saturation_range = Some(saturation_range);
        slf
    }

    pub fn with_segment_length(
        mut slf: PyRefMut<'_, Self>,
        segment_length: u8,
    ) -> PyRefMut<'_, Self> {
        (*slf).inner.segment_length = Some(segment_length);
        slf
    }

    pub fn with_segments(mut slf: PyRefMut<'_, Self>, segments: Vec<u8>) -> PyRefMut<'_, Self> {
        (*slf).inner.segments = Some(segments);
        slf
    }

    pub fn with_sequence(
        mut slf: PyRefMut<'_, Self>,
        sequence: Vec<[u16; 3]>,
    ) -> PyRefMut<'_, Self> {
        (*slf).inner.sequence = Some(sequence);
        slf
    }

    pub fn with_spread(mut slf: PyRefMut<'_, Self>, spread: u8) -> PyRefMut<'_, Self> {
        (*slf).inner.spread = Some(spread);
        slf
    }

    pub fn with_transition(mut slf: PyRefMut<'_, Self>, transition: u16) -> PyRefMut<'_, Self> {
        (*slf).inner.transition = Some(transition);
        slf
    }

    pub fn with_transition_range(
        mut slf: PyRefMut<'_, Self>,
        transition_range: [u16; 2],
    ) -> PyRefMut<'_, Self> {
        (*slf).inner.transition_range = Some(transition_range);
        slf
    }

    pub fn with_transition_sequence(
        mut slf: PyRefMut<'_, Self>,
        transition_sequence: Vec<u16>,
    ) -> PyRefMut<'_, Self> {
        (*slf).inner.transition_sequence = Some(transition_sequence);
        slf
    }
}

impl From<PyLightingEffect> for LightingEffect {
    fn from(effect: PyLightingEffect) -> Self {
        effect.inner
    }
}
