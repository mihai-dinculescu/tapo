//! Playback of hub-stored recordings over an authenticated media stream.
//!
//! The control channel is JSON carried in multipart parts. The client sends
//! `{"type": "request", "seq": N, "params": {...}}` and the hub answers with
//! `{"type": "response", "seq": N, "params": {...}}`; either side may also send
//! `{"type": "notification", "params": {"event_type": ...}}`. Media arrives as
//! `video/mp2t` parts (MPEG-TS), each numbered by `X-Data-Sequence`.
//!
//! The exchange for a recording is:
//!
//! 1. `get` `playback`, scoped to the camera by `camera_mac` and to the clip
//!    by `start_time`, as the first client part. It carries
//!    `X-Data-Window-Size: 50`. The hub responds with an `error_code` and,
//!    on success, a `session_id` that later client parts echo as
//!    `X-Session-Id`.
//! 2. Media parts. Every 25th `X-Data-Sequence` is acknowledged with a
//!    `stream_sequence` notification carrying `X-Data-Received`.
//! 3. Heartbeat notifications every `X-Hb` seconds (default 15), and a
//!    `do` `stop` request when the client is done.
//!
//! The media parts are encrypted (`X-If-Encrypt: 1`); see [`super::cipher`].
//! The hub does not stop at the requested `end_time`, so a download stops
//! itself once the MPEG-TS clock has covered the clip (see [`super::mpeg_ts`]).
//!
//! Shapes and defaults follow the Tapo app (`GetVodParams`,
//! `DoStopRequest`, and the `VodStreamConnection` read loop). Verified
//! against an H200 on 2026-09-09: a 15 s clip arrived as 705 `video/mp2t`
//! parts (7.9 MB) followed by `stream_status: finished`.

use std::time::{Duration, Instant};

use anyhow::{Context, anyhow};
use log::{debug, trace, warn};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::api::protocol::crypto;
use crate::error::{Error, TapoResponseError};
use crate::responses::{RecordingDownloadOutcome, RecordingDownloadResult};

use super::MediaStreamConnection;
use super::cipher::{KeyExchange, MediaCipher};
use super::mpeg_ts::PcrClock;
use super::multipart::{Frame, Part, PartParser, encode_client_part};

const CONTENT_TYPE_JSON: (&str, &str) = ("Content-Type", "application/json");
/// Requested by the Tapo app on the playback request.
const DATA_WINDOW_SIZE: &str = "50";
/// The Tapo app acknowledges every 25th media part.
const ACK_EVERY: u64 = 25;
/// The heartbeat interval the Tapo app falls back to when the hub does not
/// send `X-Hb`.
const DEFAULT_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
/// Spare capacity reserved before each socket read.
const READ_CHUNK_SIZE: usize = 64 * 1024;
/// MPEG-TS packets are 188 bytes and start with this sync byte.
const MPEG_TS_PACKET_SIZE: usize = 188;
const MPEG_TS_SYNC_BYTE: u8 = 0x47;
/// Warn about at most this many HMAC mismatches; the count keeps growing.
const HMAC_WARNINGS: u64 = 3;

/// Selects the recording to play back.
#[derive(Debug, Clone)]
pub(crate) struct PlaybackRequest {
    pub camera_mac: String,
    pub player_id: String,
    /// Unix timestamp (seconds).
    pub start_time: u64,
    /// Unix timestamp (seconds).
    pub end_time: u64,
}

