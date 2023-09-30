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
    pub fn to_dict<'a>(&self, py: pyo3::Python<'a>) -> pyo3::PyResult<&'a pyo3::types::PyDict> {
        let serialized = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        if let Some(object) = serialized.as_object() {
            let dict = crate::python::serde_object_to_py_dict(py, object)?;

            Ok(dict)
        } else {
            Ok(pyo3::types::PyDict::new(py))
        }
    }
}
