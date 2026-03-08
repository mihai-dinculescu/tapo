use anyhow::Result;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig as _;
use opentelemetry_sdk::Resource;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const OTEL_ENDPOINT_ENV: &str = "OTEL_EXPORTER_OTLP_ENDPOINT";

pub fn init_tracing() -> Result<Option<opentelemetry_sdk::trace::SdkTracerProvider>> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        // safe: hardcoded filter directive always parses successfully.
        .unwrap_or_else(|_| "tapo_mcp=info".parse().unwrap());

    if let Ok(endpoint) = std::env::var(OTEL_ENDPOINT_ENV) {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .with_endpoint(endpoint)
            .build()?;

        let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_batch_exporter(exporter)
            .with_resource(Resource::builder().with_service_name("tapo-mcp").build())
            .build();

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
