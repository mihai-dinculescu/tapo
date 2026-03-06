use std::ops::Deref;

use pyo3::prelude::*;
use tapo::T110Handler;
use tapo::responses::T110Result;

use crate::call_handler_method;
use crate::responses::TriggerLogsT110Result;

py_child_handler! {
    PyT110Handler(T110Handler, T110Result),
    py_name = "T110Handler",
}

#[pymethods]
impl PyT110Handler {
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
