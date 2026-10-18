use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::Response;
use opentelemetry::global;
use opentelemetry::trace::{SpanId, SpanKind, Status, TraceId, TracerProvider as _};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider, SpanData};
use serde_json::{Value, json};
use tapo_mcp::config::AppConfig;
use tapo_mcp::telemetry::DEFAULT_FILTER;
use tower::ServiceExt;
use tracing::subscriber::DefaultGuard;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;

const PROTOCOL_VERSION: &str = "2025-06-18";
const TRACE_ID: &str = "0af7651916cd43dd8448eb211c80319c";
const PARENT_SPAN_ID: &str = "b7ad6b7169203331";
const META_TRACE_ID: &str = "4bf92f3577b34da6a3ce929d0e0e4736";
const META_PARENT_SPAN_ID: &str = "00f067aa0ba902b7";

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
        api_key: None,
        allowed_hosts: vec![],
        public_url: None,
    }
}

/// Routes this thread's spans to an in-memory exporter, behind the same global
/// `EnvFilter` as production. The tests use the current-thread runtime, so
/// rmcp's session tasks run on this thread too.
fn capture_spans(filter: &str) -> (InMemorySpanExporter, SdkTracerProvider, DefaultGuard) {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let exporter = InMemorySpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::new(filter))
        .with(tracing_opentelemetry::layer().with_tracer(provider.tracer("test")));
    let guard = tracing::subscriber::set_default(subscriber);
    (exporter, provider, guard)
}

fn router() -> Router {
    tapo_mcp::router(
        Arc::new(test_config()),
        Default::default(),
        Default::default(),
        Default::default(),
    )
}

async fn post(
    router: &Router,
    session_id: Option<&str>,
    traceparent: Option<String>,
    body: Value,
) -> Response {
    let mut request = Request::builder()
        .method("POST")
        .uri("/")
        .header("Host", "localhost")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream");
    if let Some(session_id) = session_id {
        request = request
            .header("Mcp-Session-Id", session_id)
            .header("MCP-Protocol-Version", PROTOCOL_VERSION);
    }
    if let Some(traceparent) = traceparent {
        request = request.header("traceparent", traceparent);
    }
    let request = request.body(Body::from(body.to_string())).unwrap();
    router.clone().oneshot(request).await.unwrap()
}

/// Runs `initialize` and `notifications/initialized`, returning the session id.
async fn open_session(router: &Router) -> String {
    let response = post(
        router,
        None,
        None,
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": {},
                "clientInfo": { "name": "test", "version": "0.1.0" }
            }
        }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let session_id = response.headers()["mcp-session-id"]
        .to_str()
        .unwrap()
        .to_string();
    to_bytes(response.into_body(), usize::MAX).await.unwrap();

    let response = post(
        router,
        Some(&session_id),
        None,
        json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::ACCEPTED);

    session_id
}

