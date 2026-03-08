pub mod auth;
pub mod config;
pub mod server;
pub mod telemetry;

pub(crate) mod errors;
pub(crate) mod models;
pub(crate) mod requests;
pub(crate) mod resources;
pub(crate) mod tools;

use std::sync::Arc;

use axum::Router;
use config::AppConfig;

pub fn router(config: AppConfig) -> Router {
    let api_key = config.api_key.clone();
    let mcp_service = server::new_service(Arc::new(config));
    let router = Router::new().route_service("/", mcp_service);

    if let Some(key) = api_key {
        tracing::info!("API key authentication enabled");
        router.layer(axum::middleware::from_fn_with_state(
            key,
            auth::require_bearer_token,
        ))
    } else {
        tracing::warn!("No API key configured -- server is unauthenticated");
        router
    }
}
