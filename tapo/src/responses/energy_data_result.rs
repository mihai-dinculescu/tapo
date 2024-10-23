use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;
use crate::utils::der_tapo_datetime_format;

/// Energy data for the requested [`crate::requests::EnergyDataInterval`].
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
pub struct EnergyDataResult {
    /// Local time of the device.
    #[serde(deserialize_with = "der_tapo_datetime_format")]
    pub local_time: NaiveDateTime,
    /// Energy data for the given `interval` in Watt Hours (Wh).
    pub data: Vec<u64>,
    /// Start timestamp of the interval in milliseconds. This value is provided
    /// in the `get_energy_data` request and is passed through. Note that
    /// it may not align with the returned data if the method is used
    /// beyond its specified capabilities.
    pub start_timestamp: u64,
    /// End timestamp of the interval in milliseconds. This value is provided
    /// in the `get_energy_data` request and is passed through. Note that
    /// it may not align with the returned data for intervals other than hourly.
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
