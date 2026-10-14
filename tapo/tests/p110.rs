//! Runs the library end to end against a simulated P110 built from the
//! fixtures in `tests/fixtures/smart/`. The TPAP fixture of a device is its state
//! with Third-Party Compatibility off in the Tapo app, the KLAP fixture with
//! it on. The checks specific to each login are in `tpap.rs` and `klap.rs`.

mod simulator;

use std::error::Error;

use rstest::rstest;
use tapo::{ApiClient, DeviceType, StreamExt};

use simulator::SimulatedDevice;

const TPAP_FIXTURE: &str = "tests/fixtures/smart/P110(UK)_1.0_1.4.8_TPAP.json";
const KLAP_FIXTURE: &str = "tests/fixtures/smart/P110(UK)_1.0_1.4.8_KLAP.json";

#[rstest]
#[case::tpap(TPAP_FIXTURE)]
#[case::klap(KLAP_FIXTURE)]
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

    assert_eq!(
        result.device_type(),
        DeviceType::PlugEnergyMonitoring,
        "{result:?}"
    );

    // The stream ends once the device at the target address has answered.
    assert!(discovery.next().await.is_none());

    Ok(())
}

#[rstest]
#[case::tpap(TPAP_FIXTURE)]
#[case::klap(KLAP_FIXTURE)]
#[tokio::test]
async fn a_p110_takes_a_login_by_ip_address(#[case] fixture: &str) -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(fixture).start().await;

    let handler = ApiClient::new(device.username(), device.password())
        .p110(device.address())
        .await?;

    handler.get_device_info().await?;
    handler.get_device_usage().await?;

    Ok(())
}
