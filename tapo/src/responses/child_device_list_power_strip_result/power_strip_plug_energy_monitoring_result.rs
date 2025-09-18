use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{
    ChargingStatus, DecodableResultExt, DefaultPlugState, OvercurrentStatus, OverheatStatus,
    PowerProtectionStatus, TapoResponseExt, decode_value,
};

use super::AutoOffStatus;

/// Power Strip child device list result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChildDeviceListPowerStripEnergyMonitoringResult {
    /// Power Strip child devices
    #[serde(rename = "child_device_list")]
    pub plugs: Vec<PowerStripPlugEnergyMonitoringResult>,
}

impl DecodableResultExt for ChildDeviceListPowerStripEnergyMonitoringResult {
    fn decode(self) -> Result<Self, Error> {
        Ok(ChildDeviceListPowerStripEnergyMonitoringResult {
            plugs: self
                .plugs
                .into_iter()
                .map(|d| d.decode())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TapoResponseExt for ChildDeviceListPowerStripEnergyMonitoringResult {}

/// P304M and P316M power strip child plugs.
///
/// Specific properties: `auto_off_remain_time`, `auto_off_status`,
/// `bind_count`, `default_states`, `charging_status`, `is_usb`,
/// `overcurrent_status`, `overheat_status`, `position`,
/// `power_protection_status`, `slot_number`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub struct PowerStripPlugEnergyMonitoringResult {
    pub auto_off_remain_time: u64,
    pub auto_off_status: AutoOffStatus,
    pub avatar: String,
    pub bind_count: u8,
    pub category: String,
    pub default_states: DefaultPlugState,
    pub charging_status: ChargingStatus,
    pub device_id: String,
    pub device_on: bool,
    pub fw_id: String,
    pub fw_ver: String,
    pub has_set_location_info: bool,
    pub hw_id: String,
    pub hw_ver: String,
    pub is_usb: bool,
    pub latitude: Option<i64>,
    pub longitude: Option<i64>,
    pub mac: String,
    pub model: String,
    pub nickname: String,
    pub oem_id: String,
    /// The time in seconds this device has been ON since the last state change (On/Off).
    pub on_time: u64,
    pub original_device_id: String,
    pub overcurrent_status: OvercurrentStatus,
    pub overheat_status: Option<OverheatStatus>,
    pub position: u8,
    pub power_protection_status: PowerProtectionStatus,
    pub region: Option<String>,
    pub slot_number: u8,
    pub status_follow_edge: bool,
    pub r#type: String,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl PowerStripPlugEnergyMonitoringResult {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}

impl TapoResponseExt for PowerStripPlugEnergyMonitoringResult {}

impl DecodableResultExt for PowerStripPlugEnergyMonitoringResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}
