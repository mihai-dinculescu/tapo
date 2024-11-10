use std::{ops::Deref, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use tapo::{responses::T100Result, T100Handler};

use crate::call_handler_method;
use crate::responses::TriggerLogsT100Result;

#[derive(Clone)]
#[pyclass(name = "T100Handler")]
pub struct PyT100Handler {
    inner: Arc<T100Handler>,
}

impl PyT100Handler {
    pub fn new(handler: T100Handler) -> Self {
        Self {
            inner: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PyT100Handler {
    pub async fn get_device_info(&self) -> PyResult<T100Result> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), T100Handler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(handler.deref(), T100Handler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_trigger_logs(
        &self,
        page_size: u64,
        start_id: u64,
    ) -> PyResult<TriggerLogsT100Result> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            T100Handler::get_trigger_logs,
            page_size,
            start_id
        )
        .map(|result| result.into())
    }
}
