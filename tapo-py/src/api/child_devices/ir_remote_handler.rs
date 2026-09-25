use std::ops::Deref;

use pyo3::prelude::*;
use tapo::IrRemoteHandler;
use tapo::responses::IrRemoteResult;

use crate::call_handler_method;

py_child_handler! {
    PyIrRemoteHandler(IrRemoteHandler, IrRemoteResult),
    py_name = "IrRemoteHandler",
}

#[pymethods]
impl PyIrRemoteHandler {
    pub async fn send_ir_cmd_by_id(&self, key_name: String) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            IrRemoteHandler::send_ir_cmd_by_id,
            key_name
        )
    }
}
