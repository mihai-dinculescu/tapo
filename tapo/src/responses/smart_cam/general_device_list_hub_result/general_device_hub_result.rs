use serde::{Deserialize, Serialize};

use crate::responses::{AiCameraSupport, BackupWifi, TapoResponseExt};

/// General device list result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GeneralDeviceListHubResultRaw {
    general_camera_manage: GeneralCameraManageResultRaw,
}

impl GeneralDeviceListHubResultRaw {
    pub fn devices(self) -> Vec<GeneralDeviceHubResult> {
        self.general_camera_manage.paired_general_device_list
    }
}

impl TapoResponseExt for GeneralDeviceListHubResultRaw {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeneralCameraManageResultRaw {
    /// H200 firmware 1.6.5 omits its other lists when they are empty, so a
    /// missing list is taken to mean no paired cameras.
    #[serde(default)]
    paired_general_device_list: Vec<GeneralDeviceHubResult>,
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
}