/// Calls a tool and waits for its response, by which time its span has ended.
async fn call_tool(
    router: &Router,
    session_id: &str,
    traceparent: Option<String>,
    params: Value,
) -> String {
    let response = post(
        router,
        Some(session_id),
        traceparent,
        json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": params }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    String::from_utf8(body.to_vec()).unwrap()
}

fn tool_span(exporter: &InMemorySpanExporter) -> SpanData {
    let mut spans: Vec<_> = exporter
        .get_finished_spans()
        .unwrap()
        .into_iter()
        .filter(|span| span.name.starts_with("tools/call"))
        .collect();
    assert_eq!(spans.len(), 1, "expected exactly one tool call span");
    spans.remove(0)
}

fn attr(span: &SpanData, key: &str) -> Option<String> {
    span.attributes
        .iter()
        .find(|kv| kv.key.as_str() == key)
        .map(|kv| kv.value.as_str().into_owned())
}

#[tokio::test]
async fn tool_call_span_has_mcp_attributes() {
    let (exporter, _provider, _guard) = capture_spans(DEFAULT_FILTER);
    let router = router();
    let session_id = open_session(&router).await;

    call_tool(
        &router,
        &session_id,
        None,
        json!({ "name": "take_snapshot", "arguments": { "id": "camera-1", "ip": "192.0.2.10" } }),
    )
    .await;

    let span = tool_span(&exporter);
    assert_eq!(span.name, "tools/call take_snapshot");
    assert_eq!(span.span_kind, SpanKind::Server);
    assert_eq!(
        attr(&span, "mcp.method.name").as_deref(),
        Some("tools/call")
    );
    assert_eq!(
        attr(&span, "gen_ai.tool.name").as_deref(),
        Some("take_snapshot")
    );
    assert_eq!(
        attr(&span, "gen_ai.operation.name").as_deref(),
        Some("execute_tool")
    );
    assert_eq!(attr(&span, "jsonrpc.request.id").as_deref(), Some("2"));
    assert_eq!(
        attr(&span, "mcp.session.id").as_deref(),
        Some(session_id.as_str())
    );
    assert_eq!(
        attr(&span, "mcp.protocol.version").as_deref(),
        Some(PROTOCOL_VERSION)
    );
    assert_eq!(attr(&span, "tapo.device.id").as_deref(), Some("camera-1"));
    assert_eq!(attr(&span, "tapo.device.ip").as_deref(), Some("192.0.2.10"));
}

#[tokio::test]
async fn unknown_tool_span_has_generic_name() {
    let (exporter, _provider, _guard) = capture_spans(DEFAULT_FILTER);
    let router = router();
    let session_id = open_session(&router).await;

    call_tool(
        &router,
        &session_id,
        None,
        json!({
            "name": "no_such_tool",
            "arguments": { "id": "camera-1", "ip": "192.0.2.10" },
        }),
    )
    .await;

    let span = tool_span(&exporter);
    assert_eq!(span.name, "tools/call");
    assert_eq!(attr(&span, "gen_ai.tool.name"), None);
    assert_eq!(attr(&span, "tapo.device.id"), None);
    assert_eq!(attr(&span, "tapo.device.ip"), None);
    assert!(matches!(span.status, Status::Error { .. }));
}

#[tokio::test]
async fn tool_call_without_traceparent_is_a_root_span() {
    // Also enable rmcp's spans, so the tool span would have a parent if it
    // nested under rmcp's session span.
    let (exporter, _provider, _guard) = capture_spans(&format!("{DEFAULT_FILTER},rmcp=info"));
    let router = router();
    let session_id = open_session(&router).await;

    call_tool(
        &router,
        &session_id,
        None,
        json!({ "name": "list_devices" }),
    )
    .await;

    let span = tool_span(&exporter);
    assert_eq!(span.parent_span_id, SpanId::INVALID);
}

#[tokio::test]
async fn tool_call_joins_http_traceparent() {
    let (exporter, _provider, _guard) = capture_spans(DEFAULT_FILTER);
    let router = router();
    let session_id = open_session(&router).await;

    call_tool(
        &router,
        &session_id,
        Some(format!("00-{TRACE_ID}-{PARENT_SPAN_ID}-01")),
        json!({ "name": "list_devices" }),
    )
    .await;

    let span = tool_span(&exporter);
    assert_eq!(
        span.span_context.trace_id(),
        TraceId::from_hex(TRACE_ID).unwrap()
    );
    assert_eq!(
        span.parent_span_id,
        SpanId::from_hex(PARENT_SPAN_ID).unwrap()
    );
    assert!(span.parent_span_is_remote);
    assert!(span.span_context.is_sampled());
}

#[tokio::test]
async fn tool_call_joins_meta_traceparent() {
    let (exporter, _provider, _guard) = capture_spans(DEFAULT_FILTER);
    let router = router();
    let session_id = open_session(&router).await;

    call_tool(
        &router,
        &session_id,
        None,
        json!({
            "name": "list_devices",
            "_meta": { "traceparent": format!("00-{META_TRACE_ID}-{META_PARENT_SPAN_ID}-01") }
        }),
    )
    .await;

    let span = tool_span(&exporter);
    assert_eq!(
        span.span_context.trace_id(),
        TraceId::from_hex(META_TRACE_ID).unwrap()
    );
    assert_eq!(
        span.parent_span_id,
        SpanId::from_hex(META_PARENT_SPAN_ID).unwrap()
    );
    assert!(span.links.is_empty());
}

#[tokio::test]
async fn meta_traceparent_is_parent_and_http_traceparent_is_linked() {
    let (exporter, _provider, _guard) = capture_spans(DEFAULT_FILTER);
    let router = router();
    let session_id = open_session(&router).await;

    call_tool(
        &router,
        &session_id,
        Some(format!("00-{TRACE_ID}-{PARENT_SPAN_ID}-01")),
        json!({
            "name": "list_devices",
            "_meta": { "traceparent": format!("00-{META_TRACE_ID}-{META_PARENT_SPAN_ID}-01") }
        }),
    )
    .await;

    let span = tool_span(&exporter);
    assert_eq!(
        span.span_context.trace_id(),
        TraceId::from_hex(META_TRACE_ID).unwrap()
    );
    assert_eq!(
        span.parent_span_id,
        SpanId::from_hex(META_PARENT_SPAN_ID).unwrap()
    );
    assert_eq!(span.links.len(), 1);
    let link = &span.links[0].span_context;
    assert_eq!(link.trace_id(), TraceId::from_hex(TRACE_ID).unwrap());
    assert_eq!(link.span_id(), SpanId::from_hex(PARENT_SPAN_ID).unwrap());
}

#[tokio::test]
async fn failing_tool_call_has_error_status() {
    let (exporter, _provider, _guard) = capture_spans(DEFAULT_FILTER);
    let router = router();
    let session_id = open_session(&router).await;

    // No discovery has run, so the device cache is still pending and the call fails.
    let body = call_tool(
        &router,
        &session_id,
        None,
        json!({ "name": "list_devices" }),
    )
    .await;
    assert!(
        body.contains("-32603"),
        "expected an internal error: {body}"
    );

    let span = tool_span(&exporter);
    assert!(
        matches!(&span.status, Status::Error { description } if description.contains("discovery")),
        "unexpected status: {:?}",
        span.status
    );
    assert_eq!(attr(&span, "error.type").as_deref(), Some("-32603"));
    assert_eq!(
        attr(&span, "rpc.response.status_code").as_deref(),
        Some("-32603")
    );
}

#[tokio::test]
async fn failing_step_span_has_error_status() {
    let (exporter, _provider, _guard) = capture_spans(DEFAULT_FILTER);
    let router = router();
    let session_id = open_session(&router).await;

    // Nothing answers discovery on loopback, so `check_device` fails. Whether it
    // fails with a discovery error or with the device not found depends on the OS.
    call_tool(
        &router,
        &session_id,
        None,
        json!({
            "name": "get_device_state",
            "arguments": { "id": "plug-1", "ip": "127.0.0.1", "capability": { "type": "DeviceInfo" } },
        }),
    )
    .await;

    let span = exporter
        .get_finished_spans()
        .unwrap()
        .into_iter()
        .find(|span| span.name == "check_device")
        .expect("expected a check_device span");
    assert!(
        matches!(span.status, Status::Error { .. }),
        "unexpected status: {:?}",
        span.status
    );
    assert!(
        matches!(
            attr(&span, "error.type").as_deref(),
            Some("InternalDiscovery" | "DeviceNotFound")
        ),
        "unexpected error.type: {:?}",
        attr(&span, "error.type")
    );
}
