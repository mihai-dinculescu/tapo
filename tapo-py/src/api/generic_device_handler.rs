use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::responses::DeviceInfoGenericResult;
use tapo::GenericDeviceHandler;
use tokio::sync::RwLock;

use crate::call_handler_method;

#[derive(Clone)]
#[pyclass(name = "GenericDeviceHandler")]
pub struct PyGenericDeviceHandler {
    inner: Arc<RwLock<GenericDeviceHandler>>,
}

impl PyGenericDeviceHandler {
    pub fn new(handler: GenericDeviceHandler) -> Self {
        Self {
            inner: Arc::new(RwLock::new(handler)),
        }
    }
}

#[pymethods]
impl PyGenericDeviceHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.write().await.deref_mut(),
            GenericDeviceHandler::refresh_session,
            discard_result
        )
    }

    pub async fn on(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), GenericDeviceHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), GenericDeviceHandler::off)
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoGenericResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            GenericDeviceHandler::get_device_info
        )
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            GenericDeviceHandler::get_device_info_json,
        )?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }
}
