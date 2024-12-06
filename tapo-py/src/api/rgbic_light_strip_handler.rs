use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::requests::{Color, LightingEffect, LightingEffectPreset};
use tapo::responses::{DeviceInfoRgbicLightStripResult, DeviceUsageEnergyMonitoringResult};
use tapo::{HandlerExt, RgbicLightStripHandler};
use tokio::sync::RwLock;

use crate::api::PyHandlerExt;
use crate::call_handler_method;
use crate::requests::{PyColorLightSetDeviceInfoParams, PyLightingEffect};

#[derive(Clone)]
#[pyclass(name = "RgbicLightStripHandler")]
pub struct PyRgbicLightStripHandler {
    pub inner: Arc<RwLock<RgbicLightStripHandler>>,
}

impl PyRgbicLightStripHandler {
    pub fn new(handler: RgbicLightStripHandler) -> Self {
        Self {
            inner: Arc::new(RwLock::new(handler)),
        }
    }
}

impl PyHandlerExt for PyRgbicLightStripHandler {
    fn get_inner_handler(&self) -> Arc<RwLock<(impl HandlerExt + 'static)>> {
        Arc::clone(&self.inner)
    }
}

#[pymethods]
impl PyRgbicLightStripHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.write().await.deref_mut(),
            RgbicLightStripHandler::refresh_session,
            discard_result
        )
    }

    pub async fn on(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), RgbicLightStripHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), RgbicLightStripHandler::off)
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbicLightStripHandler::device_reset
        )
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoRgbicLightStripResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbicLightStripHandler::get_device_info
        )
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            RgbicLightStripHandler::get_device_info_json,
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageEnergyMonitoringResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbicLightStripHandler::get_device_usage
        )
    }

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
}

fn map_lighting_effect(lighting_effect: Py<PyAny>) -> PyResult<LightingEffect> {
    if let Some(lighting_effect) =
        Python::with_gil(|py| lighting_effect.extract::<LightingEffectPreset>(py).ok())
    {
        return Ok(lighting_effect.into());
    }

    if let Some(lighting_effect) =
        Python::with_gil(|py| lighting_effect.extract::<PyLightingEffect>(py).ok())
    {
        return Ok(lighting_effect.into());
    }

    Err(PyErr::new::<PyTypeError, _>(
        "Invalid lighting effect type. Must be one of `LightingEffect` or `LightingEffectPreset`",
    ))
}
