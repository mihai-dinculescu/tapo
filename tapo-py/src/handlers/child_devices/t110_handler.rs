use std::{ops::Deref, sync::Arc};

use pyo3::{prelude::*, types::PyDict};
use serde::{Deserialize, Serialize};
use tapo::responses::{T110Log, T110Result, TriggerLogsResult};
use tapo::T110Handler;

use crate::call_handler_method;

#[derive(Clone)]
#[pyclass(name = "T110Handler")]
pub struct PyT110Handler {
    handler: Arc<T110Handler>,
}

impl PyT110Handler {
    pub fn new(handler: T110Handler) -> Self {
        Self {
            handler: Arc::new(handler),
        }
    }
}

#[pymethods]
impl PyT110Handler {
    pub async fn get_device_info(&self) -> PyResult<T110Result> {
        let handler = self.handler.clone();
        call_handler_method!(handler.deref(), T110Handler::get_device_info)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.handler.clone();
        let result = call_handler_method!(handler.deref(), T110Handler::get_device_info_json)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_trigger_logs(
        &self,
        page_size: u64,
        start_id: u64,
    ) -> PyResult<TriggerLogsT110Result> {
        let handler = self.handler.clone();
        call_handler_method!(
            handler.deref(),
            T110Handler::get_trigger_logs,
            page_size,
            start_id
        )
        .map(|result| result.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyo3::prelude::pyclass(get_all)]
#[allow(missing_docs)]
pub struct TriggerLogsT110Result {
    start_id: u64,
    sum: u64,
    logs: Vec<T110Log>,
}

impl From<TriggerLogsResult<T110Log>> for TriggerLogsT110Result {
    fn from(result: TriggerLogsResult<T110Log>) -> Self {
        Self {
            start_id: result.start_id,
            sum: result.sum,
            logs: result.logs,
        }
    }
}

#[pyo3::pymethods]
impl TriggerLogsT110Result {
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        tapo::python::serde_object_to_py_dict(py, &value)
    }
}
