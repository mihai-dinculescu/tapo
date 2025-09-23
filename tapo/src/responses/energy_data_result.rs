use anyhow::Context as _;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone as _, Utc};
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;
use crate::utils::der_tapo_datetime_format;

/// Energy data result for the requested [`crate::requests::PowerDataInterval`].
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct EnergyDataResult {
    /// Local time of the device.
    pub local_time: NaiveDateTime,
    /// Start date and time of this result in UTC.
    /// This value is provided in the `get_energy_data` request and is passed through.
    /// Note that it may not align with the returned data if the method is used beyond its specified capabilities.
    pub start_date_time: DateTime<Utc>,
    /// List of energy data entries.
    pub entries: Vec<EnergyDataIntervalResult>,
    /// Interval length in minutes.
    pub interval_length: u64,
}

impl TapoResponseExt for EnergyDataResult {}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl EnergyDataResult {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}

/// Energy data result for a specific interval.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct EnergyDataIntervalResult {
    /// Start date and time of this interval in UTC.
    pub start_date_time: DateTime<Utc>,
    /// Energy in Watt Hours (Wh).
    pub energy: u64,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl EnergyDataIntervalResult {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct EnergyDataResultRaw {
    #[serde(deserialize_with = "der_tapo_datetime_format")]
    pub local_time: NaiveDateTime,
    pub data: Vec<u64>,
    pub start_timestamp: i64,
    pub interval: u64,
}

impl TapoResponseExt for EnergyDataResultRaw {}

impl TryInto<EnergyDataResult> for EnergyDataResultRaw {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<EnergyDataResult, Self::Error> {
        let mut entries = Vec::with_capacity(self.data.len());

        let mut local_date_time = Local
            .timestamp_opt(self.start_timestamp, 0)
            .single()
            .context("Failed to map start_timestamp to local time")?;
        let start_date_time = local_date_time.to_utc();

        for energy in self.data {
            entries.push(EnergyDataIntervalResult {
                start_date_time: local_date_time.to_utc(),
                energy,
            });
            local_date_time = match self.interval {
                60 => Ok(local_date_time + chrono::Duration::hours(1)),
                1440 => Ok(local_date_time + chrono::Duration::days(1)),
                43200 => Ok(local_date_time + chrono::Months::new(1)),
                _ => Err(anyhow::anyhow!(
                    "Unsupported interval duration: {} minutes",
                    self.interval
                )),
            }?;
        }

        Ok(EnergyDataResult {
            local_time: self.local_time,
            start_date_time,
            entries,
            interval_length: self.interval,
        })
    }
}
