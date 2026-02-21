use pyo3::prelude::*;
use tapo::requests::{SegmentEffect, SegmentEffectType};

#[derive(Clone)]
#[pyclass(from_py_object, name = "SegmentEffect")]
pub struct PySegmentEffect {
    inner: SegmentEffect,
}

#[pymethods]
impl PySegmentEffect {
    #[new]
    fn new(
        name: String,
        r#type: SegmentEffectType,
        is_custom: bool,
        enabled: bool,
        brightness: u8,
        display_colors: Vec<[u16; 4]>,
    ) -> Self {
        Self {
            inner: SegmentEffect::new(name, r#type, is_custom, enabled, brightness, display_colors),
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
        display_colors: Vec<[u16; 4]>,
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

    pub fn with_type(mut slf: PyRefMut<'_, Self>, r#type: SegmentEffectType) -> PyRefMut<'_, Self> {
        (*slf).inner.r#type = Some(r#type);
        slf
    }

    pub fn with_device_type(
        mut slf: PyRefMut<'_, Self>,
        device_type: String,
    ) -> PyRefMut<'_, Self> {
        (*slf).inner.device_type = Some(device_type);
        slf
    }

    pub fn with_segments(mut slf: PyRefMut<'_, Self>, segments: Vec<u8>) -> PyRefMut<'_, Self> {
        (*slf).inner.segments = Some(segments);
        slf
    }

    pub fn with_states(mut slf: PyRefMut<'_, Self>, states: Vec<[u16; 4]>) -> PyRefMut<'_, Self> {
        (*slf).inner.states = Some(states);
        slf
    }
}

impl From<PySegmentEffect> for SegmentEffect {
    fn from(effect: PySegmentEffect) -> Self {
        effect.inner
    }
}
