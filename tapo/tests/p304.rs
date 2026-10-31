//! Runs the library end to end against a simulated P304M built from the
//! fixture in `tests/fixtures/smart/`. The power strip takes the requests of
//! its child plugs through `control_child`. The checks specific to the login
//! are in `tpap.rs`.

mod simulator;

use std::error::Error;

use chrono::{Duration, NaiveDate, TimeZone, Utc};
use tapo::requests::{EnergyDataInterval, PowerDataInterval};
use tapo::{ApiClient, DeviceType, Plug, StreamExt};

use simulator::SimulatedDevice;

const FIXTURE: &str = "tests/fixtures/smart/P304M(UK)_1.0_1.4.2_TPAP.json";

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
        DeviceType::PowerStripEnergyMonitoring,
        "{result:?}"
    );

    // The stream ends once the device at the target address has answered.
    assert!(discovery.next().await.is_none());

    Ok(())
}

#[tokio::test]
async fn a_p304_takes_a_login_by_ip_address() -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(FIXTURE).start().await;

    let power_strip = ApiClient::new(device.username(), device.password())
        .p304(device.address())
        .await?;

    power_strip.get_device_info().await?;

    Ok(())
}

#[tokio::test]
async fn a_p304_lists_its_plugs() -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(FIXTURE).start().await;

    let power_strip = ApiClient::new(device.username(), device.password())
        .p304(device.address())
        .await?;

    power_strip.get_child_device_list().await?;

    let plug = power_strip.plug(Plug::ByPosition(1)).await?;

    plug.get_device_info().await?;

    Ok(())
}

#[tokio::test]
async fn a_p304_reads_the_usage_of_a_plug() -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(FIXTURE).start().await;

    let power_strip = ApiClient::new(device.username(), device.password())
        .p304(device.address())
        .await?;

    let plug = power_strip.plug(Plug::ByPosition(1)).await?;

    plug.get_current_power().await?;
    plug.get_device_usage().await?;
    plug.get_energy_meter_data().await?;
    plug.get_energy_usage().await?;

    Ok(())
}

#[tokio::test]
async fn a_p304_reads_the_energy_and_power_data_of_a_plug() -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(FIXTURE).start().await;

    let power_strip = ApiClient::new(device.username(), device.password())
        .p304(device.address())
        .await?;

    let plug = power_strip.plug(Plug::ByPosition(1)).await?;

    plug.get_timezone().await?;

    // The day and the hour that the fixture's energy and power data cover.
    let day = NaiveDate::from_ymd_opt(2026, 10, 31).unwrap();
    let start = Utc.with_ymd_and_hms(2026, 10, 31, 13, 55, 0).unwrap();

    plug.get_energy_data(
        EnergyDataInterval::Hourly {
            start_date: day,
            end_date: day,
        },
        None,
    )
    .await?;
    plug.get_power_data(PowerDataInterval::Every5Minutes {
        start_date_time: start,
        end_date_time: start + Duration::hours(1),
    })
    .await?;

    Ok(())
}
