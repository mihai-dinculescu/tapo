use std::{ops::Deref, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use tapo::S200Handler;
use tapo::responses::S200Result;

use crate::call_handler_method;
use crate::responses::TriggerLogsS200Result;

#[derive(Clone)]
#[pyclass(name = "S200Handler")]
pub struct PyS200Handler {
    inner: Arc<S200Handler>,
}

impl PyS200Handler {
    pub fn new(handler: S200Handler) -> Self {
        Self {
            inner: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PyS200Handler {
    pub async fn get_device_info(&self) -> PyResult<S200Result> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), S200Handler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(handler.deref(), S200Handler::get_device_info_json)?;
        Python::attach(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_trigger_logs(
        &self,
        page_size: u64,
        start_id: u64,
    ) -> PyResult<TriggerLogsS200Result> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            S200Handler::get_trigger_logs,
            page_size,
            start_id
        )
        .map(|result| result.into())
    }
}
