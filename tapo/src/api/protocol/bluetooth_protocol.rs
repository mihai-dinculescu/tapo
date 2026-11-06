//! A device in setup mode that is reached over Bluetooth LE: the scan for
//! one, the GATT link that carries TSLP packets, and TPAP over that link.

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, LazyLock, Mutex as StdMutex, PoisonError, Weak};
use std::time::Duration;

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::{
    Central as _, CentralEvent, Characteristic, Manager as _, Peripheral as _, ScanFilter,
    WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral, PeripheralId};
use log::{debug, trace};
use rand::RngExt as _;
use serde::de::DeserializeOwned;
use serde_json::json;
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio::time::Instant;
use tokio_stream::StreamExt as _;
use uuid::Uuid;

use crate::requests::TapoRequest;
use crate::responses::{TapoResponse, TapoResponseExt, validate_response};
use crate::{Error, TapoResponseError};

use super::tpap_cipher::TpapCipher;
use super::tpap_protocol::{
    DEFAULT_PASSCODE_TYPE, LoginResponse, RegisterResult, ShareResult, pake_share_error,
    parse_discover_response, prove, register_request, share_request,
};
use super::tpap_spake2p;
use super::tslp::{BUSINESS_CHANNEL, Kind, Packet};

/// The service that a device in setup mode advertises, and the one that
/// the Tapo app treats the same way.
const SETUP_SERVICES: [Uuid; 2] = [uuid_from_u16(0x8641), uuid_from_u16(0x8926)];
/// Requests are written to this characteristic.
const WRITE_CHARACTERISTIC: Uuid = uuid_from_u16(0x8883);
/// Replies are read from this characteristic, which reads empty while
/// nothing is pending.
const READ_CHARACTERISTIC: Uuid = uuid_from_u16(0x8882);
/// How often the reply characteristic is read, as the Tapo app does.
const READ_INTERVAL: Duration = Duration::from_millis(150);
/// The largest chunk that a request is written in.
const MAX_CHUNK_LENGTH: usize = 200;
/// The most reads that clear the rest of an earlier reply before a request.
const MAX_STALE_READS: usize = 32;
/// How many times a connection is tried, all within the client's timeout.
const CONNECT_ATTEMPTS: u32 = 3;
/// The longest wait for a device to advertise again after a disconnect,
/// after which it is connected to anyway. A scan can miss the
/// advertisement, and by then the operating system has closed the earlier
/// connection, which on Windows takes about 3 seconds.
const ADVERTISEMENT_WAIT: Duration = Duration::from_secs(10);

/// The protocol byte of the advertisement for TPAP over Bluetooth.
const PROTOCOL_TPAP: u8 = 4;
/// The flag of the advertisement for a device that is available for setup.
const FLAG_AVAILABLE: u8 = 0x01;

/// A new or factory-reset device that advertises setup over Bluetooth, as
/// found by [`ApiClient::discover_bluetooth_setup_devices`](crate::ApiClient::discover_bluetooth_setup_devices).
#[derive(Clone)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object))]
#[cfg_attr(feature = "python", derive(serde::Serialize))]
pub struct BluetoothSetupDevice {
    /// The name that the device advertises, as in `P110M_lgR0GOlb`.
    pub name: String,
    /// The model, from the name, as in `P110M`.
    pub model: String,
    /// The Bluetooth address, as in `98:BA:5F:A3:5F:07`. Empty where the
    /// operating system hides it, as macOS does.
    pub address: String,
    #[cfg_attr(feature = "python", serde(skip))]
    adapter: Adapter,
    #[cfg_attr(feature = "python", serde(skip))]
    id: PeripheralId,
    /// Shared by every [`BluetoothSetupDevice`] for the device, including
    /// those of later discoveries. Held while a link connects, exchanges or
    /// disconnects, so that a new link to the device waits for that instead
    /// of being cut off by it.
    #[cfg_attr(feature = "python", serde(skip))]
    state: Arc<Mutex<DeviceState>>,
}

/// The state of every device that a discovery returned and that is still
/// in use, so that a later discovery shares it.
static DEVICE_STATES: LazyLock<StdMutex<HashMap<PeripheralId, Weak<Mutex<DeviceState>>>>> =
    LazyLock::new(StdMutex::default);

