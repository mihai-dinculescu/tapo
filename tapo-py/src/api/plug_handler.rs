use std::ops::Deref;
use std::time::Duration;

use pyo3::prelude::*;
use tapo::PlugHandler;
use tapo::responses::{DeviceInfoPlugResult, DeviceUsageResult, PowerState, Timer};

use crate::call_handler_method;

py_handler! {
    PyPlugHandler(PlugHandler, DeviceInfoPlugResult),
    py_name = "PlugHandler",
    on_off,
    device_management,
    device_usage = DeviceUsageResult,
}

#[pymethods]
impl PyPlugHandler {
    pub async fn set_timer(
        &self,
        delay_seconds: u32,
        desired_state: PowerState,
    ) -> PyResult<Timer> {
        let delay = Duration::from_secs(delay_seconds.into());
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugHandler::set_timer,
            delay,
            desired_state
        )
    }

    pub async fn get_timer(&self) -> PyResult<Option<Timer>> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), PlugHandler::get_timer)
    }

    pub async fn clear_timer(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(handler.read().await.deref(), PlugHandler::clear_timer)
    }
}