/// Plays back the recording for up to `duration`, writing the decrypted body
/// of every media part to `sink` in the order received. For an MPEG-TS stream
/// the concatenation is a playable `.ts` file.
///
/// With `clip_length`, playback also stops once the MPEG-TS clock shows that
/// much media, since the hub itself plays on past the recording's end.
///
/// Fails when the hub sent no media at all, or at the first encrypted media
/// part that no cipher can be set up for. Everything else the hub reported
/// is logged rather than returned; `RUST_LOG=tapo=debug` shows the control
/// messages and the cipher setup, `tapo=trace` every part.
pub(crate) async fn play<W: AsyncWrite + Unpin>(
    connection: MediaStreamConnection,
    request: PlaybackRequest,
    duration: Duration,
    clip_length: Option<Duration>,
    sink: &mut W,
) -> Result<RecordingDownloadResult, Error> {
    let MediaStreamConnection {
        stream,
        buffered,
        session,
        password_hash,
        password,
    } = connection;
    let (mut reader, mut writer) = stream.into_split();
    let mut parser = PartParser::new(buffered);

    let heartbeat_interval = session
        .heartbeat_interval_s
        .map(Duration::from_secs)
        .unwrap_or(DEFAULT_HEARTBEAT_INTERVAL);

    let mut state = State {
        session_id: session.session_id.clone(),
        key_exchange: session.key_exchange.clone(),
        seq: 0,
        secrets: candidate_secrets(&password_hash, &password),
        cipher: None,
        pcr: PcrClock::default(),
        clip_length,
        byte_count: 0,
        part_count: 0,
        duration_s: None,
        is_mpeg_ts: None,
        hmac_verified: None,
        hmac_mismatch_count: 0,
        first_media_part_seen: false,
        outcome: RecordingDownloadOutcome::DurationElapsed,
    };

    let request_seq = state.next_seq();
    debug!("Requesting playback: {request:?}");
    send(
        &mut writer,
        &state.session_headers(&[("X-Data-Window-Size", DATA_WINDOW_SIZE), CONTENT_TYPE_JSON]),
        &ControlRequest {
            kind: "request",
            seq: request_seq,
            params: GetPlaybackParams::new(request),
        },
    )
    .await?;

    let deadline = Instant::now() + duration;
    let mut next_heartbeat = Instant::now() + heartbeat_interval;

    'session: loop {
        while let Some(frame) = parser.next_frame()? {
            match frame {
                Frame::End => {
                    debug!("The hub sent the closing delimiter");
                    state.outcome = RecordingDownloadOutcome::ClosedByHub;
                    break 'session;
                }
                Frame::Part(part) => {
                    if !state
                        .handle_part(&mut writer, sink, part, request_seq)
                        .await?
                    {
                        break 'session;
                    }
                }
            }
        }

        let now = Instant::now();
        if now >= deadline {
            debug!("Playback time limit elapsed");
            break;
        }
        if now >= next_heartbeat {
            debug!("Sending a media stream heartbeat...");
            send(
                &mut writer,
                &state.session_headers(&[CONTENT_TYPE_JSON]),
                &Notification::new("heartbeat"),
            )
            .await?;
            next_heartbeat = now + heartbeat_interval;
        }

        let wait = deadline.min(next_heartbeat).saturating_duration_since(now);
        match read(&mut reader, &mut parser, wait).await? {
            ReadOutcome::Data => {}
            // The read is cancel safe, so nothing is lost when it times out;
            // the loop then re-evaluates the heartbeat and the deadline.
            ReadOutcome::TimedOut => {}
            ReadOutcome::Eof => {
                debug!("The hub closed the media stream");
                state.outcome = RecordingDownloadOutcome::ClosedByHub;
                break;
            }
        }
    }

    // Best effort: the session is being torn down either way.
    let stop_seq = state.next_seq();
    if let Err(err) = send(
        &mut writer,
        &state.session_headers(&[CONTENT_TYPE_JSON]),
        &ControlRequest {
            kind: "request",
            seq: stop_seq,
            params: DoStopParams::default(),
        },
    )
    .await
    {
        debug!("Failed to send the playback stop request: {err:?}");
    }
    if let Err(err) = writer.shutdown().await {
        debug!("Failed to shut down the media stream: {err:?}");
    }

    sink.flush().await.context("flush the media sink")?;

    state.finish(duration)
}

