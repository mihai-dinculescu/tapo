use std::ops::Deref;

use pyo3::prelude::*;
use tapo::S200Handler;
use tapo::responses::S200Result;

use crate::call_handler_method;
use crate::responses::TriggerLogsS200Result;

py_child_handler! {
    PyS200Handler(S200Handler, S200Result),
    py_name = "S200Handler",
}

#[pymethods]
impl PyS200Handler {
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