struct DeviceState {
    peripheral: Peripheral,
    /// The generation of the newest link. Only that link may exchange and
    /// disconnect, so a new setup of the device takes over from an earlier
    /// one, as when a failed setup is retried while its handler is still
    /// alive.
    generation: u64,
    /// Whether the device was disconnected since it last advertised. A
    /// disconnect returns before the operating system closes the
    /// connection, which on Windows happens about 3 seconds later, and a
    /// connection made around then fails. The device advertises again only
    /// once the connection is closed, so the next connection waits for that.
    disconnected: bool,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl BluetoothSetupDevice {
    #[getter(name)]
    fn py_name(&self) -> &str {
        &self.name
    }

    #[getter(model)]
    fn py_model(&self) -> &str {
        &self.model
    }

    #[getter(address)]
    fn py_address(&self) -> &str {
        &self.address
    }
}

#[cfg(feature = "python")]
crate::impl_to_dict!(BluetoothSetupDevice);

impl fmt::Debug for BluetoothSetupDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BluetoothSetupDevice")
            .field("name", &self.name)
            .field("model", &self.model)
            .field("address", &self.address)
            .finish_non_exhaustive()
    }
}

/// Scans for `timeout` and returns the devices that advertise setup over
/// Bluetooth and are available for it.
///
/// # Errors
///
/// Returns an error if the host has no Bluetooth adapter or the scan fails.
pub(crate) async fn discover_bluetooth_setup_devices(
    timeout: Duration,
) -> Result<Vec<BluetoothSetupDevice>, Error> {
    let manager = Manager::new().await.map_err(bluetooth_error)?;
    let adapter = manager
        .adapters()
        .await
        .map_err(bluetooth_error)?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("The host has no Bluetooth adapter"))?;

    // The scan is not filtered by service, because not every platform
    // matches a filter against the service data that the device advertises.
    adapter
        .start_scan(ScanFilter::default())
        .await
        .map_err(bluetooth_error)?;
    tokio::time::sleep(timeout).await;
    let peripherals = adapter.peripherals().await.map_err(bluetooth_error);
    if let Err(error) = adapter.stop_scan().await {
        debug!("Stopping the Bluetooth scan failed: {error}");
    }

    let mut devices = Vec::new();
    for peripheral in peripherals? {
        let Ok(Some(properties)) = peripheral.properties().await else {
            continue;
        };
        let Some(service_data) = SETUP_SERVICES
            .iter()
            .find_map(|service| properties.service_data.get(service))
        else {
            continue;
        };
        debug!(
            "Found {:?} at {} with the setup service data {service_data:02X?}",
            properties.local_name, properties.address
        );
        if !is_available_for_setup(service_data) {
            continue;
        }

        let name = properties.local_name.unwrap_or_default();
        let address = if properties.address.into_inner() == [0; 6] {
            String::new()
        } else {
            properties.address.to_string()
        };

        devices.push(BluetoothSetupDevice {
            model: model_from_name(&name),
            name,
            address,
            adapter: adapter.clone(),
            id: peripheral.id(),
            state: device_state(peripheral),
        });
    }

    Ok(devices)
}

/// The state of the device of `peripheral`, which an earlier discovery
/// may have created.
fn device_state(peripheral: Peripheral) -> Arc<Mutex<DeviceState>> {
    let mut states = DEVICE_STATES.lock().unwrap_or_else(PoisonError::into_inner);
    states.retain(|_, state| state.strong_count() > 0);

    let id = peripheral.id();
    if let Some(state) = states.get(&id).and_then(Weak::upgrade) {
        // The peripheral of this discovery belongs to its adapter, which the
        // device now carries. An open connection keeps the one it was made
        // with.
        if let Ok(mut current) = state.try_lock()
            && (current.disconnected || current.generation == 0)
        {
            current.peripheral = peripheral;
        }
        return state;
    }
    let state = Arc::new(Mutex::new(DeviceState {
        peripheral,
        generation: 0,
        disconnected: false,
    }));
    states.insert(id, Arc::downgrade(&state));
    state
}

