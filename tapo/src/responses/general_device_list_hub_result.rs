mod ai_camera_support;
mod backup_wifi;

pub use ai_camera_support::*;
pub use backup_wifi::*;

use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// General device list result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GeneralDeviceListHubResultRaw {
    #[serde(default)]
    general_camera_manage: GeneralCameraManageResultRaw,
}

impl GeneralDeviceListHubResultRaw {
    pub fn devices(self) -> Vec<GeneralDeviceHubResult> {
        self.general_camera_manage
            .paired_general_device_list
            .into_devices()
    }
}

impl TapoResponseExt for GeneralDeviceListHubResultRaw {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct GeneralCameraManageResultRaw {
    #[serde(default)]
    paired_general_device_list: PairedGeneralDeviceListRaw,
}

/// The H200 returns the camera array directly under `general_camera_manage`,
/// but the app's beans expect an extra `paired_general_device_list` section
/// wrapper around it. Only the flat shape has been observed on the wire;
/// accept both.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum PairedGeneralDeviceListRaw {
    List(Vec<GeneralDeviceHubResult>),
    Section {
        #[serde(default)]
        paired_general_device_list: Vec<GeneralDeviceHubResult>,
    },
}

impl PairedGeneralDeviceListRaw {
    fn into_devices(self) -> Vec<GeneralDeviceHubResult> {
        match self {
            PairedGeneralDeviceListRaw::List(devices) => devices,
            PairedGeneralDeviceListRaw::Section {
                paired_general_device_list,
            } => paired_general_device_list,
        }
    }
}

impl Default for PairedGeneralDeviceListRaw {
    fn default() -> Self {
        PairedGeneralDeviceListRaw::List(Vec::new())
    }
}

/// General device (standalone Wi-Fi camera) paired to a camera hub.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct GeneralDeviceHubResult {
    /// The AI detection types the camera runs itself.
    pub ai_camera_support: AiCameraSupport,
    pub alias: String,
    /// The backup Wi-Fi network.
    pub backup_wifi: BackupWifi,
    pub category: String,
    pub device_id: String,
    pub device_model: String,
    pub device_type: String,
    /// Whether the hub stores this camera's footage.
    pub hub_storage_enabled: bool,
    pub mac: String,
    pub network_mode: String,
    pub parent_device_id: String,
    /// Whether 24h continuous recording is enabled for this camera.
    pub plan_24h_record: bool,
    pub wifi_backup_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_paired_general_device_list_parses_as_empty() {
        let json = r#"{
            "general_camera_manage": {
                "cur_24h_record_dev": 0,
                "current_bound": 0,
                "max_24h_record_dev": 4,
                "max_bound": 4,
                "paired_general_device_list": []
            }
        }"#;

        let parsed: GeneralDeviceListHubResultRaw = serde_json::from_str(json).unwrap();

        assert!(parsed.devices().is_empty());
    }

    #[test]
    fn test_missing_paired_general_device_list_parses_as_empty() {
        let json = r#"{"general_camera_manage": {"current_bound": 0, "max_bound": 4}}"#;

        let parsed: GeneralDeviceListHubResultRaw = serde_json::from_str(json).unwrap();

        assert!(parsed.devices().is_empty());
    }
}
