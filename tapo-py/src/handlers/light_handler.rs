use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::responses::{DeviceInfoLightResult, DeviceUsageEnergyMonitoringResult};
use tapo::LightHandler;
use tokio::sync::Mutex;

use crate::call_handler_method;
use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "LightHandler")]
pub struct PyLightHandler {
    handler: Arc<Mutex<LightHandler>>,
}

impl PyLightHandler {
    pub fn new(handler: LightHandler) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
        }
    }
}

#[pymethods]
impl PyLightHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        call_handler_method!(self, LightHandler::refresh_session, discard_result)
    }

    pub async fn on(&self) -> PyResult<()> {
        call_handler_method!(self, LightHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        call_handler_method!(self, LightHandler::off)
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        call_handler_method!(self, LightHandler::device_reset)
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoLightResult> {
        call_handler_method!(self, LightHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let result = call_handler_method!(self, LightHandler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageEnergyMonitoringResult> {
        call_handler_method!(self, LightHandler::get_device_usage)
    }

    pub async fn set_brightness(&self, brightness: u8) -> PyResult<()> {
        call_handler_method!(self, LightHandler::set_brightness, brightness)
    }
}