impl BluetoothSetupDevice {
    /// Scans until the device advertises, for up to [`ADVERTISEMENT_WAIT`]
    /// and before `deadline`, and returns its peripheral, or `current` if
    /// there is no newer one.
    async fn wait_for_advertisement(
        &self,
        current: &Peripheral,
        deadline: Instant,
    ) -> Result<Peripheral, Error> {
        debug!("Waiting for the device to advertise over Bluetooth again...");
        let until = deadline.min(Instant::now() + ADVERTISEMENT_WAIT);
        let scan = async {
            let mut events = self.adapter.events().await.map_err(bluetooth_error)?;
            self.adapter
                .start_scan(ScanFilter::default())
                .await
                .map_err(bluetooth_error)?;

            let advertised = tokio::time::timeout_at(until, async {
                while let Some(event) = events.next().await {
                    // The setup service data is in every advertisement of the
                    // device. Unlike other events, this one is never replayed
                    // for a device that the adapter already knows. BlueZ sends
                    // it only when the data changes, as it does after a
                    // disconnect.
                    if let CentralEvent::ServiceDataAdvertisement { id, .. } = &event
                        && *id == self.id
                    {
                        return Ok(true);
                    }
                }
                Err(Error::from(anyhow::anyhow!("The Bluetooth events ended")))
            })
            .await;

            if let Err(error) = self.adapter.stop_scan().await {
                debug!("Stopping the Bluetooth scan failed: {error}");
            }
            advertised.unwrap_or(Ok(false))
        };

        match scan.await {
            Ok(true) => {}
            Ok(false) if until < deadline => {
                debug!("The device did not advertise over Bluetooth in time, connecting anyway");
            }
            Ok(false) => {
                return Err(timeout_error(
                    "waiting for the device to advertise over Bluetooth",
                ));
            }
            // Like a missed advertisement, a failed scan does not rule out
            // that the device takes the connection.
            Err(error) => {
                debug!("Scanning for the device over Bluetooth failed, connecting anyway: {error}");
            }
        }

        // macOS forgets a peripheral when it disconnects, and knows it again,
        // as a new one, once it advertises.
        match self.adapter.peripheral(&self.id).await {
            Ok(peripheral) => Ok(peripheral),
            Err(error) => {
                debug!("Finding the device's Bluetooth peripheral again failed: {error}");
                Ok(current.clone())
            }
        }
    }
}

/// Whether the advertised `service_data` is that of a device that takes
/// TPAP over Bluetooth and is available for setup: byte 0 is the protocol
/// and byte 2 holds the flags.
fn is_available_for_setup(service_data: &[u8]) -> bool {
    matches!(service_data, [PROTOCOL_TPAP, _, flags, ..] if flags & FLAG_AVAILABLE != 0)
}

/// The model in a name such as `P110M_lgR0GOlb`.
fn model_from_name(name: &str) -> String {
    name.split_once('_')
        .map_or(name, |(model, _)| model)
        .to_string()
}

/// The GATT link to a device, which writes a packet and reads the reply
/// back, one exchange at a time.
struct BluetoothLink {
    write: Characteristic,
    read: Characteristic,
    timeout: Duration,
    /// The generation of the link, from [`DeviceState::generation`].
    generation: u64,
    device_state: Arc<Mutex<DeviceState>>,
    /// The runtime that the link connected on. The drop disconnects on it
    /// when it runs outside any runtime, as when Python frees the handler.
    runtime: Handle,
}

impl BluetoothLink {
    async fn connect(device: &BluetoothSetupDevice, timeout: Duration) -> Result<Self, Error> {
        let runtime = Handle::try_current()
            .map_err(|_| anyhow::anyhow!("Bluetooth needs a Tokio runtime"))?;
        let deadline = Instant::now() + timeout;

        // Waits for an earlier link to the device to finish what it is doing.
        let mut state = tokio::time::timeout_at(deadline, Arc::clone(&device.state).lock_owned())
            .await
            .map_err(|_| timeout_error("waiting for an earlier Bluetooth link to close"))?;

        // Takes over from an earlier link, which leaves the connection open
        // for this one to reuse but can no longer use or close it.
        state.generation += 1;
        let generation = state.generation;

        let (write, read) = match Self::open_with_retries(device, &mut state, deadline).await {
            Ok(characteristics) => characteristics,
            Err(error) => {
                // No link holds the connection, so no drop closes it. The
                // disconnect runs in the background, so that the error
                // returns at once, and holds the state, so that the next
                // connection waits for it instead of being cut off by it.
                runtime.spawn(async move {
                    disconnect(&state.peripheral, Instant::now() + timeout).await;
                    drop(state);
                });
                return Err(error);
            }
        };

        Ok(Self {
            write,
            read,
            timeout,
            generation,
            device_state: Arc::clone(&device.state),
            runtime,
        })
    }

