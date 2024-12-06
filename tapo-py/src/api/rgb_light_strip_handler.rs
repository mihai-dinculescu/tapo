use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::requests::Color;
use tapo::responses::{DeviceInfoRgbLightStripResult, DeviceUsageEnergyMonitoringResult};
use tapo::{HandlerExt, RgbLightStripHandler};
use tokio::sync::RwLock;

use crate::api::PyHandlerExt;
use crate::call_handler_method;
use crate::requests::PyColorLightSetDeviceInfoParams;

#[derive(Clone)]
#[pyclass(name = "RgbLightStripHandler")]
pub struct PyRgbLightStripHandler {
    pub inner: Arc<RwLock<RgbLightStripHandler>>,
}

impl PyRgbLightStripHandler {
    pub fn new(handler: RgbLightStripHandler) -> Self {
        Self {
            inner: Arc::new(RwLock::new(handler)),
        }
    }
}

impl PyHandlerExt for PyRgbLightStripHandler {
    fn get_inner_handler(&self) -> Arc<RwLock<(impl HandlerExt + 'static)>> {
        Arc::clone(&self.inner)
    }
}

#[pymethods]
impl PyRgbLightStripHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.write().await.deref_mut(),
            RgbLightStripHandler::refresh_session,
            discard_result
        )
    }

    pub async fn on(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), RgbLightStripHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), RgbLightStripHandler::off)
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbLightStripHandler::device_reset
        )
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoRgbLightStripResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbLightStripHandler::get_device_info
        )
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            RgbLightStripHandler::get_device_info_json,
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageEnergyMonitoringResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbLightStripHandler::get_device_usage
        )
    }

    pub fn set(&self) -> PyColorLightSetDeviceInfoParams {
        PyColorLightSetDeviceInfoParams::new()
    }

    pub async fn set_brightness(&self, brightness: u8) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbLightStripHandler::set_brightness,
            brightness
        )
    }

    pub async fn set_color(&self, color: Color) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbLightStripHandler::set_color,
            color
        )
    }

    pub async fn set_hue_saturation(&self, hue: u16, saturation: u8) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbLightStripHandler::set_hue_saturation,
            hue,
            saturation
        )
    }

    pub async fn set_color_temperature(&self, color_temperature: u16) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            RgbLightStripHandler::set_color_temperature,
            color_temperature
        )
    }
}
