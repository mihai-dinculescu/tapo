use chrono_tz::Tz;

use crate::error::Error;
use crate::responses::{DeviceInfoBasicResult, WifiJoinResult, WifiNetwork, WifiSecurity};

use super::ApiClient;

/// Handler for setting up a new or factory-reset device. It finds the
/// Wi-Fi networks that the device sees and puts the device on one of them,
/// without the Tapo app.
///
/// A device such as the [P110](https://www.tapo.com/en/search/?q=P110)
/// broadcasts a setup network, named after the device, as in
/// `Tapo_Plug_999F`. Join it, and the device is at `192.168.0.1`. Build the
/// handler with [`ApiClient::setup_over_wifi`].
///
/// A device such as the [P110M](https://www.tapo.com/en/search/?q=P110M)
/// is set up over Bluetooth instead. Find it with
/// `ApiClient::discover_bluetooth_setup_devices` and build the handler with
/// `ApiClient::setup_over_bluetooth` (both require the `bluetooth` feature).
///
/// The device takes the [`ApiClient`] credentials as its login once it
/// joins a Wi-Fi network, and only those from then on. They need not be
/// those of a TP-Link account, because the device does not check them with
/// TP-Link.
#[derive(Debug)]
pub struct SetupHandler {
    client: ApiClient,
    address: Option<(String, u16)>,
}

impl SetupHandler {
    pub(crate) fn new(client: ApiClient, address: Option<(String, u16)>) -> Self {
        Self { client, address }
    }

    /// Returns the IP address and the port that the handler reaches the
    /// device at, on its setup network, or `None` over Bluetooth.
    pub fn address(&self) -> Option<(String, u16)> {
        self.address.clone()
    }

    /// Returns *device info* as [`DeviceInfoBasicResult`], which names the
    /// device's model.
    /// It is not guaranteed to contain all the properties returned from the Tapo API.
    /// If the deserialization fails, or if a property that you care about it's not present,
    #[cfg_attr(
        feature = "debug",
        doc = "try [`SetupHandler::get_device_info_json`] (requires the `debug` feature)."
    )]
    #[cfg_attr(
        not(feature = "debug"),
        doc = "try `SetupHandler::get_device_info_json` (requires the `debug` feature)."
    )]
    pub async fn get_device_info(&self) -> Result<DeviceInfoBasicResult, Error> {
        self.client.get_device_info().await
    }

    /// Returns *device info* as [`serde_json::Value`].
    /// It contains all the properties returned from the Tapo API.
    #[cfg(feature = "debug")]
    pub async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        self.client.get_device_info().await
    }

    /// Returns the Wi-Fi networks that the device sees.
    pub async fn scan_wifi(&self) -> Result<Vec<WifiNetwork>, Error> {
        self.client.scan_wifi().await
    }

    /// Puts the device on the Wi-Fi network `ssid`, with the [`ApiClient`]
    /// credentials as its login from then on.
    ///
    /// Over its setup network, the device leaves without answering, so this
    /// returns [`WifiJoinResult::Sent`] once the connection to the device
    /// drops or times out, which can take as long as the
    /// [`ApiClient::with_timeout`] timeout. Reconnect the host to its own
    /// network to reach the device again.
    ///
    /// Over Bluetooth, the device says whether it joined, so this waits up
    /// to 65 seconds for it and returns [`WifiJoinResult::Connected`].
    ///
    /// # Arguments
    ///
    /// * `ssid` - the name of the network, as in [`WifiNetwork::ssid`]
    /// * `password` - the password of the network, empty for an open network
    /// * `security` - the security of the network, as in [`WifiNetwork::security`]
    /// * `timezone` - the time zone that the device keeps its time in, for
    ///   example for its schedules. The device takes the time itself from
    ///   the host's clock.
    ///
    /// # Errors
    ///
    /// Returns an error if `ssid` is empty, if it holds U+FFFD, which
    /// [`SetupHandler::scan_wifi`] puts in place of a name that is not
    /// UTF-8, if `security` is [`WifiSecurity::Unknown`], if `password` is
    /// empty for a network that is not open, if the device turns the request
    /// down, or, over Bluetooth, if the device does not join the network.
    pub async fn join_wifi(
        &self,
        ssid: impl AsRef<str>,
        password: impl AsRef<str>,
        security: WifiSecurity,
        timezone: Tz,
    ) -> Result<WifiJoinResult, Error> {
        self.client
            .join_wifi(ssid.as_ref(), password.as_ref(), security, timezone)
            .await
    }
}
