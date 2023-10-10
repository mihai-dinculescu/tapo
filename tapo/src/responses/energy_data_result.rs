use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;
use crate::tapo_date_format::der_tapo_datetime_format;

/// Energy data for the requested [`crate::requests::EnergyDataInterval`].
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct EnergyDataResult {
    /// Local time of the device.
    #[serde(deserialize_with = "der_tapo_datetime_format")]
    pub local_time: NaiveDateTime,
    /// Energy data for the given `interval` in watts (W).
    pub data: Vec<u64>,
    /// Interval start timestamp in milliseconds.
    pub start_timestamp: u64,
    /// Interval end timestamp in milliseconds.
    pub end_timestamp: u64,
    /// Interval in minutes.
    pub interval: u64,
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
