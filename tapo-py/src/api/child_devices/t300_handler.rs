use std::{ops::Deref, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use tapo::responses::T300Result;
use tapo::T300Handler;

use crate::call_handler_method;
use crate::responses::TriggerLogsT300Result;

#[derive(Clone)]
#[pyclass(name = "T300Handler")]
pub struct PyT300Handler {
    inner: Arc<T300Handler>,
}

impl PyT300Handler {
    pub fn new(handler: T300Handler) -> Self {
        Self {
            inner: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PyT300Handler {
    pub async fn get_device_info(&self) -> PyResult<T300Result> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), T300Handler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(handler.deref(), T300Handler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_trigger_logs(
        &self,
        page_size: u64,
        start_id: u64,
    ) -> PyResult<TriggerLogsT300Result> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            T300Handler::get_trigger_logs,
            page_size,
            start_id
        )
        .map(|result| result.into())
    }
}
