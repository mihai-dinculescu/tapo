use std::ops::Deref;
use std::time::Duration;

use pyo3::prelude::*;
use tapo::PlugHandler;
use tapo::requests::ScheduleRule;
use tapo::responses::{DeviceInfoPlugResult, DeviceUsageResult, Timer};

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
    pub async fn set_timer(&self, delay_seconds: u32, turn_on: bool) -> PyResult<Timer> {
        let delay = Duration::from_secs(delay_seconds.into());
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugHandler::set_timer,
            delay,
            turn_on
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

    pub async fn add_schedule_rule(&self, rule: ScheduleRule) -> PyResult<ScheduleRule> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugHandler::add_schedule_rule,
            rule
        )
    }

    pub async fn edit_schedule_rule(&self, rule: ScheduleRule) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugHandler::edit_schedule_rule,
            rule
        )
    }

    pub async fn get_schedule_rules(&self) -> PyResult<Vec<ScheduleRule>> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugHandler::get_schedule_rules
        )
    }

    pub async fn remove_schedule_rule(&self, id: String) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugHandler::remove_schedule_rule,
            id
        )
    }

    pub async fn remove_all_schedule_rules(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugHandler::remove_all_schedule_rules
        )
    }
}
