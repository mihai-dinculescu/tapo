use std::future::Future;

use anyhow::Result;
use axum::http::HeaderMap;
use axum::http::request::Parts;
use opentelemetry::propagation::Extractor;
use opentelemetry::trace::{TraceContextExt as _, TracerProvider as _};
use opentelemetry::{Context, global};
use opentelemetry_otlp::WithExportConfig as _;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolRequestParams, CallToolResponse, MetaObject};
use rmcp::service::{RequestContext, RoleServer};
use tracing::field::Empty;
use tracing::{Instrument as _, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt as _;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::errors::TapoMcpError;

const OTEL_ENDPOINT_ENV: &str = "OTEL_EXPORTER_OTLP_ENDPOINT";

/// The `EnvFilter` directive used when `RUST_LOG` is unset. It also decides
/// which spans are exported.
pub const DEFAULT_FILTER: &str = "tapo_mcp=info";

pub fn init_tracing() -> Result<Option<opentelemetry_sdk::trace::SdkTracerProvider>> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        // safe: hardcoded filter directive always parses successfully.
        .unwrap_or_else(|_| DEFAULT_FILTER.parse().unwrap());

    if let Ok(endpoint) = std::env::var(OTEL_ENDPOINT_ENV) {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .with_endpoint(endpoint)
            .build()?;

        let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_batch_exporter(exporter)
            .with_resource(Resource::builder().with_service_name("tapo-mcp").build())
            .build();

        global::set_text_map_propagator(TraceContextPropagator::new());

        let tracer = tracer_provider.tracer("tapo-mcp");
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .with(otel_layer)
            .try_init()?;

        return Ok(Some(tracer_provider));
    }

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    Ok(None)
}

/// Opens the server span for one `tools/call` request, following the OpenTelemetry
/// MCP semantic conventions.
///
/// The span is a tracing root (`parent: None`) so it never nests under rmcp's
/// long-lived session span. Its OpenTelemetry parent is the W3C trace context in
/// the request's `params._meta`, as the conventions require; the context in the
/// headers of the HTTP request that carried the call is then only linked. Without
/// `_meta` context the HTTP context is the parent, and without either the span
/// starts a new trace.
///
/// The span is named after the tool, and carries `gen_ai.tool.name`,
/// `tapo.device.id` and `tapo.device.ip`, only when
/// `known_tool` is set, so a client sending made-up tool names can't put
/// arbitrary strings in the trace.
pub(crate) fn tool_call_span(
    request: &CallToolRequestParams,
    known_tool: bool,
    context: &RequestContext<RoleServer>,
) -> Span {
    let tool_name = request.name.as_ref();
    let span_name = if known_tool {
        format!("tools/call {tool_name}")
    } else {
        "tools/call".to_string()
    };
    let span = tracing::info_span!(
        parent: None,
        "tools/call",
        otel.name = %span_name,
        otel.kind = "server",
        otel.status_code = Empty,
        otel.status_description = Empty,
        mcp.method.name = "tools/call",
        mcp.session.id = Empty,
        mcp.protocol.version = Empty,
        gen_ai.operation.name = "execute_tool",
        gen_ai.tool.name = Empty,
        jsonrpc.request.id = %context.id,
        network.protocol.name = "http",
        network.transport = "tcp",
        "error.type" = Empty,
        rpc.response.status_code = Empty,
        tapo.device.id = Empty,
        tapo.device.ip = Empty,
    );

    if known_tool {
        span.record("gen_ai.tool.name", tool_name);
    }

    if let Some(version) = context.protocol_version() {
        span.record("mcp.protocol.version", version.as_str());
    }

    // Every device tool takes the target as `id` and `ip` arguments.
    if known_tool && let Some(arguments) = &request.arguments {
        for (key, field) in [("id", "tapo.device.id"), ("ip", "tapo.device.ip")] {
            if let Some(value) = arguments.get(key).and_then(|value| value.as_str()) {
                span.record(field, value);
            }
        }
    }

    let parts = context.extensions.get::<Parts>();
    if let Some(session_id) = parts
        .and_then(|parts| parts.headers.get("mcp-session-id"))
        .and_then(|value| value.to_str().ok())
    {
        span.record("mcp.session.id", session_id);
    }

    let meta_cx = extract_remote_context(&MetaExtractor(&context.meta));
    let http_cx = parts.and_then(|parts| extract_remote_context(&HeaderExtractor(&parts.headers)));
    match (meta_cx, http_cx) {
        (Some(meta_cx), http_cx) => {
            // Fails only when no OpenTelemetry layer is installed, which is fine.
            let _ = span.set_parent(meta_cx);
            if let Some(http_cx) = http_cx {
                span.add_link(http_cx.span().span_context().clone());
            }
        }
        (None, Some(http_cx)) => {
            let _ = span.set_parent(http_cx);
        }
        (None, None) => {}
    }

    span
}

/// Records the outcome of a tool call on its span: the JSON-RPC error code and
/// message for a protocol error, or `tool_error` and the first text content for a
/// result with `is_error` set.
pub(crate) fn record_tool_outcome(span: &Span, result: &Result<CallToolResponse, McpError>) {
    match result {
        Ok(CallToolResponse::Complete(result)) if result.is_error == Some(true) => {
            span.record("error.type", "tool_error");
            span.record("otel.status_code", "ERROR");
            if let Some(text) = result.content.iter().find_map(|block| block.as_text()) {
                span.record("otel.status_description", text.text.as_str());
            }
        }
        Ok(_) => {}
        Err(error) => {
            let code = error.code.0.to_string();
            span.record("error.type", code.as_str());
            span.record("rpc.response.status_code", code.as_str());
            span.record("otel.status_code", "ERROR");
            span.record("otel.status_description", error.message.as_ref());
        }
    }
}

/// Runs one step of a tool call inside `span`, marking the span as failed when
/// the step returns an error. `span` must declare empty `otel.status_code` and
/// `error.type` fields; `error.type` gets the error's variant name.
pub(crate) async fn run_in_span<T>(
    span: Span,
    future: impl Future<Output = Result<T, TapoMcpError>>,
) -> Result<T, TapoMcpError> {
    let result = future.instrument(span.clone()).await;
    if let Err(error) = &result {
        span.record("otel.status_code", "ERROR");
        span.record("error.type", <&'static str>::from(error));
    }
    result
}

/// Extracts a remote span context with the global propagator, ignoring the
/// ambient context so a carrier without `traceparent` never yields the
/// session's own span.
fn extract_remote_context(extractor: &dyn Extractor) -> Option<Context> {
    let cx = global::get_text_map_propagator(|propagator| {
        propagator.extract_with_context(&Context::new(), extractor)
    });
    cx.span().span_context().is_valid().then_some(cx)
}

struct HeaderExtractor<'a>(&'a HeaderMap);

impl Extractor for HeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|key| key.as_str()).collect()
    }
}

/// Reads the unprefixed `traceparent`/`tracestate` keys that SEP-414 reserves in
/// a request's `_meta`.
struct MetaExtractor<'a>(&'a MetaObject);

impl Extractor for MetaExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.as_str())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(String::as_str).collect()
    }
}
