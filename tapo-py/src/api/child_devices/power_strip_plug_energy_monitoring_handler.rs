use std::ops::Deref;

use chrono::NaiveDate;
use chrono_tz::Tz;
use pyo3::prelude::*;
use tapo::PowerStripPlugEnergyMonitoringHandler;
use tapo::requests::{EnergyDataInterval, PowerDataInterval};
use tapo::responses::{
    CurrentPowerResult, DeviceUsageEnergyMonitoringResult, EnergyDataResult, EnergyMeterDataResult,
    EnergyUsageResult, PowerDataResult, PowerStripPlugEnergyMonitoringResult, TimezoneResult,
};

use crate::call_handler_method;
use crate::requests::{PyEnergyDataInterval, PyPowerDataInterval};
use crate::utc_date_time::UtcDateTime;

py_child_handler! {
    PyPowerStripPlugEnergyMonitoringHandler(PowerStripPlugEnergyMonitoringHandler, PowerStripPlugEnergyMonitoringResult),
    py_name = "PowerStripPlugEnergyMonitoringHandler",
    on_off,
}

#[pymethods]
impl PyPowerStripPlugEnergyMonitoringHandler {
    pub async fn get_current_power(&self) -> PyResult<CurrentPowerResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            PowerStripPlugEnergyMonitoringHandler::get_current_power,
        )
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageEnergyMonitoringResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            PowerStripPlugEnergyMonitoringHandler::get_device_usage,
        )
    }

    #[pyo3(signature = (interval, start_date, end_date=None, timezone=None))]
    pub async fn get_energy_data(
        &self,
        interval: PyEnergyDataInterval,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
        timezone: Option<Tz>,
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
            handler.deref(),
            PowerStripPlugEnergyMonitoringHandler::get_energy_data,
            interval,
            timezone
        )?;
        Ok(result)
    }

    pub async fn get_energy_meter_data(&self) -> PyResult<EnergyMeterDataResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            PowerStripPlugEnergyMonitoringHandler::get_energy_meter_data,
        )
    }

    pub async fn get_energy_usage(&self) -> PyResult<EnergyUsageResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            PowerStripPlugEnergyMonitoringHandler::get_energy_usage,
        )
    }

    pub async fn get_power_data(
        &self,
        interval: PyPowerDataInterval,
        start_date_time: UtcDateTime,
        end_date_time: UtcDateTime,
    ) -> PyResult<PowerDataResult> {
        let interval = match interval {
            PyPowerDataInterval::Every5Minutes => PowerDataInterval::Every5Minutes {
                start_date_time: start_date_time.0,
                end_date_time: end_date_time.0,
            },
            PyPowerDataInterval::Hourly => PowerDataInterval::Hourly {
                start_date_time: start_date_time.0,
                end_date_time: end_date_time.0,
            },
        };

        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.deref(),
            PowerStripPlugEnergyMonitoringHandler::get_power_data,
            interval
        )?;
        Ok(result)
    }

    pub async fn get_timezone(&self) -> PyResult<TimezoneResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            PowerStripPlugEnergyMonitoringHandler::get_timezone,
        )
    }
}
