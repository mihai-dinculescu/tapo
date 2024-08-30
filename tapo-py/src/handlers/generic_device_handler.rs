use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::responses::DeviceInfoGenericResult;
use tapo::GenericDeviceHandler;
use tokio::sync::Mutex;

use crate::call_handler_method;
use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "GenericDeviceHandler")]
pub struct PyGenericDeviceHandler {
    handler: Arc<Mutex<GenericDeviceHandler>>,
}

impl PyGenericDeviceHandler {
    pub fn new(handler: GenericDeviceHandler) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
        }
    }
}

#[pymethods]
impl PyGenericDeviceHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        call_handler_method!(self, GenericDeviceHandler::refresh_session, discard_result)
    }

    pub async fn on(&self) -> PyResult<()> {
        call_handler_method!(self, GenericDeviceHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        call_handler_method!(self, GenericDeviceHandler::off)
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoGenericResult> {
        call_handler_method!(self, GenericDeviceHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let result = call_handler_method!(self, GenericDeviceHandler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }
}
