use std::{ops::Deref, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use tapo::responses::PowerStripPlugResult;
use tapo::PowerStripPlugHandler;

use crate::call_handler_method;

#[derive(Clone)]
#[pyclass(name = "PowerStripPlugHandler")]
pub struct PyPowerStripPlugHandler {
    inner: Arc<PowerStripPlugHandler>,
}

impl PyPowerStripPlugHandler {
    pub fn new(handler: PowerStripPlugHandler) -> Self {
        Self {
            inner: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PyPowerStripPlugHandler {
    pub async fn get_device_info(&self) -> PyResult<PowerStripPlugResult> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), PowerStripPlugHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result =
            call_handler_method!(handler.deref(), PowerStripPlugHandler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn on(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), PowerStripPlugHandler::on)
    }

    pub async fn off(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), PowerStripPlugHandler::off)
    }
}