    /// Connects to the device before `deadline` and returns its write and
    /// read characteristics, trying again when a connection comes up
    /// without them. After the last failed attempt, the connection is left
    /// for the caller to close.
    async fn open_with_retries(
        device: &BluetoothSetupDevice,
        state: &mut DeviceState,
        deadline: Instant,
    ) -> Result<(Characteristic, Characteristic), Error> {
        let mut attempt = 1;
        let mut last_error = None;
        loop {
            if state.disconnected {
                match device
                    .wait_for_advertisement(&state.peripheral, deadline)
                    .await
                {
                    Ok(peripheral) => state.peripheral = peripheral,
                    // The failed connection says more than the wait does.
                    Err(error) => return Err(last_error.unwrap_or(error)),
                }
                state.disconnected = false;
            }

            let error = match tokio::time::timeout_at(deadline, Self::open(&state.peripheral)).await
            {
                Ok(Ok(characteristics)) => return Ok(characteristics),
                Ok(Err(error)) => error,
                Err(_) => timeout_error("connecting over Bluetooth"),
            };
            debug!("Connecting over Bluetooth failed, attempt {attempt}: {error}");
            let last = attempt == CONNECT_ATTEMPTS || Instant::now() >= deadline;

            state.disconnected = true;
            if last {
                return Err(error);
            }
            disconnect(&state.peripheral, deadline).await;
            last_error = Some(error);
            attempt += 1;
        }
    }

    /// Connects to `peripheral` and returns its write and read
    /// characteristics.
    async fn open(peripheral: &Peripheral) -> Result<(Characteristic, Characteristic), Error> {
        if !peripheral.is_connected().await.map_err(bluetooth_error)? {
            debug!("Connecting over Bluetooth...");
            peripheral.connect().await.map_err(bluetooth_error)?;
        }
        peripheral
            .discover_services()
            .await
            .map_err(bluetooth_error)?;

        let characteristic = |uuid: Uuid| {
            peripheral
                .characteristics()
                .into_iter()
                .find(|c| c.uuid == uuid)
                .ok_or_else(|| {
                    Error::from(anyhow::anyhow!(
                        "The device has no Bluetooth characteristic {uuid}"
                    ))
                })
        };

        Ok((
            characteristic(WRITE_CHARACTERISTIC)?,
            characteristic(READ_CHARACTERISTIC)?,
        ))
    }

    /// Writes `packet` and returns the reply that has its sequence number.
    async fn exchange(&self, packet: &Packet) -> Result<Packet, Error> {
        // Held for the whole exchange, so that a newer link waits for it.
        let state = self.device_state.lock().await;
        if state.generation != self.generation {
            return Err(anyhow::anyhow!(
                "A newer Bluetooth setup of the device took over from this one"
            )
            .into());
        }

        tokio::time::timeout(
            self.timeout,
            self.exchange_unguarded(&state.peripheral, packet),
        )
        .await
        .map_err(|_| timeout_error("waiting for the device's Bluetooth reply"))?
    }

    async fn exchange_unguarded(
        &self,
        peripheral: &Peripheral,
        packet: &Packet,
    ) -> Result<Packet, Error> {
        self.discard_stale_reply(peripheral).await?;

        let encoded = packet.encode();
        let chunk_length =
            MAX_CHUNK_LENGTH.min(usize::from(peripheral.mtu()).saturating_sub(3).max(20));
        for chunk in encoded.chunks(chunk_length) {
            peripheral
                .write(&self.write, chunk, WriteType::WithResponse)
                .await
                .map_err(bluetooth_error)?;
        }

        // A reply to an earlier request that gave up, for example after a
        // timeout, can still arrive after this one was written.
        loop {
            let reply = self.read_reply(peripheral).await?;
            if reply.sequence == packet.sequence {
                return Ok(reply);
            }
            debug!(
                "Discarding a Bluetooth reply with the sequence number {}, while waiting for {}",
                reply.sequence, packet.sequence
            );
        }
    }

    /// Reads the next whole packet from the reply characteristic.
    async fn read_reply(&self, peripheral: &Peripheral) -> Result<Packet, Error> {
        let mut reply = Vec::new();
        loop {
            let data = peripheral.read(&self.read).await.map_err(bluetooth_error)?;

            if data.is_empty() {
                tokio::time::sleep(READ_INTERVAL).await;
                continue;
            }
            reply.extend_from_slice(&data);

            if let Some(total_length) = Packet::total_length(&reply)
                && reply.len() >= total_length
            {
                if reply.len() > total_length {
                    debug!(
                        "Dropping {} bytes after the Bluetooth reply",
                        reply.len() - total_length
                    );
                }
                return Packet::decode(&reply[..total_length]);
            }
        }
    }

