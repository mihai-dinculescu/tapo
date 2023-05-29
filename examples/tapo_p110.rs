/// P110 Example
use std::{env, thread, time::Duration};

use log::{info, LevelFilter};
use tapo::{requests::EnergyDataInterval, ApiClient};
use time::macros::{date, datetime};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_level = env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .parse()
        .unwrap_or(LevelFilter::Info);

    pretty_env_logger::formatted_timed_builder()
        .filter(Some("tapo"), log_level)
        .init();

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let ip_address = env::var("IP_ADDRESS")?;

    let device = ApiClient::new(tapo_username, tapo_password)?
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

    let device_usage = device.get_device_usage().await?;
    info!("Device usage: {device_usage:?}");

    let energy_usage = device.get_energy_usage().await?;
    info!("Energy usage: {energy_usage:?}");

    let energy_data_hourly = device
        .get_energy_data(EnergyDataInterval::Hourly {
            start_datetime: datetime!(2023-02-24 00:00 UTC),
            end_datetime: datetime!(2023-02-24 23:59 UTC),
        })
        .await?;
    info!("Energy data (hourly): {energy_data_hourly:?}");

    let energy_data_daily = device
        .get_energy_data(EnergyDataInterval::Daily {
            start_date: date!(2023 - 01 - 01),
        })
        .await?;
    info!("Energy data (daily): {energy_data_daily:?}");

    let energy_data_monthly = device
        .get_energy_data(EnergyDataInterval::Monthly {
            start_date: date!(2023 - 01 - 01),
        })
        .await?;
    info!("Energy data (monthly): {energy_data_monthly:?}");

    Ok(())
}
