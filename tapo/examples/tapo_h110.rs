//! H110 Example
//!
//! The H110 is handled by the same [`HubHandler`](tapo::HubHandler) as the H100.
//! On top of the sensors that the H100 supports, it can also have IR remotes as
//! child devices, which must be configured in the Tapo app first.
//!
//! Set the optional `IR_REMOTE` and `IR_KEY` environment variables to send one of
//! the keys stored on a remote:
//!
//! ```bash
//! export IR_REMOTE="Living Room TV"
//! export IR_KEY=POWER
//! ```
use std::env;

use log::info;
use tapo::responses::ChildDeviceHubResult;
use tapo::{ApiClient, HubDevice};

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let [tapo_username, tapo_password, ip_address] =
        common::require_env_vars(["TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"])?;

    let hub = ApiClient::new(tapo_username, tapo_password)
        .h110(ip_address)
        .await?;

    let device_info = hub.get_device_info().await?;
    info!("Device info: {device_info:?}");

    info!("Getting child devices...");
    let child_device_list = hub.get_child_device_list().await?;

    for child in child_device_list {
        match child {
            ChildDeviceHubResult::IrRemote(device) => {
                let keys = device
                    .key_list
                    .iter()
                    .map(|key| format!("{} ({})", key.name, key.display_name))
                    .collect::<Vec<_>>();

                info!(
                    "Found IR remote child device with nickname: {}, id: {}, model: {}, keys: {}.",
                    device.nickname,
                    device.device_id,
                    device.model,
                    keys.join(", ")
                );
            }
            child => {
                info!(
                    "Found child device with nickname: {}, id: {}, model: {}.",
                    child.nickname(),
                    child.device_id(),
                    child.model()
                );
            }
        }
    }

    match (env::var("IR_REMOTE"), env::var("IR_KEY")) {
        (Ok(remote_nickname), Ok(key_name)) => {
            info!("Sending the '{key_name}' key on the '{remote_nickname}' remote...");

            let remote = hub
                .ir_remote(HubDevice::ByNickname(remote_nickname))
                .await?;
            remote.send_ir_cmd_by_id(key_name).await?;

            info!("The IR command has been sent.");
        }
        _ => {
            info!("Set the IR_REMOTE and IR_KEY environment variables to send an IR command.");
        }
    }

    Ok(())
}
