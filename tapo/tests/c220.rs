//! Runs the library end to end against a simulated C220 built from its
//! fixture in `tests/fixtures/smartcam/`. A camera takes TPAP over TLS and
//! its requests inside a `multipleRequest`; the login over AES SSL is not
//! simulated.

mod simulator;

use std::error::Error;

use tapo::{ApiClient, DeviceType, StreamExt};

use simulator::SimulatedDevice;
use simulator::assertions::assert_unauthorized;

const FIXTURE: &str = "tests/fixtures/smartcam/C220_1.0_1.4.4_TPAP.json";

#[tokio::test]
async fn discovery_finds_the_device_and_logs_in() -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(FIXTURE).start_with_discovery().await;

    let mut discovery = ApiClient::new(device.username(), device.password())
        .discover_devices(device.ip(), 5)
        .await?;

    let result = discovery
        .next()
        .await
        .ok_or("the device did not answer discovery")??;

    assert_eq!(result.device_type(), DeviceType::CameraPtz, "{result:?}");

    // The stream ends once the device at the target address has answered.
    assert!(discovery.next().await.is_none());

    Ok(())
}

#[tokio::test]
async fn a_c220_takes_a_login_by_ip_address() -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(FIXTURE).start().await;

    let handler = ApiClient::new(device.username(), device.password())
        .c220(device.address())
        .await?;

    handler.get_device_info().await?;

    Ok(())
}

#[tokio::test]
async fn a_c220_reads_its_presets() -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(FIXTURE).start().await;

    let handler = ApiClient::new(device.username(), device.password())
        .c220(device.address())
        .await?;

    handler.get_presets().await?;

    Ok(())
}

/// A camera answers a wrong password with the code that stands for an
/// expired session on a plug, which the library reads as the credentials
/// being wrong. The device is discovered first: it announces TPAP, so the
/// login does not fall back to AES SSL, which is not simulated.
#[tokio::test]
async fn a_c220_login_checks_the_password_only() -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(FIXTURE).start_with_discovery().await;

    let login = ApiClient::new(device.username(), "not the password")
        .discover_devices(device.ip(), 5)
        .await?
        .next()
        .await
        .ok_or("the device did not answer discovery")?
        .map_err(|error| error.source);
    assert_unauthorized(login, "TPAP_CREDENTIALS");

    ApiClient::new("someone-else@example.com", device.password())
        .discover_devices(device.ip(), 5)
        .await?
        .next()
        .await
        .ok_or("the device did not answer discovery")??;

    Ok(())
}
