//! Runs the library end to end against a simulated L535 built from its
//! fixture in `tests/fixtures/smart/`. The L535 is served by the L530
//! handler. The checks specific to each login are in `tpap.rs` and `klap.rs`.

mod simulator;

use std::error::Error;

use tapo::{ApiClient, DeviceType, StreamExt};

use simulator::SimulatedDevice;

const FIXTURE: &str = "tests/fixtures/smart/L535B(EU)_3.0_1.4.3_TPAP.json";

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

    assert_eq!(result.device_type(), DeviceType::ColorLight, "{result:?}");

    // The stream ends once the device at the target address has answered.
    assert!(discovery.next().await.is_none());

    Ok(())
}

#[tokio::test]
async fn an_l535_takes_a_login_by_ip_address() -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(FIXTURE).start().await;

    let handler = ApiClient::new(device.username(), device.password())
        .l530(device.address())
        .await?;

    handler.get_device_info().await?;
    handler.get_device_usage().await?;

    Ok(())
}
