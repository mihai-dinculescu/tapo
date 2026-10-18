use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use chrono::Utc;
use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, ContentBlock};
use tapo::DiscoveryResult;
use tapo::responses::Snapshot;
use tracing::field::Empty;

use crate::config::AppConfig;
use crate::device_locks::DeviceLocks;
use crate::errors::TapoMcpError;
use crate::models::{GetCapability, SnapshotResult, TakeSnapshotParams};
use crate::requests;
use crate::requests::CheckedDevice;
use crate::snapshots::SnapshotStore;
use crate::telemetry::run_in_span;

pub async fn take_snapshot(
    config: &AppConfig,
    locks: &DeviceLocks,
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

    let snapshot =
        requests::with_device(
            config,
            locks,
            &params.id,
            &params.ip,
            async |checked| match checked {
                CheckedDevice::Parent(DiscoveryResult::Camera { handler, .. }) => {
                    capture_snapshot(handler.get_snapshot(camera_username, camera_password)).await
                }
                CheckedDevice::Parent(DiscoveryResult::CameraPtz { handler, .. }) => {
                    capture_snapshot(handler.get_snapshot(camera_username, camera_password)).await
                }
                _ => Err(TapoMcpError::WrongDeviceType {
                    id: params.id.clone(),
                    capability: GetCapability::Snapshot.name().to_string(),
                    expected: "a camera".to_string(),
                }),
            },
        )
        .await?;

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

/// Awaits the camera's `snapshot` inside a `capture_snapshot` span.
async fn capture_snapshot(
    snapshot: impl Future<Output = Result<Snapshot, tapo::Error>>,
) -> Result<Snapshot, TapoMcpError> {
    let span = tracing::info_span!(
        "capture_snapshot",
        otel.status_code = Empty,
        "error.type" = Empty,
    );
    run_in_span(span, async { Ok(snapshot.await?) }).await
}
