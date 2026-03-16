use log::{debug, info, trace};
use serde_json::Value;
use std::net::{IpAddr, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context as TaskContext, Poll};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::Duration;
use tokio_stream::Stream;

use super::aes_discovery_query_generator::AesDiscoveryQueryGenerator;
use super::discovery_raw_result::DiscoveryRawResult;
use crate::DiscoveryError;

// Attempts discovery every 3 seconds.
const DISCOVERY_INTERVAL: Duration = Duration::from_secs(3);

/// Low-level UDP discovery that broadcasts queries and yields the [`IpAddr`] of each
/// responding device. Does not perform device login or info fetching — see
/// [`DeviceDiscovery`](super::DeviceDiscovery) for the higher-level stream that
/// wraps this and produces [`DiscoveryResult`](super::DiscoveryResult) items.
#[cfg_attr(not(feature = "debug"), allow(unreachable_pub))]
pub struct DeviceDiscoveryRaw {
    rx: Receiver<Option<Result<DiscoveryRawResult, DiscoveryError>>>,
}

impl DeviceDiscoveryRaw {
    /// Creates a new raw discovery stream targeting the given IP address.
    #[cfg_attr(not(feature = "debug"), allow(unreachable_pub))]
    pub async fn new(target_ip: IpAddr, timeout: Duration) -> anyhow::Result<Self> {
        let target = SocketAddr::new(target_ip, 20002);

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
                Self::receive_discovery_response(transport, target, seen_addrs, tx.clone()),
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
        tx: mpsc::Sender<Option<Result<DiscoveryRawResult, DiscoveryError>>>,
    ) {
        let error_handling_tx = tx.clone();
        let ip = target.ip().to_string();

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
            let _ = error_handling_tx
                .send(Some(Err(DiscoveryError {
                    ip,
                    source: e.into(),
                })))
                .await;
        }
    }

    async fn receive_discovery_response(
        transport: Arc<UdpSocket>,
        target: SocketAddr,
        seen_addrs: Arc<Mutex<Vec<SocketAddr>>>,
        tx: mpsc::Sender<Option<Result<DiscoveryRawResult, DiscoveryError>>>,
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

                    let message = if size > 16 {
                        let raw = String::from_utf8_lossy(&buf[16..size]);
                        debug!("Received discovery response from {addr:?}: {raw}");
                        serde_json::from_str(&raw).unwrap_or(Value::Null)
                    } else {
                        debug!("Received discovery response from {addr:?} (no payload)");
                        Value::Null
                    };

                    let _ = tx
                        .send(Some(Ok(DiscoveryRawResult {
                            ip: addr.ip(),
                            message,
                        })))
                        .await;

                    if addr.ip() == target.ip() {
                        debug!("Target found, stopping raw discovery responses");
                        let _ = tx.send(None).await;
                        break;
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    let error =
                        anyhow::Error::from(e).context("Failed to receive discovery response");
                    tx.send(Some(Err(DiscoveryError {
                        ip: target.ip().to_string(),
                        source: error.into(),
                    })))
                    .await
                    .ok();
                    break;
                }
            }
        }
    }
}

impl Stream for DeviceDiscoveryRaw {
    type Item = Result<DiscoveryRawResult, DiscoveryError>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
    ) -> Poll<Option<Result<DiscoveryRawResult, DiscoveryError>>> {
        match Pin::new(&mut self.rx).poll_recv(cx) {
            Poll::Ready(result) => match result {
                Some(result) => Poll::Ready(result),
                None => {
                    trace!("Raw discovery stream closed");
                    Poll::Ready(None)
                }
            },
            Poll::Pending => Poll::Pending,
        }
    }
}
