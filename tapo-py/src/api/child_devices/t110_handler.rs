use std::{ops::Deref, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use tapo::responses::T110Result;
use tapo::T110Handler;

use crate::call_handler_method;
use crate::responses::TriggerLogsT110Result;

#[derive(Clone)]
#[pyclass(name = "T110Handler")]
pub struct PyT110Handler {
    inner: Arc<T110Handler>,
}

impl PyT110Handler {
    pub fn new(handler: T110Handler) -> Self {
        Self {
            inner: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PyT110Handler {
    pub async fn get_device_info(&self) -> PyResult<T110Result> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), T110Handler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(handler.deref(), T110Handler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_trigger_logs(
        &self,
        page_size: u64,
        start_id: u64,
    ) -> PyResult<TriggerLogsT110Result> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            T110Handler::get_trigger_logs,
            page_size,
            start_id
        )
        .map(|result| result.into())
    }
}
