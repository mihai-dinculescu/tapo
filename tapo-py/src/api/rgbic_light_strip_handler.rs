use std::ops::Deref;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use tapo::RgbicLightStripHandler;
use tapo::requests::{
    Color, LightingEffect, LightingEffectPreset, SegmentEffect, SegmentEffectPreset,
};
use tapo::responses::{DeviceInfoRgbicLightStripResult, DeviceUsageEnergyMonitoringResult};

use crate::call_handler_method;
use crate::requests::{PyColorLightSetDeviceInfoParams, PyLightingEffect, PySegmentEffect};

py_handler! {
    PyRgbicLightStripHandler(RgbicLightStripHandler, DeviceInfoRgbicLightStripResult),
    py_name = "RgbicLightStripHandler",
    on_off,
    device_management,
    device_usage = DeviceUsageEnergyMonitoringResult,
}

#[pymethods]
impl PyRgbicLightStripHandler {
    pub fn set(&self) -> PyColorLightSetDeviceInfoParams {
        PyColorLightSetDeviceInfoParams::new()
    }

    pub async fn set_brightness(&self, brightness: u8) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbicLightStripHandler::set_brightness,
            brightness
        )
    }

    pub async fn set_color(&self, color: Color) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbicLightStripHandler::set_color,
            color
        )
    }

    pub async fn set_hue_saturation(&self, hue: u16, saturation: u8) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbicLightStripHandler::set_hue_saturation,
            hue,
            saturation
        )
    }

    pub async fn set_color_temperature(&self, color_temperature: u16) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbicLightStripHandler::set_color_temperature,
            color_temperature
        )
    }

    pub async fn set_lighting_effect(&self, lighting_effect: Py<PyAny>) -> PyResult<()> {
        let handler = self.inner.clone();
        let lighting_effect = map_lighting_effect(lighting_effect)?;
        call_handler_method!(
            handler.read().await.deref(),
            RgbicLightStripHandler::set_lighting_effect,
            lighting_effect
        )
    }

    pub async fn set_segment_effect(&self, segment_effect: Py<PyAny>) -> PyResult<()> {
        let handler = self.inner.clone();
        let segment_effect = map_segment_effect(segment_effect)?;
        call_handler_method!(
            handler.read().await.deref(),
            RgbicLightStripHandler::set_segment_effect,
            segment_effect
        )
    }
}

fn map_lighting_effect(lighting_effect: Py<PyAny>) -> PyResult<LightingEffect> {
    if let Some(lighting_effect) =
        Python::attach(|py| lighting_effect.extract::<LightingEffectPreset>(py).ok())
    {
        return Ok(lighting_effect.into());
    }

    if let Some(lighting_effect) =
        Python::attach(|py| lighting_effect.extract::<PyLightingEffect>(py).ok())
    {
        return Ok(lighting_effect.into());
    }

    Err(PyErr::new::<PyTypeError, _>(
        "Invalid lighting effect type. Must be one of `LightingEffect` or `LightingEffectPreset`",
    ))
}

fn map_segment_effect(segment_effect: Py<PyAny>) -> PyResult<SegmentEffect> {
    if let Some(segment_effect) =
        Python::attach(|py| segment_effect.extract::<SegmentEffectPreset>(py).ok())
    {
        return Ok(segment_effect.into());
    }

    if let Some(segment_effect) =
        Python::attach(|py| segment_effect.extract::<PySegmentEffect>(py).ok())
    {
        return Ok(segment_effect.into());
    }

    Err(PyErr::new::<PyTypeError, _>(
        "Invalid segment effect type. Must be one of `SegmentEffect` or `SegmentEffectPreset`",
    ))
}
