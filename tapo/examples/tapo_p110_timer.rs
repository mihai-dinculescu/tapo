/// P110, P110M and P115 Timer Example
use std::time::Duration;

use log::info;
use tapo::ApiClient;
use tapo::responses::PowerState;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let [tapo_username, tapo_password, ip_address] =
        common::require_env_vars(["TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"])?;

    let device = ApiClient::new(tapo_username, tapo_password)
        .p110(ip_address)
        .await?;

    // The delay must be between 1 second and 24 hours.
    info!("Arming a 5 second timer that turns the device on...");
    let timer = device
        .set_timer(Duration::from_secs(5), PowerState::On)
        .await?;
    info!("Armed timer: {timer:?}");

    let timer = device.get_timer().await?;
    info!("Timer: {timer:?}");

    info!("Waiting 10 seconds for the timer to fire...");
    tokio::time::sleep(Duration::from_secs(10)).await;

    info!("Arming a 5 second timer that turns the device off...");
    let timer = device
        .set_timer(Duration::from_secs(5), PowerState::Off)
        .await?;
    info!("Armed timer: {timer:?}");

    info!("Clearing the timer before it fires...");
    tokio::time::sleep(Duration::from_secs(1)).await;
    device.clear_timer().await?;

    info!("Waiting 10 seconds to show that the cleared timer does not fire...");
    tokio::time::sleep(Duration::from_secs(10)).await;

    info!("Turning device off...");
    device.off().await?;

    Ok(())
}
