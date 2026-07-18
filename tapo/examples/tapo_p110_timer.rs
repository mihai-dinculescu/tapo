/// Demo: arming the plug's countdown ("Timer" in the Tapo app).
///
/// Build / run:
///   cargo run --example tapo_p110_timer
///
/// Environment variables: TAPO_USERNAME, TAPO_PASSWORD, IP_ADDRESS.
use std::env;
use std::time::Duration;

use log::info;
use tapo::ApiClient;

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

    info!("Baseline: plug off, no armed timer.");
    device.off().await?;
    device.clear_timer().await?;
    assert!(device.get_timer().await?.is_none());

    info!("Arming a 10-second 'turn ON' timer...");
    let armed = device.set_timer(Duration::from_secs(10), true).await?;
    info!("Armed: id={} delay={}s", armed.id, armed.delay_seconds);

    let read_back = device.get_timer().await?.expect("a timer should be armed");
    info!(
        "Read back: id={} remain={}s turn_on={}",
        read_back.id, read_back.remaining_seconds, read_back.turn_on
    );
    assert_eq!(read_back.id, armed.id);
    assert!(read_back.turn_on);

    info!("Waiting 15 seconds for the timer to fire (10s delay + slack)...");
    tokio::time::sleep(Duration::from_secs(15)).await;
    assert!(
        device.get_device_info().await?.device_on,
        "plug should be ON after the timer fired",
    );
    info!("Timer fired — plug is ON.");

    info!("Arming a 5-second 'turn OFF' timer and clearing it before it fires...");
    device.set_timer(Duration::from_secs(5), false).await?;
    device.clear_timer().await?;
    assert!(device.get_timer().await?.is_none());

    info!("Waiting 10 seconds to confirm the cleared timer did not fire...");
    tokio::time::sleep(Duration::from_secs(10)).await;
    assert!(
        device.get_device_info().await?.device_on,
        "plug should still be on after the cleared timer's original deadline",
    );

    device.off().await?;
    info!("PASS");
    Ok(())
}
