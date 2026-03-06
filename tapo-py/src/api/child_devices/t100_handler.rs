use std::ops::Deref;

use pyo3::prelude::*;
use tapo::T100Handler;
use tapo::responses::T100Result;

use crate::call_handler_method;
use crate::responses::TriggerLogsT100Result;

py_child_handler! {
    PyT100Handler(T100Handler, T100Result),
    py_name = "T100Handler",
}

#[pymethods]
impl PyT100Handler {
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
