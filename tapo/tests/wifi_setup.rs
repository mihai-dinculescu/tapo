//! Runs the Wi-Fi setup end to end against simulated factory-reset P110s on
//! their setup network, built from their setup fixtures in
//! `tests/fixtures/smart/`.

mod simulator;

use std::error::Error;

use tapo::ApiClient;
use tapo::responses::WifiSecurity;

use simulator::SimulatedDevice;
use simulator::assertions::assert_unauthorized;

const P110_KLAP_SETUP_WIFI_FIXTURE: &str =
    "tests/fixtures/smart/P110(UK)_1.0_1.3.1_KLAP_SETUP_WIFI.json";
const P110_TPAP_SETUP_WIFI_FIXTURE: &str =
    "tests/fixtures/smart/P110(UK)_1.0_1.4.8_TPAP_SETUP_WIFI.json";
const P110_KLAP_FIXTURE: &str = "tests/fixtures/smart/P110(UK)_1.0_1.4.8_KLAP.json";
const P110_TPAP_FIXTURE: &str = "tests/fixtures/smart/P110(UK)_1.0_1.4.8_TPAP.json";

/// Sets up the factory-reset device that `fixture` describes, end to end.
async fn set_up(fixture: &str) -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(fixture).start().await;

    let handler = ApiClient::new("local-only@example.com", "local-password")
        .setup_over_wifi(device.address())
        .await?;

    handler.get_device_info().await?;
    handler.scan_wifi().await?;
    handler
        .join_wifi(
            "Simulated Wi-Fi",
            "wifi-password",
            WifiSecurity::Wpa2Psk,
            chrono_tz::Europe::London,
        )
        .await?;

    Ok(())
}

/// A factory-reset P110 on firmware 1.3.1 takes the factory login over
/// KLAP.
#[tokio::test]
async fn a_factory_reset_p110_on_klap_joins_a_wifi_network() -> Result<(), Box<dyn Error>> {
    set_up(P110_KLAP_SETUP_WIFI_FIXTURE).await
}

/// A factory-reset P110 on firmware 1.4.8 takes the default passcode over
/// TPAP.
#[tokio::test]
async fn a_factory_reset_p110_on_tpap_joins_a_wifi_network() -> Result<(), Box<dyn Error>> {
    set_up(P110_TPAP_SETUP_WIFI_FIXTURE).await
}

/// A device that has joined a Wi-Fi network no longer takes the login of a
/// device in setup mode.
#[tokio::test]
async fn a_set_up_p110_on_klap_cannot_be_set_up_again() {
    let device = SimulatedDevice::new(P110_KLAP_FIXTURE).start().await;

    let login = ApiClient::new(device.username(), device.password())
        .setup_over_wifi(device.address())
        .await;

    assert_unauthorized(login, "HASH_MISMATCH");
}

/// A device in TPAP mode that has joined a Wi-Fi network no longer takes
/// the default passcode.
#[tokio::test]
async fn a_set_up_p110_on_tpap_cannot_be_set_up_again() {
    let device = SimulatedDevice::new(P110_TPAP_FIXTURE).start().await;

    let login = ApiClient::new(device.username(), device.password())
        .setup_over_wifi(device.address())
        .await;

    assert_unauthorized(login, "HASH_MISMATCH");
}
