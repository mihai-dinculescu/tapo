use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::requests::Color;
use tapo::responses::{DeviceInfoColorLightResult, DeviceUsageEnergyMonitoringResult};
use tapo::{ColorLightHandler, DeviceManagementExt as _, HandlerExt};
use tokio::sync::RwLock;

use crate::api::PyHandlerExt;
use crate::call_handler_method;
use crate::requests::PyColorLightSetDeviceInfoParams;

#[derive(Clone)]
#[pyclass(name = "ColorLightHandler")]
pub struct PyColorLightHandler {
    inner: Arc<RwLock<ColorLightHandler>>,
}

impl PyColorLightHandler {
    pub fn new(handler: ColorLightHandler) -> Self {
        Self {
            inner: Arc::new(RwLock::new(handler)),
        }
    }
}

impl PyHandlerExt for PyColorLightHandler {
    fn get_inner_handler(&self) -> Arc<RwLock<impl HandlerExt + 'static>> {
        Arc::clone(&self.inner)
    }
}

#[pymethods]
impl PyColorLightHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.write().await.deref_mut(),
            ColorLightHandler::refresh_session,
            discard_result
        )
    }

    pub async fn on(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), ColorLightHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), ColorLightHandler::off)
    }

    pub async fn device_reboot(&self, delay_s: u16) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::device_reboot,
            delay_s
        )
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::device_reset,
        )
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoColorLightResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::get_device_info
        )
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::get_device_info_json,
        )?;
        Python::attach(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageEnergyMonitoringResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::get_device_usage
        )
    }

    pub fn set(&self) -> PyColorLightSetDeviceInfoParams {
        PyColorLightSetDeviceInfoParams::new()
    }

    pub async fn set_brightness(&self, brightness: u8) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::set_brightness,
            brightness
        )
    }

    pub async fn set_color(&self, color: Color) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::set_color,
            color
        )
    }

    pub async fn set_hue_saturation(&self, hue: u16, saturation: u8) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::set_hue_saturation,
            hue,
            saturation
        )
    }

    pub async fn set_color_temperature(&self, color_temperature: u16) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            ColorLightHandler::set_color_temperature,
            color_temperature
        )
    }
}
