use std::sync::Arc;

use chrono::NaiveDate;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tapo::requests::EnergyDataInterval;
use tapo::responses::{
    CurrentPowerResult, DeviceInfoPlugResult, DeviceUsageEnergyMonitoringResult, EnergyDataResult,
    EnergyUsageResult,
};
use tapo::PlugEnergyMonitoringHandler;
use tokio::sync::Mutex;

use crate::errors::ErrorWrapper;

#[derive(Clone)]
#[pyclass(name = "EnergyDataInterval")]
pub enum PyEnergyDataInterval {
    Hourly,
    Daily,
    Monthly,
}

#[derive(Clone)]
#[pyclass(name = "PlugEnergyMonitoringHandler")]
pub struct PyPlugEnergyMonitoringHandler {
    handler: Arc<Mutex<PlugEnergyMonitoringHandler>>,
}

impl PyPlugEnergyMonitoringHandler {
    pub fn new(handler: PlugEnergyMonitoringHandler) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
        }
    }
}

#[pymethods]
impl PyPlugEnergyMonitoringHandler {
    pub async fn refresh_session(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        handler
            .lock()
            .await
            .refresh_session()
            .await
            .map_err(ErrorWrapper)?;
        Ok(())
    }

    pub async fn on(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        handler.lock().await.on().await.map_err(ErrorWrapper)?;
        Ok(())
    }

    pub async fn off(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        handler.lock().await.off().await.map_err(ErrorWrapper)?;
        Ok(())
    }

    pub async fn device_reset(&self) -> PyResult<()> {
        let handler = self.handler.clone();
        handler
            .lock()
            .await
            .device_reset()
            .await
            .map_err(ErrorWrapper)?;
        Ok(())
    }

    pub async fn get_device_info(&self) -> PyResult<DeviceInfoPlugResult> {
        let handler = self.handler.clone();
        let result = handler
            .lock()
            .await
            .get_device_info()
            .await
            .map_err(ErrorWrapper)?;
        Ok(result)
    }

    pub async fn get_device_info_json(&self) -> PyResult<Py<PyDict>> {
        let handler = self.handler.clone();
        let result = handler
            .lock()
            .await
            .get_device_info_json()
            .await
            .map_err(ErrorWrapper)?;
        Python::with_gil(|py| tapo::python::serde_object_to_py_dict(py, &result))
    }

    pub async fn get_device_usage(&self) -> PyResult<DeviceUsageEnergyMonitoringResult> {
        let handler = self.handler.clone();
        let result = handler
            .lock()
            .await
            .get_device_usage()
            .await
            .map_err(ErrorWrapper)?;
        Ok(result)
    }

    pub async fn get_current_power(&self) -> PyResult<CurrentPowerResult> {
        let handler = self.handler.clone();
        let result = handler
            .lock()
            .await
            .get_current_power()
            .await
            .map_err(ErrorWrapper)?;
        Ok(result)
    }

    pub async fn get_energy_usage(&self) -> PyResult<EnergyUsageResult> {
        let handler = self.handler.clone();
        let result = handler
            .lock()
            .await
            .get_energy_usage()
            .await
            .map_err(ErrorWrapper)?;
        Ok(result)
    }

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

        let handler = self.handler.clone();
        let result = handler
            .lock()
            .await
            .get_energy_data(interval)
            .await
            .map_err(ErrorWrapper)?;
        Ok(result)
    }
}
