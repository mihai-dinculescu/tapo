use std::ops::Deref;
use std::time::Duration;

use chrono::{DateTime, NaiveDate, Utc};
use pyo3::prelude::*;
use tapo::PlugEnergyMonitoringHandler;
use tapo::requests::{EnergyDataInterval, PowerDataInterval, ScheduleRule};
use tapo::responses::{
    CurrentPowerResult, DeviceInfoPlugEnergyMonitoringResult, DeviceUsageEnergyMonitoringResult,
    EnergyDataResult, EnergyUsageResult, PowerDataResult, Timer,
};

use crate::call_handler_method;
use crate::requests::{PyEnergyDataInterval, PyPowerDataInterval};

py_handler! {
    PyPlugEnergyMonitoringHandler(PlugEnergyMonitoringHandler, DeviceInfoPlugEnergyMonitoringResult),
    py_name = "PlugEnergyMonitoringHandler",
    on_off,
    device_management,
    device_usage = DeviceUsageEnergyMonitoringResult,
}

#[pymethods]
impl PyPlugEnergyMonitoringHandler {
    pub async fn get_current_power(&self) -> PyResult<CurrentPowerResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::get_current_power,
        )
    }

    pub async fn get_energy_usage(&self) -> PyResult<EnergyUsageResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::get_energy_usage,
        )
    }

    #[pyo3(signature = (interval, start_date, end_date=None))]
    pub async fn get_energy_data(
        &self,
        interval: PyEnergyDataInterval,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
    ) -> PyResult<EnergyDataResult> {
        let interval = match interval {
            PyEnergyDataInterval::Hourly => EnergyDataInterval::Hourly {
                start_date,
                end_date: end_date.unwrap_or(start_date),
            },
            PyEnergyDataInterval::Daily => EnergyDataInterval::Daily { start_date },
            PyEnergyDataInterval::Monthly => EnergyDataInterval::Monthly { start_date },
        };

        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::get_energy_data,
            interval
        )?;
        Ok(result)
    }

    pub async fn get_power_data(
        &self,
        interval: PyPowerDataInterval,
        start_date_time: DateTime<Utc>,
        end_date_time: DateTime<Utc>,
    ) -> PyResult<PowerDataResult> {
        let interval = match interval {
            PyPowerDataInterval::Every5Minutes => PowerDataInterval::Every5Minutes {
                start_date_time,
                end_date_time,
            },
            PyPowerDataInterval::Hourly => PowerDataInterval::Hourly {
                start_date_time,
                end_date_time,
            },
        };

        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::get_power_data,
            interval
        )?;
        Ok(result)
    }

    pub async fn set_timer(&self, delay_seconds: u32, turn_on: bool) -> PyResult<Timer> {
        let delay = Duration::from_secs(delay_seconds.into());
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::set_timer,
            delay,
            turn_on
        )
    }

    pub async fn get_timer(&self) -> PyResult<Option<Timer>> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::get_timer
        )
    }

    pub async fn clear_timer(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::clear_timer
        )
    }

    pub async fn add_schedule_rule(&self, rule: ScheduleRule) -> PyResult<ScheduleRule> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::add_schedule_rule,
            rule
        )
    }

    pub async fn edit_schedule_rule(&self, rule: ScheduleRule) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::edit_schedule_rule,
            rule
        )
    }

    pub async fn get_schedule_rules(&self) -> PyResult<Vec<ScheduleRule>> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::get_schedule_rules
        )
    }

    pub async fn remove_schedule_rule(&self, id: String) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::remove_schedule_rule,
            id
        )
    }

    pub async fn remove_all_schedule_rules(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::remove_all_schedule_rules
        )
    }
}
