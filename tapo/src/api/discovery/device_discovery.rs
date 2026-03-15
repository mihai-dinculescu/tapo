use std::net::IpAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context as TaskContext, Poll};
use tokio::sync::RwLock;
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::Duration;
use tokio_stream::Stream;

pub use tokio_stream::StreamExt;

use super::device_discovery_raw::{DeviceDiscoveryRaw, DiscoveryRawResult};
use super::discovery_result::DiscoveryResult;
use crate::{ApiClient, DiscoveryError};

/// Device discovery process for Tapo devices.
pub struct DeviceDiscovery {
    rx: Receiver<Result<DiscoveryResult, DiscoveryError>>,
}

impl DeviceDiscovery {
    pub(crate) async fn new(
        client: ApiClient,
        target: impl Into<String>,
        timeout: Duration,
    ) -> anyhow::Result<Self> {
        let target: String = target.into();
        let target_ip: IpAddr = target.parse()?;
        let mut raw = DeviceDiscoveryRaw::new(target_ip, timeout).await?;

        let client: Arc<RwLock<ApiClient>> = Arc::new(RwLock::new(client));
        let (tx, rx) = mpsc::channel(1024);

        tokio::spawn(async move {
            use tokio_stream::StreamExt as _;

            while let Some(result) = raw.next().await {
                match result {
                    Ok(raw_result) => {
                        let client = client.clone();
                        let tx = tx.clone();
                        tokio::spawn(Self::process_discovery_response(client, raw_result, tx));
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                    }
                }
            }
        });

        Ok(Self { rx })
    }

    async fn process_discovery_response(
        client: Arc<RwLock<ApiClient>>,
        raw_result: DiscoveryRawResult,
        tx: mpsc::Sender<Result<DiscoveryResult, DiscoveryError>>,
    ) {
        let client = client.read().await.clone();
        let ip = raw_result.ip.to_string();

        let result = DiscoveryResult::new(client, raw_result.ip)
            .await
            .map_err(|source| DiscoveryError { ip, source });

        let _ = tx.send(result).await;
    }
}

impl Stream for DeviceDiscovery {
    type Item = Result<DiscoveryResult, DiscoveryError>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
    ) -> Poll<Option<Result<DiscoveryResult, DiscoveryError>>> {
        Pin::new(&mut self.rx).poll_recv(cx)
    }
}
