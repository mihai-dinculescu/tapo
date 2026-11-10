pub mod auth;
pub mod config;
pub mod device_locks;
pub mod discovery;
pub mod server;
pub mod snapshots;
pub mod telemetry;

pub(crate) mod errors;
pub(crate) mod models;
pub(crate) mod requests;
pub(crate) mod resources;
pub(crate) mod tools;

use std::sync::{Arc, Once};

use axum::Router;
use axum::extract::Request;
use axum::http::header::HOST;
use axum::http::{Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::Response;
use config::AppConfig;
use device_locks::DeviceLocks;
use discovery::DeviceCache;
use snapshots::SnapshotStore;

/// The path the MCP endpoint is served at.
const MCP_PATH: &str = "/mcp";

pub fn router(
    config: Arc<AppConfig>,
    devices: Arc<DeviceCache>,
    locks: Arc<DeviceLocks>,
    snapshots: Arc<SnapshotStore>,
) -> Router {
    let api_key = config.api_key.clone();
    let serve_snapshots = config.public_url.is_some();
    let mcp_service = server::new_service(config, devices, locks, Arc::clone(&snapshots));
    // `/` is the pre-0.7.0 endpoint, removed in a later release. Both routes share one
    // service, so they share sessions too.
    let legacy = Router::new()
        .route_service("/", mcp_service.clone())
        .layer(middleware::from_fn(warn_deprecated_path));
    let router = Router::new()
        .route_service(MCP_PATH, mcp_service)
        .merge(legacy)
        .layer(middleware::from_fn(warn_refused_host));

    let router = if let Some(key) = api_key {
        tracing::info!("API key authentication enabled");
        router.layer(middleware::from_fn_with_state(
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

/// Logs a deprecation warning the first time a client uses `/`.
///
/// Only a POST that the MCP service accepts counts, so health checks and
/// requests rejected by the `allowed_hosts` check don't count. Once per process
/// rather than once per session: clients on protocol `2026-07-28` and later have
/// no sessions.
async fn warn_deprecated_path(request: Request, next: Next) -> Response {
    static WARNED: Once = Once::new();

    let is_post = request.method() == Method::POST;
    let response = next.run(request).await;
    if is_post && response.status().is_success() {
        WARNED.call_once(|| {
            tracing::warn!(
                "MCP client connected on `/`, which is deprecated and will be removed in a later release; use `{MCP_PATH}` instead"
            );
        });
    }
    response
}

/// Logs a warning for each request the MCP service refuses for its `Host`
/// header, which is the only reason it answers `403 Forbidden`.
async fn warn_refused_host(request: Request, next: Next) -> Response {
    let host = request.headers().get(HOST).cloned();
    let response = next.run(request).await;
    if response.status() == StatusCode::FORBIDDEN {
        let host = host.as_ref().and_then(|host| host.to_str().ok());
        tracing::warn!(
            host,
            "Refused a request whose `Host` header is not in TAPO_MCP_ALLOWED_HOSTS"
        );
    }
    response
}
