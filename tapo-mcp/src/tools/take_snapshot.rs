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

pub async fn take_snapshot(
    config: &AppConfig,
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

    let result = SnapshotResult {
        device_id: params.id,
        captured_at: Utc::now(),
        content_type: snapshot.content_type.clone(),
        size_bytes: snapshot.data.len(),
    };

    let encoded = STANDARD.encode(&snapshot.data);
    Ok(CallToolResult::success(vec![
        ContentBlock::json(result)?,
        ContentBlock::image(encoded, snapshot.content_type),
    ]))
}
