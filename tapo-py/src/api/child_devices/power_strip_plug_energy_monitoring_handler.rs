use std::ops::Deref;

use chrono::{DateTime, NaiveDate, Utc};
use pyo3::prelude::*;
use tapo::PowerStripPlugEnergyMonitoringHandler;
use tapo::requests::{EnergyDataInterval, PowerDataInterval};
use tapo::responses::{
    CurrentPowerResult, DeviceUsageEnergyMonitoringResult, EnergyDataResult, EnergyUsageResult,
    PowerDataResult, PowerStripPlugEnergyMonitoringResult,
};

use crate::call_handler_method;
use crate::requests::{PyEnergyDataInterval, PyPowerDataInterval};

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

    pub async fn get_energy_usage(&self) -> PyResult<EnergyUsageResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.deref(),
            PowerStripPlugEnergyMonitoringHandler::get_energy_usage,
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
            handler.deref(),
            PowerStripPlugEnergyMonitoringHandler::get_energy_data,
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
            handler.deref(),
            PowerStripPlugEnergyMonitoringHandler::get_power_data,
            interval
        )?;
        Ok(result)
    }
}
