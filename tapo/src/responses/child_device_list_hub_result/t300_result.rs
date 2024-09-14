use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DecodableResultExt, Status, TapoResponseExt};

/// Water leak status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, eq, eq_int))]
#[allow(missing_docs)]
pub enum WaterLeakStatus {
    Normal,
    WaterDry,
    WaterLeak,
}

/// Device info of Tapo T300 water sensor.
///
/// Specific properties: `in_alarm`, `water_leak_status`, `report_interval`,
/// `last_onboarding_timestamp`, `status_follow_edge`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub struct T300Result {
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
    pub in_alarm: bool,
    #[serde(rename = "lastOnboardingTimestamp")]
    pub last_onboarding_timestamp: u64,
    /// The time in seconds between each report.
    pub report_interval: u32,
    pub status_follow_edge: bool,
    pub water_leak_status: WaterLeakStatus,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl T300Result {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}

impl TapoResponseExt for T300Result {}

impl DecodableResultExt for T300Result {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}

/// T300 Log.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event")]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub enum T300Log {
    WaterDry { id: u64, timestamp: u64 },
    WaterLeak { id: u64, timestamp: u64 },
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl T300Log {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}
