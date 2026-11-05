//! A device in setup mode that is reached over Bluetooth LE: the scan for
//! one, the GATT link that carries TSLP packets, and TPAP over that link.

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::{
    Central as _, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Manager, Peripheral};
use log::{debug, trace};
use rand::RngExt as _;
use serde::de::DeserializeOwned;
use serde_json::json;
use tokio::sync::Mutex;
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

/// The protocol byte of the advertisement for TPAP over Bluetooth.
const PROTOCOL_TPAP: u8 = 4;
/// The flag of the advertisement for a device that is available for setup.
const FLAG_AVAILABLE: u8 = 0x01;

/// A new or factory-reset device that advertises setup over Bluetooth, as
/// found by [`ApiClient::discover_bluetooth_setup_devices`](crate::ApiClient::discover_bluetooth_setup_devices).
#[derive(Clone)]
pub struct BluetoothSetupDevice {
    /// The name that the device advertises, as in `P110M_lgR0GOlb`.
    pub name: String,
    /// The model, from the name, as in `P110M`.
    pub model: String,
    /// The Bluetooth address, as in `98:BA:5F:A3:5F:07`. Empty where the
    /// operating system hides it, as macOS does.
    pub address: String,
    peripheral: Peripheral,
    /// Held while a dropped link disconnects, so that a new link to the
    /// device waits for that instead of being cut off by it.
    disconnecting: Arc<Mutex<()>>,
}

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
            peripheral,
            disconnecting: Arc::default(),
        });
    }

    Ok(devices)
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
    peripheral: Peripheral,
    write: Characteristic,
    read: Characteristic,
    timeout: Duration,
    exchange: Mutex<()>,
    disconnecting: Arc<Mutex<()>>,
}

impl BluetoothLink {
    async fn connect(device: &BluetoothSetupDevice, timeout: Duration) -> Result<Self, Error> {
        // Waits for the disconnect of an earlier link to the device.
        drop(device.disconnecting.lock().await);

        let peripheral = device.peripheral.clone();
        let (write, read) = match Self::open(&peripheral, timeout).await {
            Ok(characteristics) => characteristics,
            Err(error) => {
                // No link holds the connection yet, so no drop closes it.
                if let Ok(Err(error)) = tokio::time::timeout(timeout, peripheral.disconnect()).await
                {
                    debug!("Disconnecting over Bluetooth failed: {error}");
                }
                return Err(error);
            }
        };

        Ok(Self {
            peripheral,
            write,
            read,
            timeout,
            exchange: Mutex::new(()),
            disconnecting: Arc::clone(&device.disconnecting),
        })
    }

    /// Connects to `peripheral` and returns its write and read
    /// characteristics.
    async fn open(
        peripheral: &Peripheral,
        timeout: Duration,
    ) -> Result<(Characteristic, Characteristic), Error> {
        tokio::time::timeout(timeout, async {
            if !peripheral.is_connected().await? {
                debug!("Connecting over Bluetooth...");
                peripheral.connect().await?;
            }
            peripheral.discover_services().await
        })
        .await
        .map_err(|_| timeout_error("connecting over Bluetooth"))?
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
        let _guard = self.exchange.lock().await;

        tokio::time::timeout(self.timeout, self.exchange_unguarded(packet))
            .await
            .map_err(|_| timeout_error("waiting for the device's Bluetooth reply"))?
    }

    async fn exchange_unguarded(&self, packet: &Packet) -> Result<Packet, Error> {
        self.discard_stale_reply().await?;

        let encoded = packet.encode();
        let chunk_length =
            MAX_CHUNK_LENGTH.min(usize::from(self.peripheral.mtu()).saturating_sub(3).max(20));
        for chunk in encoded.chunks(chunk_length) {
            self.peripheral
                .write(&self.write, chunk, WriteType::WithResponse)
                .await
                .map_err(bluetooth_error)?;
        }

        // A reply to an earlier request that gave up, for example after a
        // timeout, can still arrive after this one was written.
        loop {
            let reply = self.read_reply().await?;
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
    async fn read_reply(&self) -> Result<Packet, Error> {
        let mut reply = Vec::new();
        loop {
            let data = self
                .peripheral
                .read(&self.read)
                .await
                .map_err(bluetooth_error)?;

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
    async fn discard_stale_reply(&self) -> Result<(), Error> {
        let mut discarded = 0;
        for _ in 0..MAX_STALE_READS {
            let data = self
                .peripheral
                .read(&self.read)
                .await
                .map_err(bluetooth_error)?;
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
        let Ok(runtime) = tokio::runtime::Handle::try_current() else {
            return;
        };
        let peripheral = self.peripheral.clone();
        let timeout = self.timeout;
        // Taken here rather than in the task, so that a new link cannot
        // connect before the disconnect has started.
        let guard = Arc::clone(&self.disconnecting).try_lock_owned().ok();
        runtime.spawn(async move {
            let _guard = guard;
            // The link may already be gone, so a failure is only logged. The
            // wait is bounded, so that a hung disconnect does not hold the
            // guard, and with it the next connect, forever.
            match tokio::time::timeout(timeout, peripheral.disconnect()).await {
                Ok(Ok(())) => {}
                Ok(Err(error)) => debug!("Disconnecting over Bluetooth failed: {error}"),
                Err(_) => debug!("Disconnecting over Bluetooth timed out"),
            }
        });
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
