use anyhow::Result;
use std::time::Duration;
use tokio::sync::oneshot;

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(10);

use tapo_mcp::config::AppConfig;
use tapo_mcp::telemetry::init_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    let tracer_provider = init_tracing()?;

    let app_config = AppConfig::from_env()?;
    let listener = tokio::net::TcpListener::bind(&app_config.http_addr).await?;
    tracing::info!(addr = %app_config.http_addr, "Tapo MCP server listening");

    let app = tapo_mcp::router(app_config);

    // Channel to notify when the signal has fired, so we can start the timeout.
    let (signal_tx, signal_rx) = oneshot::channel::<()>();

    // Spawn the force-exit timeout: starts counting only after the signal fires.
    tokio::spawn(async move {
        // Wait until a shutdown signal has been received.
        let _ = signal_rx.await;
        tokio::time::sleep(SHUTDOWN_TIMEOUT).await;
        tracing::warn!("Graceful shutdown timed out after {SHUTDOWN_TIMEOUT:?}, forcing exit");
        std::process::exit(1);
    });

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(signal_tx))
        .await?;

    if let Some(provider) = tracer_provider {
        match tokio::task::spawn_blocking(move || provider.shutdown()).await {
            Ok(Err(err)) => tracing::warn!("Failed to shutdown tracer provider: {err}"),
            Err(err) => tracing::warn!("Tracer shutdown task panicked: {err}"),
            Ok(Ok(())) => {}
        }
    }

    Ok(())
}

async fn shutdown_signal(signal_tx: oneshot::Sender<()>) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!(
        "Shutdown signal received, waiting up to {SHUTDOWN_TIMEOUT:?} for graceful shutdown"
    );

    // Start the force-exit timeout.
    let _ = signal_tx.send(());
}
