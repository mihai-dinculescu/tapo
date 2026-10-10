use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use chrono::Utc;
use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, ContentBlock};
use tapo::DiscoveryResult;

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::models::{CheckDeviceParams, SnapshotResult, TakeSnapshotParams};
use crate::requests;
use crate::requests::CheckedDevice;
use crate::snapshots::SnapshotStore;

pub async fn take_snapshot(
    config: &AppConfig,
    snapshots: &SnapshotStore,
    params: TakeSnapshotParams,
) -> Result<CallToolResult, McpError> {
    let camera_username = config
        .camera_username
        .as_deref()
        .ok_or(TapoMcpError::CameraCredentialsMissing)?;
    let camera_password = config
        .camera_password
        .as_deref()
        .ok_or(TapoMcpError::CameraCredentialsMissing)?;

    let check_params = CheckDeviceParams {
        id: params.id.clone(),
        ip: params.ip.clone(),
    };
    let checked = requests::check_device(config, check_params).await?;

    let snapshot = match checked {
        CheckedDevice::Parent(DiscoveryResult::CameraPtz { handler, .. }) => handler
            .get_snapshot(camera_username, camera_password)
            .await
            .map_err(TapoMcpError::Internal)?,
        _ => {
            return Err(TapoMcpError::WrongDeviceType {
                id: params.id,
                capability: "Snapshot".to_string(),
                expected: "a camera".to_string(),
            }
            .into());
        }
    };

    let captured_at = Utc::now();
    let size_bytes = snapshot.data.len();
    let encoded = STANDARD.encode(&snapshot.data);

    let (url, expires_at) = match &config.public_url {
        Some(public_url) => {
            let (token, expires_at) = snapshots.insert(snapshot.data);
            (
                Some(format!("{public_url}/snapshots/{token}.jpg")),
                Some(expires_at),
            )
        }
        None => (None, None),
    };

    let result = SnapshotResult {
        device_id: params.id,
        captured_at,
        size_bytes,
        url,
        expires_at,
    };

    Ok(CallToolResult::success(vec![
        ContentBlock::json(result)?,
        ContentBlock::image(encoded, snapshot.content_type),
    ]))
}
