use std::ops::Deref;

use pyo3::prelude::*;
use tapo::S210Handler;
use tapo::responses::{DeviceUsageResult, S210Result};

use crate::call_handler_method;

py_child_handler! {
    PyS210Handler(S210Handler, S210Result),
    py_name = "S210Handler",
    on_off,
}

#[pymethods]
impl PyS210Handler {
    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageResult> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), S210Handler::get_device_usage)
    }
}
