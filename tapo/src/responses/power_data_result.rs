use anyhow::Context as _;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// Power data for the requested [`crate::requests::PowerDataInterval`].
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct PowerDataResult {
    /// Start date and time of this result in UTC.
    pub start_date_time: DateTime<Utc>,
    /// End date and time of this result in UTC.
    pub end_date_time: DateTime<Utc>,
    /// List of power data entries.
    pub entries: Vec<PowerDataIntervalResult>,
    /// Interval length in minutes.
    pub interval_length: u64,
}

impl TapoResponseExt for PowerDataResult {}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl PowerDataResult {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}

/// Power data result for a specific interval.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct PowerDataIntervalResult {
    /// Date and time of this interval in UTC.
    pub date_time: DateTime<Utc>,
    /// Power in Watts (W). `None` if no data is available for this interval.
    pub power: Option<u64>,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl PowerDataIntervalResult {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub(crate) struct PowerDataResultRaw {
    #[serde(deserialize_with = "deserialize_power_data")]
    pub data: Vec<Option<u64>>,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub interval: u64,
}

impl TapoResponseExt for PowerDataResultRaw {}

impl TryInto<PowerDataResult> for PowerDataResultRaw {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<PowerDataResult, Self::Error> {
        let mut data = Vec::with_capacity(self.data.len());

        let interval_duration = match self.interval {
            5 => Ok(chrono::Duration::minutes(5)),
            60 => Ok(chrono::Duration::hours(1)),
            _ => Err(anyhow::anyhow!(
                "Unsupported interval duration: {} minutes",
                self.interval
            )),
        }?;

        let mut date_time = chrono::DateTime::from_timestamp_secs(self.start_timestamp)
            .context("Invalid start timestamp")?;

        for power in self.data {
            data.push(PowerDataIntervalResult { date_time, power });
            date_time += interval_duration;
        }

        Ok(PowerDataResult {
            start_date_time: chrono::DateTime::from_timestamp_secs(self.start_timestamp)
                .context("Invalid start timestamp")?,
            end_date_time: chrono::DateTime::from_timestamp_secs(self.end_timestamp)
                .context("Invalid end timestamp")?,
            entries: data,
            interval_length: self.interval,
        })
    }
}

fn deserialize_power_data<'de, D>(deserializer: D) -> Result<Vec<Option<u64>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let raw = Vec::<serde_json::Value>::deserialize(deserializer)?;
    let mut out = Vec::with_capacity(raw.len());
    for v in raw {
        match v {
            serde_json::Value::Null => out.push(None),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    if i == -1 {
                        out.push(None);
                    } else if i >= 0 {
                        out.push(Some(i as u64));
                    } else {
                        return Err(D::Error::custom(format!(
                            "Negative value {i} not allowed (except -1 sentinel)"
                        )));
                    }
                } else {
                    return Err(D::Error::custom("Number out of i64 range"));
                }
            }
            other => {
                return Err(D::Error::custom(format!(
                    "Unexpected value in power data array: {other}"
                )));
            }
        }
    }
    Ok(out)
}
