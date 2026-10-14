use std::ops::Deref;

use pyo3::prelude::*;
use tapo::HubIrHandler;
use tapo::responses::DeviceInfoHubResult;

use crate::api::PyIrRemoteHandler;
use crate::call_handler_method;

py_handler! {
    PyHubIrHandler(HubIrHandler, DeviceInfoHubResult),
    py_name = "HubIrHandler",
    device_management,
}

py_hub_alarm_handlers!(PyHubIrHandler, HubIrHandler);

py_hub_child_handlers!(
    PyHubIrHandler,
    HubIrHandler,
    ChildDeviceHubIrResult,
    IrRemote
);

#[pymethods]
impl PyHubIrHandler {
    #[pyo3(signature = (device_id=None, nickname=None))]
    pub async fn ir_remote(
        &self,
        device_id: Option<String>,
        nickname: Option<String>,
    ) -> PyResult<PyIrRemoteHandler> {
        let handler = self.inner.clone();
        let identifier = Self::parse_identifier(device_id, nickname)?;

        let child_handler = call_handler_method!(
            handler.read().await.deref(),
            HubIrHandler::ir_remote,
            identifier
        )?;
        Ok(PyIrRemoteHandler::new(child_handler))
    }

    pub async fn ir_remote_unchecked(&self, device_id: String) -> PyResult<PyIrRemoteHandler> {
        let child = self.inner.read().await.ir_remote_unchecked(device_id);
        Ok(PyIrRemoteHandler::new(child))
    }
}
