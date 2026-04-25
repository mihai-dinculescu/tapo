/// Tapo cameras with PTZ (C210, C220, C225, C325WB, C520WS, TC40, TC70) Example
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
    let camera_username = env::var("TAPO_CAMERA_USERNAME")?;
    let camera_password = env::var("TAPO_CAMERA_PASSWORD")?;

    let device = ApiClient::new(&tapo_username, &tapo_password)
        .c220(ip_address)
        .await?;

    let device_info = device.get_device_info().await?;
    info!("Device info: {device_info:?}");

    let rtsp_url = device.get_rtsp_stream_url(&camera_username, &camera_password);
    info!("RTSP HD: {}", rtsp_url.hd);
    info!("RTSP SD: {}", rtsp_url.sd);
    info!("RTSP MJPEG: {}", rtsp_url.mjpeg);

    info!("Capturing snapshot...");
    let snapshot = device
        .get_snapshot(&camera_username, &camera_password)
        .await?;
    let snapshot_path = format!("snapshot_{}.jpg", std::process::id());
    std::fs::write(&snapshot_path, &snapshot.data)?;
    info!(
        "Saved snapshot ({} bytes, {}) to {snapshot_path}",
        snapshot.data.len(),
        snapshot.content_type,
    );

    let preset_name = format!("example_{}", std::process::id());

    info!("Saving current position as preset '{preset_name}'...");
    device.save_preset(&preset_name).await?;

    let presets = device.get_presets().await?;
    info!("Presets: {presets:?}");

    let preset_id = presets
        .iter()
        .find(|p| p.name == preset_name)
        .expect("preset not found")
        .id
        .clone();

    info!("Panning and tilting by 10, 10...");
    device.pan_tilt(10, 10).await?;

    info!("Waiting 2 seconds...");
    tokio::time::sleep(Duration::from_secs(2)).await;

    info!("Going back to saved preset (id '{preset_id}')...");
    device.goto_preset(&preset_id).await?;

    info!("Waiting 2 seconds...");
    tokio::time::sleep(Duration::from_secs(2)).await;

    info!("Deleting preset (id '{preset_id}')...");
    device.delete_preset(&preset_id).await?;

    Ok(())
}
