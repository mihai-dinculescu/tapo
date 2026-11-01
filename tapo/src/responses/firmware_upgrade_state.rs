use serde::{Deserialize, Serialize};

use super::TapoResponseExt;

/// Contains the progress of a firmware upgrade running on the device.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct FirmwareUpgradeState {
    /// Upgrade status reported by the device; `-1` means idle or failed.
    pub status: Option<i32>,
    /// Download progress in percent, from 0 to 100.
    pub download_progress: Option<u8>,
    /// Seconds until the device reboots.
    pub reboot_time: Option<u32>,
    /// Seconds the upgrade takes.
    pub upgrade_time: Option<u32>,
    /// Whether the device upgrades itself automatically.
    pub auto_upgrade: Option<bool>,
}
impl TapoResponseExt for FirmwareUpgradeState {}

#[cfg(feature = "python")]
crate::impl_to_dict!(FirmwareUpgradeState);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn firmware_upgrade_state_reads_progress_and_times() {
        let state: FirmwareUpgradeState = serde_json::from_value(json!({
            "status": 2,
            "download_progress": 100,
            "reboot_time": 5,
            "upgrade_time": 30,
            "auto_upgrade": false
        }))
        .unwrap();

        assert_eq!(state.status, Some(2));
        assert_eq!(state.download_progress, Some(100));
        assert_eq!(state.reboot_time, Some(5));
        assert_eq!(state.upgrade_time, Some(30));
        assert_eq!(state.auto_upgrade, Some(false));
    }

    #[test]
    fn firmware_upgrade_state_reads_a_negative_status() {
        let state: FirmwareUpgradeState = serde_json::from_value(json!({ "status": -1 })).unwrap();

        assert_eq!(state.status, Some(-1));
    }

    #[test]
    fn firmware_upgrade_state_ignores_unknown_fields() {
        let state: FirmwareUpgradeState =
            serde_json::from_value(json!({ "status": 0, "vendor_extra": { "a": 1 } })).unwrap();

        assert_eq!(state.status, Some(0));
    }
}
