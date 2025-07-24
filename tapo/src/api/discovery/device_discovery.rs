use log::{Level, debug, info, log_enabled, trace};
use std::net::{IpAddr, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context as TaskContext, Poll};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{self, Receiver};
use tokio::sync::{Mutex, RwLock};
use tokio::time::Duration;
use tokio_stream::Stream;

pub use tokio_stream::StreamExt;

use super::aes_discovery_query_generator::AesDiscoveryQueryGenerator;
use super::discovery_result::DiscoveryResult;
use crate::{ApiClient, Error};

// Attempts discovery every 3 seconds.
const DISCOVERY_INTERVAL: Duration = Duration::from_secs(3);

/// Device discovery process for Tapo devices.
pub struct DeviceDiscovery {
    rx: Receiver<Option<Result<DiscoveryResult, Error>>>,
}

impl DeviceDiscovery {
    pub(crate) async fn new(
        client: ApiClient,
        target: impl Into<String>,
        timeout: Duration,
    ) -> anyhow::Result<Self> {
        let target = SocketAddr::new(target.into().parse()?, 20002);

        let bind_address = match target.ip() {
            IpAddr::V4(_) => "0.0.0.0:0", // IPv4
            IpAddr::V6(_) => "[::]:0",    // IPv6
        };

        let transport = UdpSocket::bind(bind_address).await?;
        transport.set_broadcast(true)?;
        let transport = Arc::new(transport);

        let (tx, rx) = mpsc::channel(1024);
        let seen_addrs = Arc::new(Mutex::new(vec![]));

        let discovery_transport = transport.clone();
        let discovery_seen_addrs = seen_addrs.clone();
        let discovery_tx = tx.clone();

        let client: Arc<RwLock<ApiClient>> = Arc::new(RwLock::new(client));

        tokio::spawn(async move {
            let result = tokio::time::timeout(
                timeout,
                Self::send_discovery_query(
                    discovery_transport,
                    target,
                    discovery_seen_addrs,
                    discovery_tx.clone(),
                ),
            )
            .await;

            if result.is_err() {
                trace!("Discovery query timed out");
            }
        });

        tokio::spawn(async move {
            let result = tokio::time::timeout(
                timeout,
                Self::receive_discovery_response(client, transport, target, seen_addrs, tx.clone()),
            )
            .await;

            if result.is_err() {
                trace!("Discovery response timed out");
            }
        });

        Ok(Self { rx })
    }

    async fn send_discovery_query(
        transport: Arc<UdpSocket>,
        target: SocketAddr,
        seen_addrs: Arc<Mutex<Vec<SocketAddr>>>,
        tx: mpsc::Sender<Option<Result<DiscoveryResult, Error>>>,
    ) {
        let error_handling_tx = tx.clone();

        let result = async move {
            let aes_discovery_query = AesDiscoveryQueryGenerator::new()?.generate()?;

            loop {
                if tx.is_closed() {
                    info!("Channel closed, stopping discovery queries");
                    break;
                }

                let seen_addrs = seen_addrs.lock().await;
                if seen_addrs.contains(&target) {
                    trace!("Target found, stopping discovery queries");
                    break;
                }
                drop(seen_addrs);

                transport.send_to(&aes_discovery_query, target).await?;

                tokio::time::sleep(DISCOVERY_INTERVAL).await;
            }

            trace!("Discovery queries finished");

            Ok::<_, anyhow::Error>(())
        }
        .await;

        if let Err(e) = result {
            let _ = error_handling_tx.send(Some(Err(e.into()))).await;
        }
    }

    async fn receive_discovery_response(
        client: Arc<RwLock<ApiClient>>,
        transport: Arc<UdpSocket>,
        target: SocketAddr,
        seen_addrs: Arc<Mutex<Vec<SocketAddr>>>,
        tx: mpsc::Sender<Option<Result<DiscoveryResult, Error>>>,
    ) {
        loop {
            if tx.is_closed() {
                trace!("Channel closed, stopping discovery responses");
                break;
            }

            if tokio::time::timeout(Duration::from_millis(100), transport.readable())
                .await
                .is_err()
            {
                continue;
            }

            let mut buf = [0; 2048];

            // Try to recv data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match transport.try_recv_from(&mut buf) {
                Ok((size, addr)) => {
                    let mut seen_addrs = seen_addrs.lock().await;
                    if seen_addrs.contains(&addr) {
                        continue;
                    } else {
                        seen_addrs.push(addr);
                    }
                    drop(seen_addrs);

                    if size > 16 && log_enabled!(Level::Debug) {
                        debug!("Received discovery response from {addr:?}");
                        let message: String = String::from_utf8_lossy(&buf[16..]).to_string();
                        debug!("Discovery response message: {message}");
                    }

                    tokio::spawn(Self::process_discovery_response(
                        client.clone(),
                        addr.ip(),
                        target.ip(),
                        tx.clone(),
                    ));
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    let error =
                        anyhow::Error::from(e).context("Failed to receive discovery response");
                    tx.send(Some(Err(error.into()))).await.ok();
                    break;
                }
            }
        }
    }

    async fn process_discovery_response(
        client: Arc<RwLock<ApiClient>>,
        ip_addr: IpAddr,
        target: IpAddr,
        tx: mpsc::Sender<Option<Result<DiscoveryResult, Error>>>,
    ) {
        let client = client.read().await.clone();

        let result = DiscoveryResult::new(client, ip_addr).await;

        let _ = tx.send(Some(result)).await;

        if ip_addr == target {
            debug!("Target found, stopping discovery responses");
            let _ = tx.send(None).await;
        }
    }
}

impl Stream for DeviceDiscovery {
    type Item = Result<DiscoveryResult, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
    ) -> Poll<Option<Result<DiscoveryResult, Error>>> {
        match Pin::new(&mut self.rx).poll_recv(cx) {
            Poll::Ready(result) => match result {
                Some(result) => Poll::Ready(result),
                None => {
                    trace!("Discovery stream closed");
                    Poll::Ready(None)
                }
            },
            Poll::Pending => Poll::Pending,
        }
    }
}
