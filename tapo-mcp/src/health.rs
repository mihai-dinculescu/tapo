use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;

/// The route for liveness probes: `/livez` answers while the process serves
/// requests.
///
/// Deliberately outside the API key and the `allowed_hosts` Host check: a probe
/// sends neither the bearer token nor a Host the operator lists, and the route
/// says nothing about the devices.
pub fn router() -> Router {
    Router::new().route("/livez", get(livez))
}

async fn livez() -> StatusCode {
    StatusCode::OK
}
