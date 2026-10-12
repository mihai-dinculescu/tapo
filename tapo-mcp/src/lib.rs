pub mod auth;
pub mod config;
pub mod discovery;
pub mod server;
pub mod snapshots;
pub mod telemetry;

pub(crate) mod errors;
pub(crate) mod models;
pub(crate) mod requests;
pub(crate) mod resources;
pub(crate) mod tools;

use std::sync::Arc;

use axum::Router;
use config::AppConfig;
use discovery::DeviceCache;
use snapshots::SnapshotStore;

pub fn router(
    config: Arc<AppConfig>,
    devices: Arc<DeviceCache>,
    snapshots: Arc<SnapshotStore>,
) -> Router {
    let api_key = config.api_key.clone();
    let serve_snapshots = config.public_url.is_some();
    let mcp_service = server::new_service(config, devices, Arc::clone(&snapshots));
    let router = Router::new().route_service("/", mcp_service);

    let router = if let Some(key) = api_key {
        tracing::info!("API key authentication enabled");
        router.layer(axum::middleware::from_fn_with_state(
            key,
            auth::require_bearer_token,
        ))
    } else {
        tracing::warn!("No API key configured -- server is unauthenticated");
        router
    };

    // Merged outside the MCP service and after the auth layer, so snapshot links
    // skip both the API key and the `allowed_hosts` Host check. The random token
    // in the path is their only credential.
    if serve_snapshots {
        router.merge(snapshots::router(snapshots))
    } else {
        router
    }
}
