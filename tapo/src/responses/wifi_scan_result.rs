use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// One page of the Wi-Fi networks that a device in setup mode sees. `sum`
/// is the number of networks over all pages.
#[derive(Debug, Deserialize)]
pub(crate) struct WifiScanResultRaw {
    pub ap_list: Vec<WifiNetworkRaw>,
    pub sum: usize,
}

impl TapoResponseExt for WifiScanResultRaw {}

/// A network as the device sends it.
#[derive(Debug, Deserialize)]
pub(crate) struct WifiNetworkRaw {
    ssid: String,
    key_type: WifiSecurity,
    signal_level: u8,
    bssid: String,
    channel: u8,
}

impl TryFrom<WifiNetworkRaw> for WifiNetwork {
    type Error = anyhow::Error;

    fn try_from(raw: WifiNetworkRaw) -> Result<Self, Self::Error> {
        let ssid = general_purpose::STANDARD.decode(&raw.ssid)?;

        Ok(Self {
            // An SSID is any 32 bytes, not necessarily UTF-8, and one that
            // is not should not fail the whole scan.
            ssid: String::from_utf8_lossy(&ssid).into_owned(),
            security: raw.key_type,
            signal_level: raw.signal_level,
            bssid: raw.bssid,
            channel: raw.channel,
        })
    }
}

/// A Wi-Fi network that the device sees.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct WifiNetwork {
    /// The network's name.
    pub ssid: String,
    /// The security of the network. Pass it to `join_wifi` to join this network.
    pub security: WifiSecurity,
    /// The signal strength as the device rates it, higher being stronger.
    pub signal_level: u8,
    /// The MAC address of the access point, as upper-case hex without separators.
    pub bssid: String,
    /// The Wi-Fi channel.
    pub channel: u8,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(WifiNetwork);

/// The security of a Wi-Fi network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, eq, hash, frozen)
)]
#[non_exhaustive]
pub enum WifiSecurity {
    /// An open network, without a password.
    None,
    /// WEP.
    Wep,
    /// WPA Personal.
    WpaPsk,
    /// WPA2 Personal.
    Wpa2Psk,
    /// WPA3 Personal.
    Wpa3Sae,
    /// A security type that the library does not know. A network with it
    /// cannot be joined.
    #[serde(other)]
    Unknown,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl WifiSecurity {
    // `None` is a keyword in Python, so `WifiSecurity.None` is a syntax error.
    // A `#[pyo3(name)]` on the variant cannot be written behind `cfg_attr`, so the
    // variant is re-exported as `NONE` instead, and `tapo-py` removes the generated
    // `None` attribute. `__repr__` uses the same name, since the generated one would
    // print the Rust variant name.
    #[classattr]
    #[pyo3(name = "NONE")]
    const PY_NONE: WifiSecurity = WifiSecurity::None;

    fn __repr__(&self) -> String {
        match self {
            Self::None => "WifiSecurity.NONE".to_string(),
            other => format!("WifiSecurity.{other:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_network_has_its_ssid_decoded() {
        let json = r#"{"ssid": "U2ltdWxhdGVkIFdpLUZp", "bssid": "AABBCCDDEE01", "channel": 7, "cipher_type": 2, "key_type": "wpa2_psk", "signal_level": 3}"#;

        let network: WifiNetwork = serde_json::from_str::<WifiNetworkRaw>(json)
            .unwrap()
            .try_into()
            .unwrap();

        assert_eq!(network.ssid, "Simulated Wi-Fi");
        assert_eq!(network.security, WifiSecurity::Wpa2Psk);
        assert_eq!(network.channel, 7);
    }

    #[test]
    fn security_types_are_read_by_their_device_names() {
        for (name, security) in [
            ("none", WifiSecurity::None),
            ("wep", WifiSecurity::Wep),
            ("wpa_psk", WifiSecurity::WpaPsk),
            ("wpa2_psk", WifiSecurity::Wpa2Psk),
            ("wpa3_sae", WifiSecurity::Wpa3Sae),
            ("wpa4_future", WifiSecurity::Unknown),
        ] {
            let parsed: WifiSecurity = serde_json::from_value(serde_json::json!(name)).unwrap();
            assert_eq!(parsed, security, "{name}");
        }
    }
}
