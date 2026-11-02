use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// The firmware update state as the device sends it. When an H100 picks up a
/// child device's pending update again, it briefly reports the child as
/// downloading without `upgrade_time` and `reboot_time`, as seen for a T100
/// and a T110.
#[derive(Debug, Deserialize)]
pub(crate) struct FirmwareUpdateStateResultRaw {
    status: i32,
    download_progress: u8,
    upgrade_time: Option<u16>,
    reboot_time: Option<u16>,
}

impl TapoResponseExt for FirmwareUpdateStateResultRaw {}

impl TryFrom<FirmwareUpdateStateResultRaw> for FirmwareUpdateStateResult {
    type Error = anyhow::Error;

    fn try_from(raw: FirmwareUpdateStateResultRaw) -> Result<Self, Self::Error> {
        Ok(match raw.status {
            0 => Self::Idle,
            1 | 2 => Self::Downloading {
                progress: raw.download_progress,
            },
            3 => {
                let (Some(install_time_s), Some(reboot_time_s)) =
                    (raw.upgrade_time, raw.reboot_time)
                else {
                    anyhow::bail!(
                        "the device reported a downloaded update without its install and reboot times"
                    );
                };

                Self::Downloaded {
                    install_time_s,
                    reboot_time_s,
                }
            }
            4 => Self::AwaitingTransfer,
            code if code < 0 => Self::Failed { code },
            code => Self::Unknown { code },
        })
    }
}

/// The state of a firmware update on the device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FirmwareUpdateStateResult {
    /// No firmware update is in progress.
    Idle,
    /// The device is downloading the update.
    Downloading {
        /// Download progress in percent (0-100).
        progress: u8,
    },
    /// The update is downloaded and installs next.
    ///
    /// The device installs the update and reboots, which takes about
    /// `install_time_s` plus `reboot_time_s`, and does not answer while it
    /// reboots. An update that the device downloaded by itself for an automatic
    /// update stays in this state until the device installs it. For a hub child
    /// device, the hub can report this state when the child device already runs
    /// the new firmware.
    Downloaded {
        /// Time the device takes to install the update, in seconds.
        install_time_s: u16,
        /// Time the device takes to reboot after installing the update, in seconds.
        reboot_time_s: u16,
    },
    /// The hub has downloaded the update for this child device and has not
    /// sent it to the child device yet. The hub sends it once it can reach the
    /// child device, so a child device that is offline, for example with a
    /// flat battery, stays in this state.
    AwaitingTransfer,
    /// The update failed.
    ///
    /// The Tapo app reads `-1001` as a failed download, `-1002` as a failed
    /// installation check, `-1003` as a failed transfer from the hub to the child
    /// device, and `-1004` as a battery too low to update.
    Failed {
        /// Status code that the device reported.
        code: i32,
    },
    /// The device reported a status that the library does not recognize.
    Unknown {
        /// Status code that the device reported.
        code: i32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(status: i32, download_progress: u8) -> FirmwareUpdateStateResult {
        let json = serde_json::json!({
            "auto_upgrade": false,
            "download_progress": download_progress,
            "reboot_time": 5,
            "status": status,
            "upgrade_time": 10
        });

        let raw: FirmwareUpdateStateResultRaw = serde_json::from_value(json).unwrap();
        raw.try_into().unwrap()
    }

    #[test]
    fn reads_an_idle_device() {
        assert_eq!(state(0, 0), FirmwareUpdateStateResult::Idle);
    }

    #[test]
    fn reads_a_download_in_progress() {
        assert_eq!(
            state(1, 0),
            FirmwareUpdateStateResult::Downloading { progress: 0 }
        );
        assert_eq!(
            state(2, 42),
            FirmwareUpdateStateResult::Downloading { progress: 42 }
        );
    }

    #[test]
    fn reads_a_downloaded_update() {
        assert_eq!(
            state(3, 100),
            FirmwareUpdateStateResult::Downloaded {
                install_time_s: 10,
                reboot_time_s: 5,
            }
        );
    }

    #[test]
    fn reads_an_update_awaiting_transfer() {
        assert_eq!(state(4, 30), FirmwareUpdateStateResult::AwaitingTransfer);
    }

    #[test]
    fn reads_a_failed_update() {
        assert_eq!(
            state(-1001, 0),
            FirmwareUpdateStateResult::Failed { code: -1001 }
        );
    }

    #[test]
    fn reads_an_unknown_status() {
        assert_eq!(state(6, 0), FirmwareUpdateStateResult::Unknown { code: 6 });
    }

    #[test]
    fn reads_a_child_device_without_auto_upgrade() {
        let json = serde_json::json!({
            "cloud_cache_seconds": 1,
            "download_progress": 0,
            "reboot_time": 5,
            "status": 0,
            "upgrade_time": 5
        });

        let raw: FirmwareUpdateStateResultRaw = serde_json::from_value(json).unwrap();

        assert_eq!(
            FirmwareUpdateStateResult::try_from(raw).unwrap(),
            FirmwareUpdateStateResult::Idle
        );
    }

    // As an H100 reported a T100 and a T110 when it picked up their pending update again.
    #[test]
    fn reads_a_child_device_download_without_times() {
        let json = serde_json::json!({
            "cloud_cache_seconds": 1,
            "download_progress": 0,
            "status": 2
        });

        let raw: FirmwareUpdateStateResultRaw = serde_json::from_value(json).unwrap();

        assert_eq!(
            FirmwareUpdateStateResult::try_from(raw).unwrap(),
            FirmwareUpdateStateResult::Downloading { progress: 0 }
        );
    }

    #[test]
    fn rejects_a_downloaded_update_without_times() {
        let json = serde_json::json!({
            "download_progress": 100,
            "status": 3
        });

        let raw: FirmwareUpdateStateResultRaw = serde_json::from_value(json).unwrap();

        assert!(FirmwareUpdateStateResult::try_from(raw).is_err());
    }
}
