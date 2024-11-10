use std::{ops::Deref, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use tapo::responses::S200BResult;
use tapo::S200BHandler;

use crate::call_handler_method;
use crate::responses::TriggerLogsS200BResult;

#[derive(Clone)]
#[pyclass(name = "S200BHandler")]
pub struct PyS200BHandler {
    inner: Arc<S200BHandler>,
}

impl PyS200BHandler {
    pub fn new(handler: S200BHandler) -> Self {
        Self {
            inner: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PyS200BHandler {
    pub async fn get_device_info(&self) -> PyResult<S200BResult> {
        let handler = self.inner.clone();
        call_handler_method!(handler.deref(), S200BHandler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(handler.deref(), S200BHandler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_trigger_logs(
        &self,
        page_size: u64,
        start_id: u64,
    ) -> PyResult<TriggerLogsS200BResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            S200BHandler::get_trigger_logs,
            page_size,
            start_id
        )
        .map(|result| result.into())
    }
}
