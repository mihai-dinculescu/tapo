/// P110, P110M and P115 Example
use std::{env, thread, time::Duration};

use chrono::{Datelike as _, NaiveDate, Utc};
use log::info;
use tapo::ApiClient;
use tapo::requests::{EnergyDataInterval, PowerDataInterval};

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let ip_address = env::var("IP_ADDRESS")?;

    let device = ApiClient::new(tapo_username, tapo_password)
        .p110(ip_address)
        .await?;

    info!("Turning device on...");
    device.on().await?;

    info!("Waiting 2 seconds...");
    thread::sleep(Duration::from_secs(2));

    info!("Turning device off...");
    device.off().await?;

    let device_info = device.get_device_info().await?;
    info!("Device info: {device_info:?}");

    let current_power = device.get_current_power().await?;
    info!("Current power: {current_power:?}");

    let device_usage = device.get_device_usage().await?;
    info!("Device usage: {device_usage:?}");

    let energy_usage = device.get_energy_usage().await?;
    info!("Energy usage: {energy_usage:?}");

    let current_date = Utc::now().naive_utc().date();

    // Energy data - Hourly interval
    // `start_date` and `end_date` are an inclusive interval that must not be greater than 8 days.
    let energy_data_hourly = device
        .get_energy_data(EnergyDataInterval::Hourly {
            start_date: current_date,
            end_date: current_date,
        })
        .await?;
    info!(
        "Energy data (hourly): Start date time '{}', Entries {}, First entry: {:?}",
        energy_data_hourly.start_date_time,
        energy_data_hourly.entries.len(),
        energy_data_hourly.entries.first()
    );

    // Energy data - Daily interval
    // `start_date` must be the first day of a quarter.
    let energy_data_daily = device
        .get_energy_data(EnergyDataInterval::Daily {
            start_date: NaiveDate::from_ymd_opt(
                current_date.year(),
                get_quarter_start_month(&current_date),
                1,
            )
            .unwrap(),
        })
        .await?;
    info!(
        "Energy data (daily): Start date time '{}', Entries {}, First entry: {:?}",
        energy_data_daily.start_date_time,
        energy_data_daily.entries.len(),
        energy_data_daily.entries.first()
    );

    // Energy data - Monthly interval
    // `start_date` must be the first day of a year.
    let energy_data_monthly = device
        .get_energy_data(EnergyDataInterval::Monthly {
            start_date: NaiveDate::from_ymd_opt(current_date.year(), 1, 1).unwrap(),
        })
        .await?;
    info!(
        "Energy data (monthly): Start date time '{}', Entries {}, First entry: {:?}",
        energy_data_monthly.start_date_time,
        energy_data_monthly.entries.len(),
        energy_data_monthly.entries.first()
    );

    // Power data - Every 5 minutes interval
    // `start_date_time` and `end_date_time` describe an exclusive interval.
    // If the result would yield more than 144 entries (i.e. 12 hours),
    // the `end_date_time` will be adjusted to an earlier date and time.
    let power_data_every_5_minutes = device
        .get_power_data(PowerDataInterval::Every5Minutes {
            start_date_time: Utc::now() - chrono::Duration::hours(12),
            end_date_time: Utc::now(),
        })
        .await?;
    info!(
        "Power data (every 5 minutes): Start date time '{}', End date time '{}', Entries {}, First entry: {:?}",
        power_data_every_5_minutes.start_date_time,
        power_data_every_5_minutes.end_date_time,
        power_data_every_5_minutes.entries.len(),
        power_data_every_5_minutes.entries.first()
    );

    // Power data - Hourly interval
    // `start_date_time` and `end_date_time` describe an exclusive interval.
    // If the result would yield more than 144 entries (i.e. 6 days),
    // the `end_date_time` will be adjusted to an earlier date and time.
    let power_data_hourly = device
        .get_power_data(PowerDataInterval::Hourly {
            start_date_time: Utc::now() - chrono::Duration::days(3),
            end_date_time: Utc::now(),
        })
        .await?;
    info!(
        "Power data (hourly): Start date time '{}', End date time '{}', Entries {}, First entry: {:?}",
        power_data_hourly.start_date_time,
        power_data_hourly.end_date_time,
        power_data_hourly.entries.len(),
        power_data_hourly.entries.first()
    );

    Ok(())
}

fn get_quarter_start_month(current_date: &NaiveDate) -> u32 {
    ((current_date.month() - 1) / 3) * 3 + 1
}
