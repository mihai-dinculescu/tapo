/// Bluetooth setup of a new or factory-reset device Example
///
/// Requires the `bluetooth` feature. The device takes TAPO_USERNAME and
/// TAPO_PASSWORD as its login from then on; they need not be those of a
/// TP-Link account.
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
    let timezone: chrono_tz::Tz = timezone.parse()?;

    info!("Scanning for devices to set up over Bluetooth...");
    let devices = ApiClient::discover_bluetooth_setup_devices(10).await?;
    for device in &devices {
        info!("Device: {device:?}");
    }
    let device = devices
        .first()
        .ok_or("No device to set up over Bluetooth was found")?;

    let setup = ApiClient::new(tapo_username, tapo_password)
        .setup_over_bluetooth(device)
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
    info!("Join result: {result:?}. Discover the device on {wifi_ssid}.");

    Ok(())
}
