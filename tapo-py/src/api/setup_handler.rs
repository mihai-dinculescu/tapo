use std::ops::Deref;
use std::sync::Arc;

use chrono_tz::Tz;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::SetupHandler;
use tapo::responses::{DeviceInfoBasicResult, WifiJoinResult, WifiNetwork, WifiSecurity};

use crate::call_handler_method;

#[derive(Clone)]
#[pyclass(from_py_object, name = "SetupHandler")]
pub struct PySetupHandler {
    inner: Arc<SetupHandler>,
}

impl PySetupHandler {
    pub fn new(handler: SetupHandler) -> Self {
        Self {
            inner: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PySetupHandler {
    #[getter]
    pub fn address(&self) -> Option<(String, u16)> {
        self.inner.address()
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoBasicResult> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), SetupHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(handler.deref(), SetupHandler::get_device_info_json)?;
        Python::attach(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn scan_wifi(&self) -> PyResult<Vec<WifiNetwork>> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), SetupHandler::scan_wifi)
    }

    pub async fn join_wifi(
        &self,
        ssid: String,
        password: String,
        security: WifiSecurity,
        timezone: Tz,
    ) -> PyResult<WifiJoinResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            SetupHandler::join_wifi,
            ssid,
            password,
            security,
            timezone
        )
    }
}
