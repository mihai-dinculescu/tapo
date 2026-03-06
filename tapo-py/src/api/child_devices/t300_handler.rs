use std::ops::Deref;

use pyo3::prelude::*;
use tapo::T300Handler;
use tapo::responses::T300Result;

use crate::call_handler_method;
use crate::responses::TriggerLogsT300Result;

py_child_handler! {
    PyT300Handler(T300Handler, T300Result),
    py_name = "T300Handler",
}

#[pymethods]
impl PyT300Handler {
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
