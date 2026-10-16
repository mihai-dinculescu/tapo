//! Runs the library end to end against simulated L530 and L535 bulbs built
//! from the fixtures in `tests/fixtures/smart/`. Both are served by the L530
//! handler. The L530 reports its model as `L530 Series` and the L535B as
//! `L535B`. The checks specific to each login are in `tpap.rs` and
//! `klap.rs`.

mod simulator;

use std::error::Error;

use rstest::rstest;
use tapo::{ApiClient, DeviceType, StreamExt};

use simulator::SimulatedDevice;

const L530_FIXTURE: &str = "tests/fixtures/smart/L530 Series_1.0_1.3.1_KLAP.json";
const L535_FIXTURE: &str = "tests/fixtures/smart/L535B(EU)_3.0_1.4.3_TPAP.json";

#[rstest]
#[case::l530(L530_FIXTURE)]
#[case::l535(L535_FIXTURE)]
#[tokio::test]
async fn discovery_finds_the_device_and_logs_in(
    #[case] fixture: &str,
) -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(fixture).start_with_discovery().await;

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

#[rstest]
#[case::l530(L530_FIXTURE)]
#[case::l535(L535_FIXTURE)]
#[tokio::test]
async fn an_l53x_takes_a_login_by_ip_address(#[case] fixture: &str) -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(fixture).start().await;

    let handler = ApiClient::new(device.username(), device.password())
        .l530(device.address())
        .await?;

    handler.get_device_info().await?;
    handler.get_device_usage().await?;

    Ok(())
}
