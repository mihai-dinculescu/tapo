use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// How far a device that was set up over Bluetooth has got with joining a
/// Wi-Fi network: `idle`, `connecting`, `config_ip` or `connected`.
#[derive(Debug, Deserialize)]
pub(crate) struct WifiConnStatusResultRaw {
    pub status: String,
    /// Why the last attempt failed, in the device's own terms.
    #[serde(default)]
    pub last_fail_reason: Option<serde_json::Value>,
}

impl TapoResponseExt for WifiConnStatusResultRaw {}

/// How far the device has confirmed joining a Wi-Fi network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, eq, hash, frozen)
)]
pub enum WifiJoinResult {
    /// The device took the network's details and left its setup network
    /// without saying whether it could join. Find it on the network that it
    /// was given, for example with
    /// [`ApiClient::discover_devices`](crate::ApiClient::discover_devices).
    Sent,
    /// The device said that it joined the network, which it does when it is
    /// set up over Bluetooth. Find it on that network, for example with
    /// [`ApiClient::discover_devices`](crate::ApiClient::discover_devices).
    Connected,
}