    /// Reads away what is left of a reply that an earlier exchange gave up
    /// on, for example after a timeout, so that it is not taken for the
    /// start of the next reply.
    async fn discard_stale_reply(&self, peripheral: &Peripheral) -> Result<(), Error> {
        let mut discarded = 0;
        for _ in 0..MAX_STALE_READS {
            let data = peripheral.read(&self.read).await.map_err(bluetooth_error)?;
            if data.is_empty() {
                break;
            }
            discarded += data.len();
        }
        if discarded > 0 {
            debug!("Discarded {discarded} bytes of an earlier Bluetooth reply");
        }

        Ok(())
    }
}

impl Drop for BluetoothLink {
    /// Disconnects in the background, so that the device is not held after
    /// the session ends, such as after a failed login or join.
    fn drop(&mut self) {
        let timeout = self.timeout;
        let device_state = Arc::clone(&self.device_state);
        // The runtime of the drop, if there is one, since the one that the
        // link connected on may have shut down since.
        let runtime = Handle::try_current().unwrap_or_else(|_| self.runtime.clone());
        let generation = self.generation;
        // A link that connects before this task runs takes over the
        // connection, so this does not cut it off.
        runtime.spawn(async move {
            let mut state = device_state.lock().await;
            if state.generation == generation {
                disconnect(&state.peripheral, Instant::now() + timeout).await;
                state.disconnected = true;
            }
        });
    }
}

/// Disconnects from `peripheral`. The link may already be gone, so a
/// failure is only logged. The wait ends at `deadline`, so that a hung
/// disconnect does not hold up the next connection forever.
async fn disconnect(peripheral: &Peripheral, deadline: Instant) {
    match tokio::time::timeout_at(deadline, peripheral.disconnect()).await {
        Ok(Ok(())) => {}
        Ok(Err(error)) => debug!("Disconnecting over Bluetooth failed: {error}"),
        Err(_) => debug!("Disconnecting over Bluetooth timed out"),
    }
}

struct BluetoothSession {
    cipher: TpapCipher,
    /// The sequence number of the last business request. It is held for
    /// the whole exchange, so that requests go out in sequence order.
    seq: Mutex<i32>,
}

/// TPAP over Bluetooth to a device in setup mode, which takes the default
/// passcode derived from its MAC address.
pub(super) struct BluetoothProtocol {
    link: BluetoothLink,
    session: Option<BluetoothSession>,
}

impl BluetoothProtocol {
    pub async fn connect(device: &BluetoothSetupDevice, timeout: Duration) -> Result<Self, Error> {
        Ok(Self {
            link: BluetoothLink::connect(device, timeout).await?,
            session: None,
        })
    }

    /// Logs in with the default passcode.
    ///
    /// # Errors
    ///
    /// Returns an error if the device cannot be reached, if it is not in
    /// setup mode, or if it does not take the login.
    // Of the login, only the discover answer and the error codes are logged,
    // and nothing that is derived from the passcode.
    pub async fn login(&mut self) -> Result<(), Error> {
        self.session = None;

        debug!("Performing discover...");
        let discover: serde_json::Value = self
            .auth_request(&json!({
                "method": "login",
                "params": { "sub_method": "discover" },
            }))
            .await?;
        debug!("Device responded with: {discover}");

        let Some((info, mac)) = parse_discover_response(&discover)? else {
            return Err(Error::unsupported_setup_tpap(format!(
                "discover: {discover}"
            )));
        };
        if !info.takes_default_passcode() {
            return Err(Error::Tapo(
                TapoResponseError::bluetooth_setup_hash_mismatch(),
            ));
        }
        let Some(mac) = mac else {
            return Err(Error::unsupported_setup_tpap(
                "the discover answer has no `mac`".to_string(),
            ));
        };
        let passcode = tpap_spake2p::default_passcode(&mac)?;

        let user_random: [u8; 32] = rand::rng().random();
        debug!("Performing pake_register...");
        let registration = self
            .auth_request::<LoginResponse>(&register_request(DEFAULT_PASSCODE_TYPE, &user_random))
            .await?
            .into_result::<RegisterResult>("pake_register")?
            .validate()?;
        debug!("pake_register OK");

        let proof = prove(passcode, registration, user_random).await?;

        let dac_nonce: Option<[u8; 32]> = info.has_dac().then(|| rand::rng().random());
        debug!("Performing pake_share...");
        let share_result = self
            .auth_request::<LoginResponse>(&share_request(
                &proof,
                dac_nonce.as_ref().map(|n| n.as_slice()),
            ))
            .await?
            .into_result::<ShareResult>("pake_share")
            .map_err(pake_share_error)?;
        share_result.verify(&proof)?;

        self.session = Some(BluetoothSession {
            cipher: TpapCipher::new(&proof.shared_key, share_result.start_sequence)?,
            seq: Mutex::new(share_result.start_sequence),
        });

        Ok(())
    }

