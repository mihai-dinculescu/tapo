use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DecodableResultExt, Status, TapoResponseExt};

/// Temperature unit for KE100 devices.
/// Currently *Celsius* is the only unit supported by KE100.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, eq, eq_int))]
#[allow(missing_docs)]
pub enum TemperatureUnitKE100 {
    Celsius,
}

/// Device info of Tapo KE100 thermostatic radiator valve (TRV).
///
/// Specific properties: `temperature_unit`, `current_temperature`, `target_temperature`,
/// `min_control_temperature`, `max_control_temperature`, `temperature_offset`,
/// `child_protection_on`, `frost_protection_on`, `location`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub struct KE100Result {
    // Common properties to all Hub child devices.
    pub at_low_battery: bool,
    pub avatar: String,
    pub bind_count: u32,
    pub category: String,
    pub device_id: String,
    pub fw_ver: String,
    pub hw_id: String,
    pub hw_ver: String,
    pub jamming_rssi: i16,
    pub jamming_signal_level: u8,
    pub mac: String,
    pub nickname: String,
    pub oem_id: String,
    pub parent_device_id: String,
    pub region: String,
    pub rssi: i16,
    pub signal_level: u8,
    pub specs: String,
    pub status: Status,
    pub r#type: String,
    // Specific properties to this device.
    #[serde(rename = "child_protection")]
    pub child_protection_on: bool,
    #[serde(rename = "current_temp")]
    pub current_temperature: f32,
    pub frost_protection_on: bool,
    pub location: String,
    #[serde(rename = "max_control_temp")]
    pub max_control_temperature: u8,
    #[serde(rename = "min_control_temp")]
    pub min_control_temperature: u8,
    #[serde(rename = "target_temp")]
    pub target_temperature: f32,
    #[serde(rename = "temp_offset")]
    pub temperature_offset: i8,
    #[serde(rename = "temp_unit")]
    pub temperature_unit: TemperatureUnitKE100,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl KE100Result {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}

impl TapoResponseExt for KE100Result {}

impl DecodableResultExt for KE100Result {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}
