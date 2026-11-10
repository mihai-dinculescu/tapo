use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use tapo_mcp::config::AppConfig;
use tower::ServiceExt;

fn test_config() -> AppConfig {
    AppConfig {
        http_addr: "127.0.0.1:0".to_string(),
        username: "test@example.com".to_string(),
        password: "test-password".to_string(),
        camera_username: None,
        camera_password: None,
        discovery_target: "192.168.1.255".to_string(),
        discovery_timeout: 1,
        discovery_interval: 600,
        api_key: Some("test-key".to_string()),
        allowed_hosts: vec![],
        public_url: None,
    }
}

/// A probe as the kubelet sends it: no bearer token, and the pod's address as Host.
async fn probe(path: &str) -> StatusCode {
    let request = Request::builder()
        .method("GET")
        .uri(path)
        .header("Host", "10.0.0.5:3000")
        .body(Body::empty())
        .unwrap();
    tapo_mcp::router(
        test_config().into(),
        Default::default(),
        Default::default(),
        Default::default(),
    )
    .oneshot(request)
    .await
    .unwrap()
    .status()
}

#[tokio::test]
async fn livez_answers_without_bearer_token_or_allowed_host() {
    assert_eq!(probe("/livez").await, StatusCode::OK);
}
