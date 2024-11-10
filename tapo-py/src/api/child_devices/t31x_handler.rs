use std::{ops::Deref, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use tapo::responses::{T31XResult, TemperatureHumidityRecords};
use tapo::T31XHandler;

use crate::call_handler_method;

#[derive(Clone)]
#[pyclass(name = "T31XHandler")]
pub struct PyT31XHandler {
    inner: Arc<T31XHandler>,
}

impl PyT31XHandler {
    pub fn new(handler: T31XHandler) -> Self {
        Self {
            inner: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PyT31XHandler {
    pub async fn get_device_info(&self) -> PyResult<T31XResult> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), T31XHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(handler.deref(), T31XHandler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_temperature_humidity_records(&self) -> PyResult<TemperatureHumidityRecords> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            T31XHandler::get_temperature_humidity_records
        )
    }
}
