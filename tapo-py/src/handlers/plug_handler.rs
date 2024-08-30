use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::responses::{DeviceInfoPlugResult, DeviceUsageResult};
use tapo::PlugHandler;
use tokio::sync::Mutex;

use crate::call_handler_method;
use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "PlugHandler")]
pub struct PyPlugHandler {
    handler: Arc<Mutex<PlugHandler>>,
}

impl PyPlugHandler {
    pub fn new(handler: PlugHandler) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
        }
    }
}

#[pymethods]
impl PyPlugHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        call_handler_method!(self, PlugHandler::refresh_session, discard_result)
    }

    pub async fn on(&self) -> PyResult<()> {
        call_handler_method!(self, PlugHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        call_handler_method!(self, PlugHandler::off)
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        call_handler_method!(self, PlugHandler::device_reset)
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoPlugResult> {
        call_handler_method!(self, PlugHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let result = call_handler_method!(self, PlugHandler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageResult> {
        call_handler_method!(self, PlugHandler::get_device_usage)
    }
}