/// The secrets to try for the media cipher, most likely first: the Digest
/// pre-hash (what the app uses), the other pre-hash, the raw password, and
/// an empty secret.
fn candidate_secrets(password_hash: &str, password: &str) -> Vec<(String, &'static str)> {
    let mut secrets: Vec<(String, &'static str)> =
        vec![(password_hash.to_string(), "Digest pre-hash")];
    for (secret, label) in [
        (crypto::sha256_hex(password.as_bytes()), "SHA-256 pre-hash"),
        (crypto::md5_hex(password.as_bytes()), "MD5 pre-hash"),
        (password.to_string(), "raw password"),
        (String::new(), "empty secret"),
    ] {
        if !secrets.iter().any(|(known, _)| *known == secret) {
            secrets.push((secret, label));
        }
    }
    secrets
}

struct State {
    /// The session id echoed as `X-Session-Id` on client parts: the one
    /// issued for the playback once known, otherwise the one from the
    /// handshake, if any.
    session_id: Option<String>,
    /// The `Key-Exchange` header the media cipher is derived from.
    key_exchange: Option<String>,
    seq: u32,
    secrets: Vec<(String, &'static str)>,
    /// Set up from the first encrypted media part.
    cipher: Option<MediaCipher>,
    pcr: PcrClock,
    clip_length: Option<Duration>,
    byte_count: u64,
    part_count: u64,
    duration_s: Option<f64>,
    is_mpeg_ts: Option<bool>,
    hmac_verified: Option<bool>,
    hmac_mismatch_count: u64,
    first_media_part_seen: bool,
    outcome: RecordingDownloadOutcome,
}

impl State {
    /// Turns the bookkeeping into the caller's result, or into the error that
    /// explains why the download is not usable.
    fn finish(self, time_limit: Duration) -> Result<RecordingDownloadResult, Error> {
        debug!(
            "Download finished: {} parts, {} bytes, duration {:?}, outcome {:?}, decrypted {}, HMAC verified {:?}, HMAC mismatches {}",
            self.part_count,
            self.byte_count,
            self.duration_s,
            self.outcome,
            self.cipher.is_some(),
            self.hmac_verified,
            self.hmac_mismatch_count,
        );

        if self.part_count == 0 {
            return Err(
                anyhow!("the hub sent no media for the recording within {time_limit:?}").into(),
            );
        }

        Ok(RecordingDownloadResult {
            byte_count: self.byte_count,
            duration_s: self.duration_s,
            outcome: self.outcome,
        })
    }

    fn next_seq(&mut self) -> u32 {
        self.seq += 1;
        self.seq
    }

    fn session_headers<'a>(&'a self, headers: &[(&'a str, &'a str)]) -> Vec<(&'a str, &'a str)> {
        let mut all = Vec::with_capacity(headers.len() + 1);
        if let Some(session_id) = &self.session_id {
            all.push(("X-Session-Id", session_id.as_str()));
        }
        all.extend_from_slice(headers);
        all
    }

    /// Returns `false` when the session is over.
    async fn handle_part<W: AsyncWrite + Unpin>(
        &mut self,
        writer: &mut OwnedWriteHalf,
        sink: &mut W,
        part: Part,
        request_seq: u32,
    ) -> Result<bool, Error> {
        if part.is_json() {
            let text = String::from_utf8_lossy(&part.body);
            debug!("Media stream control message: {text}");

            let message: ControlMessage = serde_json::from_slice(&part.body)
                .context("invalid media stream control message")?;

            match message.kind.as_str() {
                "response" => {
                    if message.seq == Some(i64::from(request_seq)) {
                        return self.handle_playback_response(message.params);
                    }
                }
                "notification" => {
                    let notification: NotificationMessage = serde_json::from_value(message.params)
                        .context("invalid media stream notification")?;
                    let event_type = notification.event_type.unwrap_or_default();

                    let outcome = match event_type.as_str() {
                        "stream_finish" => Some(RecordingDownloadOutcome::Finished),
                        "stream_status" if notification.status.as_deref() == Some("finished") => {
                            Some(RecordingDownloadOutcome::Finished)
                        }
                        "connection_closed" => Some(RecordingDownloadOutcome::ClosedByHub),
                        _ => None,
                    };
                    debug!("Media stream notification: {event_type}");

                    if let Some(outcome) = outcome {
                        self.outcome = outcome;
                        return Ok(false);
                    }
                }
                other => debug!("Ignoring media stream control message of type `{other}`"),
            }

            return Ok(true);
        }

        let content_type = part.content_type().unwrap_or("").to_string();
        trace!(
            "Media stream media part: {content_type} ({} bytes), headers: {:?}",
            part.body.len(),
            part.headers
        );

        if !self.first_media_part_seen {
            self.first_media_part_seen = true;
            debug!("First media part headers: {:?}", part.headers);
        }

        let sequence = part
            .header("x-data-sequence")
            .and_then(|value| value.trim().parse::<u64>().ok());

        let body = if part
            .header("x-if-encrypt")
            .is_some_and(|value| value.trim() == "1")
        {
            match self.decrypt(&part)? {
                Some(body) => body,
                // Dropped (HMAC mismatch); still acknowledge the sequence.
                None => {
                    self.acknowledge(writer, sequence).await?;
                    return Ok(true);
                }
            }
        } else {
            part.body
        };

        if self.is_mpeg_ts.is_none() {
            let is_mpeg_ts = looks_like_mpeg_ts(&body);
            if !is_mpeg_ts {
                warn!(
                    "The media stream parts ({content_type}) do not look like MPEG-TS (decrypted: {})",
                    self.cipher.is_some()
                );
            }
            self.is_mpeg_ts = Some(is_mpeg_ts);
        }

        self.part_count += 1;
        self.byte_count += body.len() as u64;

        sink.write_all(&body)
            .await
            .context("write a media part to the media sink")?;

        // Only a stream known to be MPEG-TS has a meaningful clock; ciphertext
        // would yield random "sync bytes" and a nonsense duration.
        let elapsed = if self.is_mpeg_ts == Some(true) {
            self.pcr.observe(&body);
            self.pcr.elapsed()
        } else {
            None
        };
        self.duration_s = elapsed.map(|elapsed| elapsed.as_secs_f64());

        self.acknowledge(writer, sequence).await?;

        if let (Some(clip_length), Some(elapsed)) = (self.clip_length, elapsed)
            && elapsed >= clip_length
        {
            debug!("The media covers the clip's {clip_length:?}; stopping the playback");
            self.outcome = RecordingDownloadOutcome::ClipEndReached;
            return Ok(false);
        }

        Ok(true)
    }

    /// Records the sequence and acknowledges every `ACK_EVERY`th one.
    async fn acknowledge(
        &mut self,
        writer: &mut OwnedWriteHalf,
        sequence: Option<u64>,
    ) -> Result<(), Error> {
        let Some(sequence) = sequence else {
            return Ok(());
        };

        if sequence % ACK_EVERY == 0 {
            trace!("Acknowledging media stream sequence {sequence}");
            let received = sequence.to_string();
            send(
                writer,
                &self.session_headers(&[("X-Data-Received", received.as_str()), CONTENT_TYPE_JSON]),
                &Notification::new("stream_sequence"),
            )
            .await?;
        }

        Ok(())
    }

    /// Decrypts an encrypted media part. Returns `None` when the part must be
    /// dropped because its HMAC does not match, and fails when no cipher can
    /// be set up for it.
    fn decrypt(&mut self, part: &Part) -> Result<Option<Vec<u8>>, Error> {
        let cipher = match &self.cipher {
            Some(cipher) => cipher,
            None => {
                let cipher = self.select_cipher(part)?;
                self.cipher.insert(cipher)
            }
        };

        if let Some(hmac) = part.header("x-data-hmac")
            && !cipher.verify_hmac(&part.body, hmac)
        {
            self.hmac_mismatch_count += 1;
            if self.hmac_mismatch_count <= HMAC_WARNINGS {
                warn!(
                    "Dropping a media stream part whose X-Data-Hmac does not match (sequence {:?})",
                    part.header("x-data-sequence")
                );
            }
            return Ok(None);
        }

        let Some(nonce) = part.header("x-nonce") else {
            return Err(anyhow!("encrypted media stream part without an X-Nonce").into());
        };

        Ok(Some(cipher.decrypt(nonce, &part.body)?))
    }

    /// Derives the media cipher from the session's `Key-Exchange`, letting the
    /// part's `X-Data-Hmac` choose among the candidate secrets.
    fn select_cipher(&mut self, part: &Part) -> Result<MediaCipher, Error> {
        let key_exchange = self
            .key_exchange
            .as_deref()
            .ok_or_else(|| anyhow!("the hub encrypts the recording but sent no Key-Exchange"))?;
        let key_exchange =
            KeyExchange::parse(key_exchange).context("invalid media stream Key-Exchange")?;
        if !key_exchange.is_supported() {
            return Err(anyhow!(
                "unsupported media stream cipher: cipher={:?}, algorithm={:?}",
                key_exchange.cipher,
                key_exchange.algorithm
            )
            .into());
        }

        let hmac = part.header("x-data-hmac");
        for (index, (secret, label)) in self.secrets.iter().enumerate() {
            let cipher = MediaCipher::derive(&key_exchange, secret)
                .context("derive the media stream keys")?;

            match hmac {
                Some(hmac) if cipher.verify_hmac(&part.body, hmac) => {
                    debug!(
                        "Media stream cipher ready: secret candidate {index} ({label}) matches the X-Data-Hmac"
                    );
                    self.hmac_verified = Some(true);
                    return Ok(cipher);
                }
                Some(_) => continue,
                None => {
                    debug!(
                        "Media stream cipher ready: secret candidate {index} ({label}), unverified (no X-Data-Hmac)"
                    );
                    return Ok(cipher);
                }
            }
        }

        Err(anyhow!(
            "cannot decrypt the recording: none of the {} candidate secrets matches the X-Data-Hmac of the media stream parts",
            self.secrets.len()
        )
        .into())
    }

    fn handle_playback_response(&mut self, params: serde_json::Value) -> Result<bool, Error> {
        let response: GetPlaybackResponse =
            serde_json::from_value(params).context("invalid media stream playback response")?;

        if response.error_code != 0 {
            return Err(Error::Tapo(TapoResponseError::ResponseError {
                description: format!(
                    "The hub rejected the playback request with error code {}",
                    response.error_code
                ),
            }));
        }

        debug!(
            "Playback accepted: session_id={:?}, speed={:?}",
            response.session_id, response.speed
        );
        if response.session_id.is_some() {
            self.session_id = response.session_id;
        }

        Ok(true)
    }
}

/// Whether `body` starts with MPEG-TS packets: a sync byte at every packet
/// boundary that falls inside the body.
fn looks_like_mpeg_ts(body: &[u8]) -> bool {
    !body.is_empty()
        && body
            .iter()
            .step_by(MPEG_TS_PACKET_SIZE)
            .all(|byte| *byte == MPEG_TS_SYNC_BYTE)
}

enum ReadOutcome {
    Data,
    TimedOut,
    Eof,
}

async fn read(
    reader: &mut OwnedReadHalf,
    parser: &mut PartParser,
    wait: Duration,
) -> anyhow::Result<ReadOutcome> {
    let buffer = parser.buffer_mut();
    buffer.reserve(READ_CHUNK_SIZE);

    match tokio::time::timeout(wait, reader.read_buf(buffer)).await {
        Ok(Ok(0)) => Ok(ReadOutcome::Eof),
        Ok(Ok(_)) => Ok(ReadOutcome::Data),
        Ok(Err(err)) => Err(anyhow::Error::new(err).context("read media stream")),
        Err(_) => Ok(ReadOutcome::TimedOut),
    }
}

async fn send<T: Serialize>(
    writer: &mut OwnedWriteHalf,
    headers: &[(&str, &str)],
    message: &T,
) -> anyhow::Result<()> {
    let body = serde_json::to_vec(message).context("serialize media stream control message")?;
    trace!(
        "Media stream client part: headers {headers:?}, body {}",
        String::from_utf8_lossy(&body)
    );

    writer
        .write_all(&encode_client_part(headers, &body))
        .await
        .context("write media stream part")?;
    writer.flush().await.context("flush media stream part")?;

    Ok(())
}

#[derive(Debug, Serialize)]
struct ControlRequest<T> {
    #[serde(rename = "type")]
    kind: &'static str,
    seq: u32,
    params: T,
}

/// `{"method": "get", "playback": {...}}`, after the Tapo app's
/// `GetPlaybackRequest` / `GetVodParams`. Fields the app leaves unset for
/// plain hub playback (event filters, audio config, face ids) are omitted.
#[derive(Debug, Serialize)]
struct GetPlaybackParams {
    method: &'static str,
    playback: VodParams,
}

impl GetPlaybackParams {
    fn new(request: PlaybackRequest) -> Self {
        Self {
            method: "get",
            playback: VodParams {
                channels: vec![0],
                client_id: 1,
                scale: "1/1",
                start_time: request.start_time.to_string(),
                end_time: request.end_time.to_string(),
                player_id: request.player_id,
                vod_type: 0,
                auto_seek: 0,
                auto_switch_date: "0",
                camera_mac: request.camera_mac,
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct VodParams {
    channels: Vec<u8>,
    /// Distinguishes viewers on one connection. The app passes a small
    /// per-player integer; a single viewer uses 1.
    client_id: u32,
    /// Playback speed (`VodScale`); `1/1` is real time.
    scale: &'static str,
    /// Unix timestamps (seconds), as strings like the app sends them.
    start_time: String,
    end_time: String,
    player_id: String,
    /// `VodType::NORMAL`.
    vod_type: u8,
    /// `SeekMethod::NORMAL`.
    auto_seek: u8,
    auto_switch_date: &'static str,
    camera_mac: String,
}

/// `{"method": "do", "stop": "null"}`, after the Tapo app's `DoStopRequest`.
#[derive(Debug, Serialize)]
struct DoStopParams {
    method: &'static str,
    stop: &'static str,
}

impl Default for DoStopParams {
    fn default() -> Self {
        Self {
            method: "do",
            stop: "null",
        }
    }
}

#[derive(Debug, Serialize)]
struct Notification {
    #[serde(rename = "type")]
    kind: &'static str,
    params: NotificationParams,
}

impl Notification {
    fn new(event_type: &'static str) -> Self {
        Self {
            kind: "notification",
            params: NotificationParams { event_type },
        }
    }
}

#[derive(Debug, Serialize)]
struct NotificationParams {
    event_type: &'static str,
}

#[derive(Debug, Deserialize)]
struct ControlMessage {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    seq: Option<i64>,
    #[serde(default)]
    params: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct NotificationMessage {
    #[serde(default)]
    event_type: Option<String>,
    /// Carried by `stream_status` notifications.
    #[serde(default)]
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GetPlaybackResponse {
    #[serde(default)]
    error_code: i64,
    #[serde(default)]
    session_id: Option<String>,
    #[serde(default)]
    speed: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> PlaybackRequest {
        PlaybackRequest {
            camera_mac: "AA-BB-CC-DD-EE-FF".to_string(),
            player_id: "PLAYER".to_string(),
            start_time: 1_700_000_000,
            end_time: 1_700_000_060,
        }
    }

    #[test]
    fn test_get_playback_request_json() {
        let message = ControlRequest {
            kind: "request",
            seq: 1,
            params: GetPlaybackParams::new(request()),
        };

        let json = serde_json::to_value(&message).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "request",
                "seq": 1,
                "params": {
                    "method": "get",
                    "playback": {
                        "channels": [0],
                        "client_id": 1,
                        "scale": "1/1",
                        "start_time": "1700000000",
                        "end_time": "1700000060",
                        "player_id": "PLAYER",
                        "vod_type": 0,
                        "auto_seek": 0,
                        "auto_switch_date": "0",
                        "camera_mac": "AA-BB-CC-DD-EE-FF",
                    }
                }
            })
        );
    }

    #[test]
    fn test_stop_and_notification_json() {
        let stop = ControlRequest {
            kind: "request",
            seq: 2,
            params: DoStopParams::default(),
        };
        assert_eq!(
            serde_json::to_value(&stop).unwrap(),
            serde_json::json!({
                "type": "request",
                "seq": 2,
                "params": {"method": "do", "stop": "null"}
            })
        );

        assert_eq!(
            serde_json::to_value(Notification::new("heartbeat")).unwrap(),
            serde_json::json!({
                "type": "notification",
                "params": {"event_type": "heartbeat"}
            })
        );
    }

    #[test]
    fn test_parse_playback_response() {
        let message: ControlMessage = serde_json::from_str(
            r#"{"type":"response","seq":1,"params":{"error_code":0,"session_id":"42","speed":"1/1"}}"#,
        )
        .unwrap();
        assert_eq!(message.kind, "response");
        assert_eq!(message.seq, Some(1));

        let response: GetPlaybackResponse = serde_json::from_value(message.params).unwrap();
        assert_eq!(response.error_code, 0);
        assert_eq!(response.session_id.as_deref(), Some("42"));
        assert_eq!(response.speed.as_deref(), Some("1/1"));
    }

    #[test]
    fn test_candidate_secrets_are_distinct_and_ordered() {
        let secrets = candidate_secrets(&crypto::sha256_hex(b"pw"), "pw");
        let labels: Vec<&str> = secrets.iter().map(|(_, label)| *label).collect();
        // The Digest pre-hash equals the SHA-256 pre-hash, which is dropped.
        assert_eq!(
            labels,
            [
                "Digest pre-hash",
                "MD5 pre-hash",
                "raw password",
                "empty secret"
            ]
        );
        assert_eq!(secrets[0].0, crypto::sha256_hex(b"pw"));
        assert_eq!(secrets[2].0, "pw");
        assert_eq!(secrets[3].0, "");
    }

    #[test]
    fn test_looks_like_mpeg_ts() {
        let mut packets = vec![0u8; 188 * 3];
        for index in [0, 188, 376] {
            packets[index] = 0x47;
        }
        assert!(looks_like_mpeg_ts(&packets));
        // A partial trailing packet is still MPEG-TS.
        assert!(looks_like_mpeg_ts(&packets[..300]));

        assert!(!looks_like_mpeg_ts(&[]));
        assert!(!looks_like_mpeg_ts(&[0x00; 188]));
        packets[188] = 0x00;
        assert!(!looks_like_mpeg_ts(&packets));
    }

    #[test]
    fn test_parse_notification_without_seq() {
        let message: ControlMessage = serde_json::from_str(
            r#"{"type":"notification","params":{"event_type":"stream_status","status":"finished"}}"#,
        )
        .unwrap();
        assert_eq!(message.seq, None);

        let notification: NotificationMessage = serde_json::from_value(message.params).unwrap();
        assert_eq!(notification.event_type.as_deref(), Some("stream_status"));
        assert_eq!(notification.status.as_deref(), Some("finished"));
    }

    #[test]
    fn test_session_headers_prepend_session_id() {
        let mut state = State {
            session_id: None,
            key_exchange: None,
            seq: 0,
            secrets: candidate_secrets("HASH", "pw"),
            cipher: None,
            pcr: PcrClock::default(),
            clip_length: None,
            byte_count: 0,
            part_count: 0,
            duration_s: None,
            is_mpeg_ts: None,
            hmac_verified: None,
            hmac_mismatch_count: 0,
            first_media_part_seen: false,
            outcome: RecordingDownloadOutcome::DurationElapsed,
        };

        assert_eq!(
            state.session_headers(&[CONTENT_TYPE_JSON]),
            vec![CONTENT_TYPE_JSON]
        );
        assert_eq!(state.next_seq(), 1);
        assert_eq!(state.next_seq(), 2);

        state.session_id = Some("42".to_string());
        assert_eq!(
            state.session_headers(&[CONTENT_TYPE_JSON]),
            vec![("X-Session-Id", "42"), CONTENT_TYPE_JSON]
        );
    }
}
