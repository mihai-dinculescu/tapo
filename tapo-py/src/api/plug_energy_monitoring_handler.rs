use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use chrono::{DateTime, NaiveDate, Utc};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::requests::{EnergyDataInterval, PowerDataInterval};
use tapo::responses::{
    CurrentPowerResult, DeviceInfoPlugEnergyMonitoringResult, DeviceUsageEnergyMonitoringResult,
    EnergyDataResult, EnergyUsageResult, PowerDataResult,
};
use tapo::{DeviceManagementExt as _, PlugEnergyMonitoringHandler};
use tokio::sync::RwLock;

use crate::call_handler_method;
use crate::requests::{PyEnergyDataInterval, PyPowerDataInterval};

#[derive(Clone)]
#[pyclass(name = "PlugEnergyMonitoringHandler")]
pub struct PyPlugEnergyMonitoringHandler {
    inner: Arc<RwLock<PlugEnergyMonitoringHandler>>,
}

impl PyPlugEnergyMonitoringHandler {
    pub fn new(handler: PlugEnergyMonitoringHandler) -> Self {
        Self {
            inner: Arc::new(RwLock::new(handler)),
        }
    }
}

#[pymethods]
impl PyPlugEnergyMonitoringHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.write().await.deref_mut(),
            PlugEnergyMonitoringHandler::refresh_session,
            discard_result
        )
    }

    pub async fn on(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::on
        )
    }

    pub async fn off(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::off
        )
    }

    pub async fn device_reboot(&self, delay_s: u16) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::device_reboot,
            delay_s
        )
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::device_reset,
        )
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoPlugEnergyMonitoringResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::get_device_info,
        )
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.inner.clone();
        let result = call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::get_device_info_json,
        )?;
        Python::attach(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_current_power(&self) -> PyResult<CurrentPowerResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::get_current_power,
        )
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageEnergyMonitoringResult> {
        let handler = self.inner.clone();
        call_handler_method!(
            handler.read().await.deref(),
            PlugEnergyMonitoringHandler::get_device_usage,
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
}
