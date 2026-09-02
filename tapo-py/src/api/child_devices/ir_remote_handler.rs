use std::ops::Deref;
use std::sync::Arc;

use pyo3::prelude::*;
use tapo::IrRemoteHandler;

use crate::call_handler_method;

#[derive(Clone)]
#[pyclass(from_py_object, name = "IrRemoteHandler")]
pub struct PyIrRemoteHandler {
    inner: Arc<IrRemoteHandler>,
}

impl PyIrRemoteHandler {
    pub fn new(handler: IrRemoteHandler) -> Self {
        Self {
            inner: Arc::new(handler),
        }
    }
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
