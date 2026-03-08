use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use tapo_mcp::config::AppConfig;
use tower::ServiceExt;

const MCP_INITIALIZE: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"test","version":"0.1.0"}}}"#;

fn test_config(api_key: Option<&str>) -> AppConfig {
    AppConfig {
        http_addr: "127.0.0.1:0".to_string(),
        username: "test@example.com".to_string(),
        password: "test-password".to_string(),
        discovery_target: "192.168.1.255".to_string(),
        discovery_timeout: 1,
        api_key: api_key.map(String::from),
    }
}

fn mcp_request(authorization: Option<&str>) -> Request {
    let mut builder = Request::builder()
        .method("POST")
        .uri("/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream");

    if let Some(value) = authorization {
        builder = builder.header("Authorization", value);
    }

    builder.body(Body::from(MCP_INITIALIZE)).unwrap()
}

async fn status(api_key: Option<&str>, authorization: Option<&str>) -> StatusCode {
    let app = tapo_mcp::router(test_config(api_key));
    app.oneshot(mcp_request(authorization))
        .await
        .unwrap()
        .status()
}

#[tokio::test]
async fn no_auth_allows_request() {
    assert_eq!(status(None, None).await, StatusCode::OK);
}

#[tokio::test]
async fn rejects_request_without_token() {
    assert_eq!(
        status(Some("test-key"), None).await,
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn rejects_request_with_wrong_token() {
    assert_eq!(
        status(Some("test-key"), Some("Bearer wrong-key")).await,
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn rejects_request_with_wrong_scheme() {
    assert_eq!(
        status(Some("test-key"), Some("Basic dXNlcjpwYXNz")).await,
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn allows_request_with_correct_token() {
    assert_eq!(
        status(Some("test-key"), Some("Bearer test-key")).await,
        StatusCode::OK
    );
}

#[tokio::test]
async fn allows_case_insensitive_scheme() {
    assert_eq!(
        status(Some("test-key"), Some("bearer test-key")).await,
        StatusCode::OK
    );
}

#[tokio::test]
async fn rejects_no_space_after_scheme() {
    assert_eq!(
        status(Some("test-key"), Some("Bearertest-key")).await,
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn unauthorized_includes_www_authenticate_header() {
    let app = tapo_mcp::router(test_config(Some("test-key")));
    let response = app.oneshot(mcp_request(None)).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.headers().get("WWW-Authenticate").unwrap(),
        "Bearer"
    );
}
