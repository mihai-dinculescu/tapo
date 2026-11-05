/// Wi-Fi setup of a new or factory-reset device Example
///
/// Join the device's setup network (e.g. `Tapo_Plug_999F`) first. The device
/// takes TAPO_USERNAME and TAPO_PASSWORD as its login from then on; they need
/// not be those of a TP-Link account.
use std::env;

use log::info;
use tapo::ApiClient;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let [
        tapo_username,
        tapo_password,
        wifi_ssid,
        wifi_password,
        timezone,
    ] = common::require_env_vars([
        "TAPO_USERNAME",
        "TAPO_PASSWORD",
        "WIFI_SSID",
        "WIFI_PASSWORD",
        "TIMEZONE",
    ])?;
    let ip_address = env::var("IP_ADDRESS").unwrap_or_else(|_| "192.168.0.1".to_string());
    let timezone: chrono_tz::Tz = timezone.parse()?;

    let setup = ApiClient::new(tapo_username, tapo_password)
        .setup_over_wifi(ip_address)
        .await?;

    let device_info = setup.get_device_info().await?;
    info!("Device info: {device_info:?}");

    info!("Scanning for Wi-Fi networks...");
    let networks = setup.scan_wifi().await?;
    for network in &networks {
        info!("Network: {network:?}");
    }

    let network = networks
        .iter()
        .find(|network| network.ssid == wifi_ssid)
        .ok_or_else(|| format!("The device does not see the Wi-Fi network {wifi_ssid}"))?;

    info!("Joining {wifi_ssid}...");
    let result = setup
        .join_wifi(&wifi_ssid, wifi_password, network.security, timezone)
        .await?;
    info!("Join result: {result:?}. Reconnect to {wifi_ssid} and discover the device there.");

    Ok(())
}
