use serde::{Deserialize, Serialize};

/// How far the device has confirmed joining a Wi-Fi network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WifiJoinResult {
    /// The device took the network's details and left its setup network
    /// without saying whether it could join. Find it on the network that it
    /// was given, for example with
    /// [`ApiClient::discover_devices`](crate::ApiClient::discover_devices).
    Sent,
}
