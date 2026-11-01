use serde::{Deserialize, Serialize};

use super::TapoResponseExt;

/// Contains the latest firmware the device offers for itself.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct LatestFirmwareResult {
    /// Version string of the available firmware.
    pub fw_ver: Option<String>,
    /// Whether the available firmware differs from the installed one.
    pub need_to_upgrade: Option<bool>,
    /// Release notes of the available firmware.
    pub release_note: Option<String>,
    /// Release date of the available firmware.
    pub fw_release_date: Option<String>,
    /// Size of the firmware image in bytes.
    pub fw_size: Option<u64>,
    /// OEM identifier of the firmware.
    pub oem_id: Option<String>,
    /// Upgrade type reported by the device.
    #[serde(rename = "type")]
    pub upgrade_type: Option<i32>,
}
impl TapoResponseExt for LatestFirmwareResult {}

#[cfg(feature = "python")]
crate::impl_to_dict!(LatestFirmwareResult);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn latest_firmware_reads_the_documented_fields() {
        let result: LatestFirmwareResult = serde_json::from_value(json!({
            "fw_ver": "1.4.9 Build 260101 Rel. 12345",
            "need_to_upgrade": true,
            "release_note": "Bug fixes",
            "fw_release_date": "2026-01-01",
            "fw_size": 1234567,
            "oem_id": "abc123",
            "type": 2
        }))
        .unwrap();

        assert_eq!(
            result.fw_ver.as_deref(),
            Some("1.4.9 Build 260101 Rel. 12345")
        );
        assert_eq!(result.need_to_upgrade, Some(true));
        assert_eq!(result.release_note.as_deref(), Some("Bug fixes"));
        assert_eq!(result.fw_release_date.as_deref(), Some("2026-01-01"));
        assert_eq!(result.fw_size, Some(1_234_567));
        assert_eq!(result.oem_id.as_deref(), Some("abc123"));
        assert_eq!(result.upgrade_type, Some(2));
    }

    #[test]
    fn latest_firmware_accepts_a_sparse_result() {
        let result: LatestFirmwareResult = serde_json::from_value(json!({})).unwrap();

        assert_eq!(result, LatestFirmwareResult::default());
    }

    #[test]
    fn latest_firmware_reads_large_firmware_size() {
        let result: LatestFirmwareResult =
            serde_json::from_value(json!({ "fw_size": 5_000_000_000u64 })).unwrap();

        assert_eq!(result.fw_size, Some(5_000_000_000));
    }
}
