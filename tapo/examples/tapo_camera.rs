/// Tapo cameras without PTZ (C120) Example
use log::info;
use tapo::ApiClient;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let [
        tapo_username,
        tapo_password,
        ip_address,
        camera_username,
        camera_password,
    ] = common::require_env_vars([
        "TAPO_USERNAME",
        "TAPO_PASSWORD",
        "IP_ADDRESS",
        "TAPO_CAMERA_USERNAME",
        "TAPO_CAMERA_PASSWORD",
    ])?;

    let device = ApiClient::new(&tapo_username, &tapo_password)
        .c120(ip_address)
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

    Ok(())
}
