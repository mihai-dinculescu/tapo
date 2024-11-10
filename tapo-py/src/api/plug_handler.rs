use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::responses::{DeviceInfoPlugResult, DeviceUsageResult};
use tapo::PlugHandler;
use tokio::sync::RwLock;

use crate::call_handler_method;

#[derive(Clone)]
#[pyclass(name = "PlugHandler")]
pub struct PyPlugHandler {
    inner: Arc<RwLock<PlugHandler>>,
}

impl PyPlugHandler {
    pub fn new(handler: PlugHandler) -> Self {
        Self {
            inner: Arc::new(RwLock::new(handler)),
        }
    }
}

#[pymethods]
impl PyPlugHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.write().await.deref_mut(),
            PlugHandler::refresh_session,
            discard_result
        )
    }

    pub async fn on(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), PlugHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), PlugHandler::off)
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), PlugHandler::device_reset)
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoPlugResult> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), PlugHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            PlugHandler::get_device_info_json
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageResult> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), PlugHandler::get_device_usage)
    }
}
