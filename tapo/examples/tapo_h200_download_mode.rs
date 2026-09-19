//! H200 download mode experiment
//!
//! Downloads one recording three ways and logs how each went, to compare the
//! hub's `type=download` stream with the `type=sdvod` playback that
//! `download_recording` uses:
//!
//! 1. `playback`: `download_recording`, which names the camera by its mac.
//! 2. `download_app`: the download request as the Tapo app sends it, with the
//!    mac in the stream URI and both the mac and the device id in the request.
//! 3. `download_device_id`: the same request with the device id alone.
//!
//! Run it with `cargo run --example tapo_h200_download_mode --features debug`.

use log::{info, warn};
use tapo::responses::RecordingDownloadResult;
use tapo::{ApiClient, Error};

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let [tapo_username, tapo_password, ip_address] =
        common::require_env_vars(["TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"])?;

    let hub = ApiClient::new(tapo_username, tapo_password)
        .h200(ip_address)
        .await?;

    let end_time = chrono::Utc::now();
    let start_time = end_time - chrono::Duration::days(7);

    // The first recording on the last date of the first camera that has one.
    let mut target = None;
    for camera in hub.get_general_device_list().await? {
        if !camera.hub_storage_enabled {
            continue;
        }

        let recording_dates = hub
            .get_recording_dates(
                camera.device_id.clone(),
                camera.mac.clone(),
                start_time,
                end_time,
            )
            .await?;
        let Some(recording_date) = recording_dates.last() else {
            continue;
        };

        let recordings = hub
            .get_recordings(
                camera.device_id.clone(),
                camera.mac.clone(),
                recording_date.start_time,
                recording_date.end_time,
            )
            .await?;
        if let Some(recording) = recordings.into_iter().next() {
            target = Some((camera, recording));
            break;
        }
    }

    let Some((camera, recording)) = target else {
        info!("No recording found to download.");
        return Ok(());
    };

    info!(
        "Downloading the {} s recording from {} to {} of {} (id: {}, mac: {}) three ways...",
        (recording.end_time - recording.start_time).num_seconds(),
        recording.start_time,
        recording.end_time,
        camera.alias,
        camera.device_id,
        camera.mac
    );
    let file_prefix = format!("recording_{}", recording.start_time.timestamp());

    let mut media = Vec::new();
    let result = hub
        .download_recording(
            camera.mac.clone(),
            recording.start_time,
            recording.end_time,
            &mut media,
        )
        .await;
    report("playback", &file_prefix, result, &media)?;

    let mut media = Vec::new();
    let result = hub
        .probe_recording_download(
            camera.device_id.clone(),
            Some(camera.mac),
            recording.start_time,
            recording.end_time,
            &mut media,
        )
        .await;
    report("download_app", &file_prefix, result, &media)?;

    let mut media = Vec::new();
    let result = hub
        .probe_recording_download(
            camera.device_id,
            None,
            recording.start_time,
            recording.end_time,
            &mut media,
        )
        .await;
    report("download_device_id", &file_prefix, result, &media)?;

    Ok(())
}

/// Logs how a download went and writes whatever media arrived, even when the
/// download failed part way.
fn report(
    mode: &str,
    file_prefix: &str,
    result: Result<RecordingDownloadResult, Error>,
    media: &[u8],
) -> std::io::Result<()> {
    let path = format!("{file_prefix}_{mode}.ts");
    if !media.is_empty() {
        std::fs::write(&path, media)?;
    }

    match result {
        Ok(result) => {
            let duration = match result.duration_s {
                Some(duration_s) => format!("{duration_s:.3} s"),
                None => "an unknown length".to_string(),
            };
            info!(
                "{mode}: wrote {} bytes to {path}: {duration} of video, {:?}.",
                media.len(),
                result.outcome
            );
        }
        Err(err) => {
            warn!(
                "{mode}: failed after {} bytes of media: {err:?}",
                media.len()
            );
        }
    }

    Ok(())
}
