use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// The newest firmware as the device sends it. With no update available, it
/// sends the running version as `fw_ver` and an empty `release_date` and
/// `release_note`.
#[derive(Debug, Deserialize)]
pub(crate) struct LatestFirmwareResultRaw {
    need_to_upgrade: bool,
    fw_ver: String,
    release_date: String,
    release_note: String,
}

impl TapoResponseExt for LatestFirmwareResultRaw {}

impl TryFrom<LatestFirmwareResultRaw> for LatestFirmwareResult {
    type Error = anyhow::Error;

    fn try_from(raw: LatestFirmwareResultRaw) -> Result<Self, Self::Error> {
        if !raw.need_to_upgrade {
            return Ok(Self::UpToDate { fw_ver: raw.fw_ver });
        }

        let release_date = NaiveDate::parse_from_str(&raw.release_date, "%Y-%m-%d")
            .map_err(|error| {
                anyhow::anyhow!(
                    "the release date `{}` of an available update is not a YYYY-MM-DD date: {error}",
                    raw.release_date
                )
            })?;

        Ok(Self::UpdateAvailable {
            fw_ver: raw.fw_ver,
            release_date,
            release_notes: raw.release_note,
        })
    }
}

/// The newest firmware that the device can update to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LatestFirmwareResult {
    /// No update is available.
    UpToDate {
        /// Version of the firmware that the device is running.
        fw_ver: String,
    },
    /// A firmware update is available.
    UpdateAvailable {
        /// Version of the update.
        fw_ver: String,
        /// Release date of the update.
        release_date: NaiveDate,
        /// Release notes of the update.
        release_notes: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_an_available_update() {
        let json = serde_json::json!({
            "fw_size": 786432,
            "fw_ver": "2.1.0 Build 260101 Rel.123456",
            "hw_id": "00000000000000000000000000000000",
            "need_to_upgrade": true,
            "oem_id": "11111111111111111111111111111111",
            "release_date": "2026-01-15",
            "release_note": "Modifications and Bug Fixes:\n1. Improved stability.",
            "type": 1
        });

        let raw: LatestFirmwareResultRaw = serde_json::from_value(json).unwrap();
        let result = LatestFirmwareResult::try_from(raw).unwrap();

        let LatestFirmwareResult::UpdateAvailable {
            fw_ver,
            release_date,
            release_notes,
        } = result
        else {
            panic!("expected an available update, got {result:?}");
        };
        assert_eq!(fw_ver, "2.1.0 Build 260101 Rel.123456");
        assert_eq!(Some(release_date), NaiveDate::from_ymd_opt(2026, 1, 15));
        assert_eq!(
            release_notes,
            "Modifications and Bug Fixes:\n1. Improved stability."
        );
    }

    #[test]
    fn deserializes_an_up_to_date_device() {
        let json = serde_json::json!({
            "fw_size": 0,
            "fw_ver": "2.0.0 Build 251201 Rel.654321",
            "hw_id": "",
            "need_to_upgrade": false,
            "oem_id": "",
            "release_date": "",
            "release_note": "",
            "type": 0
        });

        let raw: LatestFirmwareResultRaw = serde_json::from_value(json).unwrap();
        let result = LatestFirmwareResult::try_from(raw).unwrap();

        let LatestFirmwareResult::UpToDate { fw_ver } = result else {
            panic!("expected an up to date device, got {result:?}");
        };
        assert_eq!(fw_ver, "2.0.0 Build 251201 Rel.654321");
    }

    #[test]
    fn deserializes_a_hub_child_update_without_fw_size() {
        let json = serde_json::json!({
            "fw_ver": "1.5.0 Build 251101 Rel.111111",
            "hw_id": "00000000000000000000000000000000",
            "need_to_upgrade": true,
            "oem_id": "11111111111111111111111111111111",
            "release_date": "2025-11-01",
            "release_note": "Fixed some minor bugs.",
            "type": 1
        });

        let raw: LatestFirmwareResultRaw = serde_json::from_value(json).unwrap();
        let result = LatestFirmwareResult::try_from(raw).unwrap();

        assert!(matches!(
            result,
            LatestFirmwareResult::UpdateAvailable { release_date, .. }
                if Some(release_date) == NaiveDate::from_ymd_opt(2025, 11, 1)
        ));
    }

    #[test]
    fn round_trips_through_serde() {
        let result = LatestFirmwareResult::UpdateAvailable {
            fw_ver: "2.1.0 Build 260101 Rel.123456".to_string(),
            release_date: NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
            release_notes: "Improved stability.".to_string(),
        };

        let json = serde_json::to_value(&result).unwrap();
        let read_back: LatestFirmwareResult = serde_json::from_value(json).unwrap();

        assert!(matches!(
            read_back,
            LatestFirmwareResult::UpdateAvailable { ref fw_ver, .. }
                if fw_ver == "2.1.0 Build 260101 Rel.123456"
        ));
    }

    #[test]
    fn rejects_an_available_update_without_a_release_date() {
        let json = serde_json::json!({
            "fw_ver": "1.5.0 Build 251101 Rel.111111",
            "need_to_upgrade": true,
            "release_date": "",
            "release_note": "Fixed some minor bugs."
        });

        let raw: LatestFirmwareResultRaw = serde_json::from_value(json).unwrap();
        let result = LatestFirmwareResult::try_from(raw);

        assert!(result.is_err(), "{result:?}");
    }
}