    pub async fn execute_request<R>(&self, request: TapoRequest) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Bluetooth session not initialized (login first)"))?;

        let request_string = serde_json::to_string(&request)?;
        debug!("Request: {}", request.for_log(&request_string));

        // The first business request takes the sequence number after the
        // one that the login started at.
        let mut last_seq = session.seq.lock().await;
        let seq = last_seq.wrapping_add(1);
        *last_seq = seq;
        let payload = session.cipher.encrypt_at(seq, request_string.as_bytes())?;

        let reply = self
            .link
            .exchange(&Packet {
                direction: 2,
                kind: Kind::Business,
                channel: BUSINESS_CHANNEL.to_string(),
                sequence: seq,
                payload,
            })
            .await?;

        if reply.kind != Kind::Business {
            let response: LoginResponse = serde_json::from_slice(&reply.payload)?;
            response.into_result::<serde_json::Value>("request")?;
            return Err(Error::Tapo(TapoResponseError::ResponseError {
                description: "The device answered a request with a login reply".to_string(),
            }));
        }
        let response_decrypted = session
            .cipher
            .decrypt_at(reply.sequence, &reply.payload)
            .map_err(|error| {
                Error::Tapo(TapoResponseError::ResponseError {
                    description: format!("The response could not be decrypted: {error}"),
                })
            })?;
        trace!("Device responded with (raw): {response_decrypted}");

        let response: TapoResponse<R> = serde_json::from_str(&response_decrypted)?;
        debug!("Device responded with: {response:?}");

        validate_response(response.error_code)?;

        Ok(response.result)
    }

    /// Sends a login request and returns the reply's JSON.
    async fn auth_request<T: DeserializeOwned>(
        &self,
        body: &serde_json::Value,
    ) -> Result<T, Error> {
        // A login request takes a random sequence number. It is drawn in its
        // own statement, because the generator must not live across an await.
        let sequence: i32 = rand::rng().random();
        let reply = self
            .link
            .exchange(&Packet {
                direction: 1,
                kind: Kind::Auth,
                channel: String::new(),
                sequence,
                payload: serde_json::to_vec(body)?,
            })
            .await?;

        if reply.kind != Kind::Auth {
            return Err(Error::Tapo(TapoResponseError::ResponseError {
                description: "The device answered a login request with a business reply"
                    .to_string(),
            }));
        }

        Ok(serde_json::from_slice(&reply.payload)?)
    }
}

fn bluetooth_error(error: btleplug::Error) -> Error {
    anyhow::anyhow!("Bluetooth: {error}").into()
}

fn timeout_error(what: &str) -> Error {
    anyhow::anyhow!("Timed out {what}").into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn available_tpap_device_is_available_for_setup() {
        // What the P110M advertises.
        assert!(is_available_for_setup(&[0x04, 0x81, 0x09, 0x03, 0x00]));
    }

    #[test]
    fn owned_or_other_protocol_device_is_not_available_for_setup() {
        assert!(!is_available_for_setup(&[0x04, 0x81, 0x02, 0x03, 0x00]));
        assert!(!is_available_for_setup(&[0x03, 0x81, 0x09, 0x03, 0x00]));
        assert!(!is_available_for_setup(&[0x04, 0x81]));
        assert!(!is_available_for_setup(&[]));
    }

    #[test]
    fn model_is_the_name_before_the_underscore() {
        assert_eq!(model_from_name("P110M_lgR0GOlb"), "P110M");
        assert_eq!(model_from_name("P110M"), "P110M");
    }
}
