//! Runs the library end to end against a simulated L930 light strip built
//! from its fixture in `tests/fixtures/smart/`. The strip reports its model
//! as `L930-5(EU)` in discovery and as `L930` in its device info, which
//! holds the lighting effect that it ran last. The checks specific to the
//! login are in `tpap.rs`.

mod simulator;

use std::error::Error;

use tapo::{ApiClient, DeviceType, StreamExt};

use simulator::SimulatedDevice;

const FIXTURE: &str = "tests/fixtures/smart/L930-5(EU)_1.0_1.4.3_TPAP.json";

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

    assert_eq!(
        result.device_type(),
        DeviceType::RgbicLightStrip,
        "{result:?}"
    );

    // The stream ends once the device at the target address has answered.
    assert!(discovery.next().await.is_none());

    Ok(())
}

#[tokio::test]
async fn an_l930_reads_its_state() -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(FIXTURE).start().await;

    let handler = ApiClient::new(device.username(), device.password())
        .l930(device.address())
        .await?;

    handler.get_device_info().await?;
    handler.get_device_usage().await?;

    Ok(())
}

#[tokio::test]
async fn an_l930_reads_its_firmware() -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(FIXTURE).start().await;

    let handler = ApiClient::new(device.username(), device.password())
        .l930(device.address())
        .await?;

    handler.get_latest_firmware().await?;
    handler.get_firmware_update_state().await?;

    Ok(())
}
