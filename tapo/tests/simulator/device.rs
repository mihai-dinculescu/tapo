//! The device itself: built from a fixture, started on a loopback address of
//! its own, and reachable until it is dropped. It answers discovery only
//! when asked to, and takes its requests over TLS when its fixture says so.

use std::io::ErrorKind;
use std::net::Ipv4Addr;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use env_logger::Env;
use tokio::net::{TcpListener, UdpSocket};
use tokio::task::JoinHandle;

use super::fixture::Fixture;
use super::klap::Klap;
use super::tpap::Tpap;
use super::{discovery, http, tls};

/// The port that Tapo devices answer discovery on. The client always asks
/// on it, so every device takes it on an address of its own.
const DISCOVERY_PORT: u16 = 20002;
/// The account that the device belongs to.
const USERNAME: &str = "tapo-username@example.com";
const PASSWORD: &str = "hunter2";

/// The last octet of the address of the most recently started device.
static LAST_DEVICE: AtomicU8 = AtomicU8::new(0);

/// A device that is not running yet.
pub struct SimulatedDevice {
    fixture: Fixture,
}

/// A device in the mode that its fixture announces.
pub enum Device {
    Tpap(Box<Tpap>),
    Klap(Klap),
}

impl SimulatedDevice {
    /// The device that the fixture file at `fixture` describes.
    pub fn new(fixture: impl AsRef<Path>) -> Self {
        init_logging();

        Self {
            fixture: Fixture::load(fixture.as_ref()),
        }
    }

    /// Starts answering requests on the loopback address, but not discovery.
    pub async fn start(self) -> RunningDevice {
        self.start_device(device_ip(), None).await
    }

    /// Starts answering requests and discovery on the loopback address.
    pub async fn start_with_discovery(self) -> RunningDevice {
        let ip = device_ip();
        let discovery_socket = bind_discovery_socket(ip).await;

        self.start_device(ip, Some(discovery_socket)).await
    }

    async fn start_device(
        self,
        ip: Ipv4Addr,
        discovery_socket: Option<UdpSocket>,
    ) -> RunningDevice {
        let listener = TcpListener::bind((ip, 0))
            .await
            .expect("an ephemeral port is free");
        let port = listener.local_addr().expect("the listener is bound").port();

        let fixture = self.fixture.at(&ip.to_string(), port);
        let announcement = fixture.discovery_response().clone();
        let tls = fixture.tpap_tls().then(tls::acceptor);
        let device = match fixture.auth_protocol() {
            "TPAP" => Device::Tpap(Box::new(Tpap::new(PASSWORD, fixture))),
            "KLAP" => Device::Klap(Klap::new(USERNAME, PASSWORD, fixture)),
            other => unimplemented!("the simulator does not speak {other}"),
        };

        let mut tasks = vec![tokio::spawn(http::serve(listener, tls, Arc::new(device)))];
        if let Some(discovery_socket) = discovery_socket {
            tasks.push(tokio::spawn(discovery::serve(
                discovery_socket,
                announcement,
            )));
        }

        RunningDevice { ip, port, tasks }
    }
}

/// A device that answers until it is dropped.
pub struct RunningDevice {
    ip: Ipv4Addr,
    port: u16,
    tasks: Vec<JoinHandle<()>>,
}

impl RunningDevice {
    /// The address to discover the device at.
    pub fn ip(&self) -> String {
        self.ip.to_string()
    }

    /// The address to log in to the device at, with the port that it takes
    /// requests on.
    pub fn address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    /// The account that the device belongs to.
    pub fn username(&self) -> &str {
        USERNAME
    }

    /// The password of that account.
    pub fn password(&self) -> &str {
        PASSWORD
    }
}

impl Drop for RunningDevice {
    fn drop(&mut self) {
        for task in &self.tasks {
            task.abort();
        }
    }
}

/// Captures the logs of the library and of the simulator for the running
/// test, which the harness prints only when the test fails. Everything at
/// debug and above is kept unless `RUST_LOG` says otherwise. Every test
/// starts a device, so only the first call in a binary installs the logger.
fn init_logging() {
    let _ = env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .is_test(true)
        .try_init();
}

/// A loopback address that no other device takes, in this test binary or
/// in another one that runs at the same time: the process id picks the
/// middle two octets, and the devices of the process count up in the last.
fn device_ip() -> Ipv4Addr {
    let device = LAST_DEVICE.fetch_add(1, Ordering::Relaxed);
    assert!(
        device < u8::MAX - 1,
        "a test binary starts at most 254 devices"
    );
    let device = device + 1;

    let [_, _, high, low] = std::process::id().to_be_bytes();
    Ipv4Addr::new(127, high, low, device)
}

/// Takes the discovery port on `ip`, which only a process outside the tests
/// can hold, such as one that listens for Tapo devices on every address.
async fn bind_discovery_socket(ip: Ipv4Addr) -> UdpSocket {
    match UdpSocket::bind((ip, DISCOVERY_PORT)).await {
        Ok(socket) => socket,
        Err(error) if error.kind() == ErrorKind::AddrInUse => panic!(
            "the discovery port {DISCOVERY_PORT} is taken by another process on this machine: {error}"
        ),
        Err(error) => {
            panic!("the discovery port {DISCOVERY_PORT} cannot be taken on {ip}: {error}")
        }
    }
}
