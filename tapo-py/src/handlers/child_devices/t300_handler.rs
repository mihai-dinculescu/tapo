use std::{ops::Deref, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use serde::{Deserialize, Serialize};
use tapo::responses::{T300Log, T300Result, TriggerLogsResult};
use tapo::T300Handler;

use crate::call_handler_method;

#[derive(Clone)]
#[pyclass(name = "T300Handler")]
pub struct PyT300Handler {
    handler: Arc<T300Handler>,
}

impl PyT300Handler {
    pub fn new(handler: T300Handler) -> Self {
        Self {
            handler: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PyT300Handler {
    pub async fn get_device_info(&self) -> PyResult<T300Result> {
        let handler = self.handler.clone();
        call_handler_method!(handler.deref(), T300Handler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.handler.clone();
        let result = call_handler_method!(handler.deref(), T300Handler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_trigger_logs(
        &self,
        page_size: u64,
        start_id: u64,
    ) -> PyResult<TriggerLogsT300Result> {
        let handler = self.handler.clone();
        call_handler_method!(
            handler.deref(),
            T300Handler::get_trigger_logs,
            page_size,
            start_id
        )
        .map(|result| result.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyo3::prelude::pyclass(get_all)]
#[allow(missing_docs)]
pub struct TriggerLogsT300Result {
    start_id: u64,
    sum: u64,
    logs: Vec<T300Log>,
}

impl From<TriggerLogsResult<T300Log>> for TriggerLogsT300Result {
    fn from(result: TriggerLogsResult<T300Log>) -> Self {
        Self {
            start_id: result.start_id,
            sum: result.sum,
            logs: result.logs,
        }
    }
}

#[pyo3::pymethods]
impl TriggerLogsT300Result {
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        tapo::python::serde_object_to_py_dict(py, &value)
    }
}
