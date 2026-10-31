//! Runs the library end to end against a simulated P110 built from the
//! fixtures in `tests/fixtures/smart/`. The TPAP fixture of a device is its state
//! with Third-Party Compatibility off in the Tapo app, the KLAP fixture with
//! it on. The checks specific to each login are in `tpap.rs` and `klap.rs`.

mod simulator;

use std::error::Error;

use chrono::{Duration, NaiveDate, TimeZone, Utc};
use rstest::rstest;
use tapo::requests::{EnergyDataInterval, PowerDataInterval};
use tapo::{ApiClient, DeviceType, StreamExt};

use simulator::SimulatedDevice;

const P110_TPAP_FIXTURE: &str = "tests/fixtures/smart/P110(UK)_1.0_1.4.8_TPAP.json";
const P110_KLAP_FIXTURE: &str = "tests/fixtures/smart/P110(UK)_1.0_1.4.8_KLAP.json";
const P110M_TPAP_FIXTURE: &str = "tests/fixtures/smart/P110M(US)_1.0_1.4.4_TPAP.json";

#[rstest]
#[case::p110_tpap(P110_TPAP_FIXTURE)]
#[case::p110_klap(P110_KLAP_FIXTURE)]
#[case::p110m_tpap(P110M_TPAP_FIXTURE)]
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
#[case::p110_tpap(P110_TPAP_FIXTURE)]
#[case::p110_klap(P110_KLAP_FIXTURE)]
#[case::p110m_tpap(P110M_TPAP_FIXTURE)]
#[tokio::test]
async fn a_p110_takes_a_login_by_ip_address(#[case] fixture: &str) -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(fixture).start().await;

    let handler = ApiClient::new(device.username(), device.password())
        .p110(device.address())
        .await?;

    handler.get_device_info().await?;

    Ok(())
}

#[rstest]
#[case::p110_tpap(P110_TPAP_FIXTURE)]
#[case::p110_klap(P110_KLAP_FIXTURE)]
#[case::p110m_tpap(P110M_TPAP_FIXTURE)]
#[tokio::test]
async fn a_p110_reads_its_usage(#[case] fixture: &str) -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(fixture).start().await;

    let handler = ApiClient::new(device.username(), device.password())
        .p110(device.address())
        .await?;

    handler.get_device_usage().await?;
    handler.get_energy_meter_data().await?;

    Ok(())
}

// The KLAP fixture holds no energy or power data.
#[rstest]
#[case::p110_tpap(P110_TPAP_FIXTURE)]
#[case::p110m_tpap(P110M_TPAP_FIXTURE)]
#[tokio::test]
async fn a_p110_reads_its_energy_and_power_data(
    #[case] fixture: &str,
) -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(fixture).start().await;

    let handler = ApiClient::new(device.username(), device.password())
        .p110(device.address())
        .await?;

    handler.get_timezone().await?;

    // The day and the hour that the fixture's energy and power data cover.
    let day = NaiveDate::from_ymd_opt(2026, 10, 31).unwrap();
    let start = Utc.with_ymd_and_hms(2026, 10, 31, 13, 55, 0).unwrap();

    handler
        .get_energy_data(
            EnergyDataInterval::Hourly {
                start_date: day,
                end_date: day,
            },
            None,
        )
        .await?;
    handler
        .get_power_data(PowerDataInterval::Every5Minutes {
            start_date_time: start,
            end_date_time: start + Duration::hours(1),
        })
        .await?;

    Ok(())
}
