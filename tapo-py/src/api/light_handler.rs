use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::responses::{DeviceInfoLightResult, DeviceUsageEnergyMonitoringResult};
use tapo::LightHandler;
use tokio::sync::RwLock;

use crate::call_handler_method;

#[derive(Clone)]
#[pyclass(name = "LightHandler")]
pub struct PyLightHandler {
    inner: Arc<RwLock<LightHandler>>,
}

impl PyLightHandler {
    pub fn new(handler: LightHandler) -> Self {
        Self {
            inner: Arc::new(RwLock::new(handler)),
        }
    }
}

#[pymethods]
impl PyLightHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.write().await.deref_mut(),
            LightHandler::refresh_session,
            discard_result
        )
    }

    pub async fn on(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), LightHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), LightHandler::off)
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), LightHandler::device_reset)
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoLightResult> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), LightHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            LightHandler::get_device_info_json
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageEnergyMonitoringResult> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), LightHandler::get_device_usage)
    }

    pub async fn set_brightness(&self, brightness: u8) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            LightHandler::set_brightness,
            brightness
        )
    }
}
