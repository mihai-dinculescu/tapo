use serde::{Deserialize, Serialize};

/// The backup Wi-Fi network of a camera paired to a camera hub.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum BackupWifi {
    /// The backup network is chosen automatically.
    Auto,
    /// No backup network is configured.
    None,
    /// A specific configured SSID.
    Ssid(String),
}

impl From<String> for BackupWifi {
    fn from(value: String) -> Self {
        match value.as_str() {
            "auto" => BackupWifi::Auto,
            "" => BackupWifi::None,
            _ => BackupWifi::Ssid(value),
        }
    }
}

impl From<BackupWifi> for String {
    fn from(value: BackupWifi) -> Self {
        match value {
            BackupWifi::Auto => "auto".into(),
            BackupWifi::None => String::new(),
            BackupWifi::Ssid(ssid) => ssid,
        }
    }
}
