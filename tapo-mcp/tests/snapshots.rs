use std::sync::Arc;

use axum::body::{Body, to_bytes};
use axum::extract::Request;
use axum::http::{StatusCode, header};
use axum::response::Response;
use tapo_mcp::config::AppConfig;
use tapo_mcp::snapshots::SnapshotStore;
use tower::ServiceExt;

const MCP_INITIALIZE: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"test","version":"0.1.0"}}}"#;
const IMAGE: &[u8] = b"\xFF\xD8\xFF\xE0fake-jpeg";

fn test_config(api_key: Option<&str>, public_url: Option<&str>) -> AppConfig {
    AppConfig {
        http_addr: "127.0.0.1:0".to_string(),
        username: "test@example.com".to_string(),
        password: "test-password".to_string(),
        camera_username: None,
        camera_password: None,
        discovery_target: "192.168.1.255".to_string(),
        discovery_timeout: 1,
        api_key: api_key.map(String::from),
        allowed_hosts: vec![],
        public_url: public_url.map(String::from),
    }
}

fn seeded_store() -> (Arc<SnapshotStore>, String) {
    let store = Arc::new(SnapshotStore::default());
    let (token, _) = store.insert(IMAGE.to_vec());
    (store, token)
}

async fn get(config: AppConfig, store: Arc<SnapshotStore>, path: &str) -> Response {
    let request = Request::builder()
        .method("GET")
        .uri(path)
        .header("Host", "localhost")
        .body(Body::empty())
        .unwrap();
    tapo_mcp::router(config, store)
        .oneshot(request)
        .await
        .unwrap()
}

#[tokio::test]
async fn unknown_token_returns_not_found_without_bearer_token() {
    let (store, _) = seeded_store();
    let config = test_config(Some("test-key"), Some("http://localhost:3000"));
    let response = get(config, store, "/snapshots/unknown.jpg").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn mcp_endpoint_still_requires_bearer_token() {
    let (store, _) = seeded_store();
    let config = test_config(Some("test-key"), Some("http://localhost:3000"));
    let request = Request::builder()
        .method("POST")
        .uri("/")
        .header("Host", "localhost")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream")
        .body(Body::from(MCP_INITIALIZE))
        .unwrap();
    let response = tapo_mcp::router(config, store)
        .oneshot(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn seeded_token_serves_the_image() {
    let (store, token) = seeded_store();
    let config = test_config(Some("test-key"), Some("http://localhost:3000"));
    let response = get(config, store, &format!("/snapshots/{token}.jpg")).await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()[header::CONTENT_TYPE], "image/jpeg");
    assert_eq!(
        response.headers()[header::CACHE_CONTROL],
        "private, no-store"
    );
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(&body[..], IMAGE);
}

#[tokio::test]
async fn token_without_extension_is_not_found() {
    let (store, token) = seeded_store();
    let config = test_config(None, Some("http://localhost:3000"));
    let response = get(config, store, &format!("/snapshots/{token}")).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn route_absent_without_public_url() {
    let (store, token) = seeded_store();
    let config = test_config(None, None);
    let response = get(config, store, &format!("/snapshots/{token}.jpg")).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
