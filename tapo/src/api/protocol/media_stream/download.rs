//! Download of hub-stored recordings over an authenticated media stream.
//!
//! The control channel is JSON carried in multipart parts. The client sends
//! `{"type": "request", "seq": N, "params": {...}}` and the hub answers with
//! `{"type": "response", "seq": N, "params": {...}}`; either side may also send
//! `{"type": "notification", "params": {"event_type": ...}}`. Media arrives as
//! `video/mp2t` parts (MPEG-TS), each numbered by `X-Data-Sequence`.
//!
//! The exchange for a recording is:
//!
//! 1. `get` `download`, scoped to the camera by `dev_id` and to the clip by
//!    `start_time` and `end_time`, as the first client part. It carries
//!    `X-Data-Window-Size: 50`. The hub responds with an `error_code` and,
//!    on success, a `session_id` that later client parts echo as
//!    `X-Session-Id`.
//! 2. Media parts. `X-Data-Sequence` counts them from 1 without gaps, and
//!    every 25th is acknowledged with a `stream_sequence` notification
//!    carrying `X-Data-Received`.
//! 3. Heartbeat notifications every `X-Hb` seconds (default 15), and a
//!    `do` `stop` request when the client is done.
//!
//! The hub ends the clip itself with a `stream_status` notification of
//! `finished`, so nothing here has to work out where the footage stops. The
//! MPEG-TS clock only measures how much media arrived (see
//! [`super::mpeg_ts`]). The media parts are encrypted (`X-If-Encrypt: 1`);
//! see [`super::cipher`].
//!
//! Shapes and defaults follow the Tapo app's clip save: its download
//! request, its stop request, and its read loop. Verified against an H200
//! on 2026-09-20: a 9 s clip arrived as 224 `video/mp2t` parts (2.7 MB)
//! holding 8.428 s of media, and the hub ended it about 2 s after the
//! request.

use std::time::{Duration, Instant};

use anyhow::{Context, anyhow};
use log::{debug, trace, warn};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::error::{Error, TapoResponseError};
use crate::responses::{RecordingDownloadOutcome, RecordingDownloadResult};

use super::MediaStreamConnection;
use super::cipher::{KeyExchange, MediaCipher};
use super::mpeg_ts::StreamClock;
use super::multipart::{Frame, Part, PartParser, encode_client_part};

const CONTENT_TYPE_JSON: (&str, &str) = ("Content-Type", "application/json");
/// Requested by the Tapo app on the download request.
const DATA_WINDOW_SIZE: &str = "50";
/// The Tapo app acknowledges every 25th media part.
const ACK_EVERY: u64 = 25;
/// The heartbeat interval the Tapo app falls back to when the hub does not
/// send `X-Hb`.
const DEFAULT_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
/// Spare capacity reserved before each socket read.
const READ_CHUNK_SIZE: usize = 64 * 1024;
/// Warn about at most this many gaps in `X-Data-Sequence`; the counts keep
/// growing.
const SEQUENCE_GAP_WARNINGS: u64 = 3;

/// Selects the recording to download.
#[derive(Debug, Clone)]
pub(crate) struct DownloadRequest {
    /// Sent as `dev_id`.
    pub device_id: String,
    pub player_id: String,
    /// Unix timestamp (seconds).
    pub start_time: u64,
    /// Unix timestamp (seconds).
    pub end_time: u64,
}

/// Downloads the recording within `time_limit`, writing the decrypted body of
/// every media part to `sink` in the order received. For an MPEG-TS stream
/// the concatenation is a playable `.ts` file.
///
/// The hub ends the clip itself, so `time_limit` is only a backstop.
///
/// Fails when the hub rejects the request, sends no media at all, or sends
/// something that cannot be read: a control message or part that does not
/// parse, or an encrypted media part that cannot be decrypted or does not
/// match its `X-Data-Hmac`. Also fails when reading from the hub, writing to
/// it, or writing to `sink` fails.
///
/// Stopping early is not an error: when the time limit runs out or the hub
/// closes the stream, the result's `outcome` says so. Gaps in
/// `X-Data-Sequence` and control messages that need no action are only
/// logged.
pub(crate) async fn download<W: AsyncWrite + Unpin>(
    connection: MediaStreamConnection,
    request: DownloadRequest,
    time_limit: Duration,
    sink: &mut W,
) -> Result<RecordingDownloadResult, Error> {
    let MediaStreamConnection {
        stream,
        buffered,
        session,
        password_hash,
    } = connection;
    let (mut reader, mut writer) = stream.into_split();
    let mut parser = PartParser::new(buffered);

    let heartbeat_interval = session
        .heartbeat_interval_s
        .map(Duration::from_secs)
        .unwrap_or(DEFAULT_HEARTBEAT_INTERVAL);

    let mut state = State {
        session_id: session.session_id,
        key_exchange: session.key_exchange,
        seq: 0,
        secret: password_hash,
        cipher: None,
        clock: StreamClock::default(),
        byte_count: 0,
        part_count: 0,
        duration_s: None,
        last_sequence: None,
        sequence_gap_count: 0,
        missing_part_count: 0,
        first_media_part_seen: false,
        outcome: RecordingDownloadOutcome::DurationElapsed,
    };

    let request_seq = state.next_seq();
    debug!("Requesting the recording: {request:?}");
    send(
        &mut writer,
        &state.session_headers(&[("X-Data-Window-Size", DATA_WINDOW_SIZE), CONTENT_TYPE_JSON]),
        &ControlRequest {
            kind: "request",
            seq: request_seq,
            params: GetDownloadParams::new(request),
        },
    )
    .await?;

    let deadline = Instant::now() + time_limit;
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
            debug!("Download time limit elapsed");
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
        debug!("Failed to send the download stop request: {err:?}");
    }
    if let Err(err) = writer.shutdown().await {
        debug!("Failed to shut down the media stream: {err:?}");
    }

    sink.flush().await.context("flush the media sink")?;

    state.finish(time_limit)
}

struct State {
    /// The session id echoed as `X-Session-Id` on client parts: the one
    /// issued for the download once known, otherwise the one from the
    /// handshake, if any.
    session_id: Option<String>,
    /// The `Key-Exchange` header the media cipher is derived from.
    key_exchange: Option<String>,
    seq: u32,
    /// The media cipher secret: the password as pre-hashed for the Digest
    /// round.
    secret: String,
    /// Set up from the first encrypted media part.
    cipher: Option<MediaCipher>,
    clock: StreamClock,
    byte_count: u64,
    part_count: u64,
    duration_s: Option<f64>,
    /// The last `X-Data-Sequence` seen, which the hub numbers from 1 without
    /// gaps.
    last_sequence: Option<u64>,
    sequence_gap_count: u64,
    /// How many parts those gaps skipped: media the hub numbered but never
    /// sent, which leaves a hole in the written stream.
    missing_part_count: u64,
    first_media_part_seen: bool,
    outcome: RecordingDownloadOutcome,
}

impl State {
    /// Turns the bookkeeping into the caller's result, or into the error that
    /// explains why the download is not usable.
    fn finish(self, time_limit: Duration) -> Result<RecordingDownloadResult, Error> {
        debug!(
            "Download finished: {} parts, {} bytes, duration {:?}, outcome {:?}, decrypted {}, missing parts {}",
            self.part_count,
            self.byte_count,
            self.duration_s,
            self.outcome,
            self.cipher.is_some(),
            self.missing_part_count,
        );

        if self.part_count == 0 {
            return Err(match self.outcome {
                RecordingDownloadOutcome::Finished => {
                    anyhow!("the hub reported the end of the recording without sending any media")
                }
                RecordingDownloadOutcome::ClosedByHub => {
                    anyhow!("the hub closed the stream without sending any media")
                }
                RecordingDownloadOutcome::DurationElapsed => {
                    anyhow!("the hub sent no media for the recording within {time_limit:?}")
                }
            }
            .into());
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
                        return self.handle_download_response(message.params);
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

        trace!(
            "Media stream media part: {} ({} bytes), headers: {:?}",
            part.content_type().unwrap_or(""),
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
        if let Some(sequence) = sequence {
            self.record_sequence(sequence);
        }

        let body = if part
            .header("x-if-encrypt")
            .is_some_and(|value| value.trim() == "1")
        {
            self.decrypt(&part)?
        } else {
            part.body
        };

        self.part_count += 1;

        self.clock.observe(&body);
        self.duration_s = self.clock.elapsed().map(|elapsed| elapsed.as_secs_f64());

        self.byte_count += body.len() as u64;
        sink.write_all(&body)
            .await
            .context("write a media part to the media sink")?;

        self.acknowledge(writer, sequence).await?;

        Ok(true)
    }

    /// Notes the part's `X-Data-Sequence` and warns when the hub skipped
    /// one, which the sink has no other way of showing: the written stream
    /// stays a valid concatenation of the parts that did arrive, so a gap
    /// only shows up as footage missing from the middle of the clip.
    fn record_sequence(&mut self, sequence: u64) {
        if let Some(previous) = self.last_sequence
            && sequence > previous + 1
        {
            self.missing_part_count += sequence - previous - 1;
            self.sequence_gap_count += 1;
            if self.sequence_gap_count <= SEQUENCE_GAP_WARNINGS {
                warn!(
                    "The media stream sequence jumped from {previous} to {sequence}, so the recording is missing footage"
                );
            }
        }

        self.last_sequence = Some(sequence);
    }

    /// Acknowledges every `ACK_EVERY`th sequence.
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

    /// Decrypts an encrypted media part after checking it against its
    /// `X-Data-Hmac`, when it has one. Fails when no cipher can be set up for
    /// it or when the HMAC does not match.
    fn decrypt(&mut self, part: &Part) -> Result<Vec<u8>, Error> {
        let cipher = match &self.cipher {
            Some(cipher) => cipher,
            None => {
                let cipher = self.derive_cipher()?;
                self.cipher.insert(cipher)
            }
        };

        if let Some(hmac) = part.header("x-data-hmac")
            && !cipher.verify_hmac(&part.body, hmac)
        {
            return Err(anyhow!(
                "cannot decrypt the recording: the X-Data-Hmac of media stream part {} does not match the media stream keys",
                part.header("x-data-sequence").unwrap_or("?")
            )
            .into());
        }

        let Some(nonce) = part.header("x-nonce") else {
            return Err(anyhow!("encrypted media stream part without an X-Nonce").into());
        };

        Ok(cipher.decrypt(nonce, &part.body)?)
    }

    /// Derives the media cipher from the session's `Key-Exchange`.
    fn derive_cipher(&self) -> Result<MediaCipher, Error> {
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

        let cipher = MediaCipher::derive(&key_exchange, &self.secret)
            .context("derive the media stream keys")?;
        debug!("Media stream cipher ready");

        Ok(cipher)
    }

    fn handle_download_response(&mut self, params: serde_json::Value) -> Result<bool, Error> {
        let response: GetDownloadResponse =
            serde_json::from_value(params).context("invalid media stream download response")?;

        if response.error_code != 0 {
            return Err(Error::Tapo(TapoResponseError::ResponseError {
                description: format!(
                    "The hub rejected the recording request with error code {}",
                    response.error_code
                ),
            }));
        }

        debug!(
            "Recording request accepted: session_id={:?}",
            response.session_id
        );
        if response.session_id.is_some() {
            self.session_id = response.session_id;
        }

        Ok(true)
    }
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

/// `{"method": "get", "download": {...}}`, shaped like the download request
/// the Tapo app sends to save a clip. Fields the app leaves unset for a
/// plain video clip (download type, event filters, audio config, `last_pts`
/// for a resumed download) are omitted.
#[derive(Debug, Serialize)]
struct GetDownloadParams {
    method: &'static str,
    download: DownloadParams,
}

impl GetDownloadParams {
    fn new(request: DownloadRequest) -> Self {
        Self {
            method: "get",
            download: DownloadParams {
                dev_id: request.device_id,
                client_id: 1,
                end_time: request.end_time.to_string(),
                media_type: 0,
                player_id: request.player_id,
                start_time: request.start_time.to_string(),
                channels: vec![0],
                streams: Vec::new(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct DownloadParams {
    /// The app sends the camera's mac as well, but an H200 finds the camera
    /// from the device id alone.
    dev_id: String,
    /// The app always uses 1 when downloading from a camera hub.
    client_id: u32,
    /// Unix timestamps (seconds), as strings like the app sends them.
    end_time: String,
    /// 0 selects video, as the app sends for a plain clip.
    media_type: u8,
    player_id: String,
    start_time: String,
    channels: Vec<u8>,
    /// Stream selectors for trajectory clips; empty for a plain clip.
    streams: Vec<u8>,
}

/// `{"method": "do", "stop": "null"}`, the stop request the Tapo app sends.
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
struct GetDownloadResponse {
    #[serde(default)]
    error_code: i64,
    #[serde(default)]
    session_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> DownloadRequest {
        DownloadRequest {
            device_id: "DEVICE".to_string(),
            player_id: "PLAYER".to_string(),
            start_time: 1_700_000_000,
            end_time: 1_700_000_060,
        }
    }

    #[test]
    fn test_get_download_request_json() {
        let message = ControlRequest {
            kind: "request",
            seq: 1,
            params: GetDownloadParams::new(request()),
        };

        let json = serde_json::to_value(&message).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "type": "request",
                "seq": 1,
                "params": {
                    "method": "get",
                    "download": {
                        "dev_id": "DEVICE",
                        "client_id": 1,
                        "end_time": "1700000060",
                        "media_type": 0,
                        "player_id": "PLAYER",
                        "start_time": "1700000000",
                        "channels": [0],
                        "streams": [],
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
    fn test_parse_download_response() {
        let message: ControlMessage = serde_json::from_str(
            r#"{"type":"response","seq":1,"params":{"error_code":0,"session_id":"42"}}"#,
        )
        .unwrap();
        assert_eq!(message.kind, "response");
        assert_eq!(message.seq, Some(1));

        let response: GetDownloadResponse = serde_json::from_value(message.params).unwrap();
        assert_eq!(response.error_code, 0);
        assert_eq!(response.session_id.as_deref(), Some("42"));
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

    fn state() -> State {
        State {
            session_id: None,
            key_exchange: None,
            seq: 0,
            secret: "HASH".to_string(),
            cipher: None,
            clock: StreamClock::default(),
            byte_count: 0,
            part_count: 0,
            duration_s: None,
            last_sequence: None,
            sequence_gap_count: 0,
            missing_part_count: 0,
            first_media_part_seen: false,
            outcome: RecordingDownloadOutcome::DurationElapsed,
        }
    }

    #[test]
    fn test_session_headers_prepend_session_id() {
        let mut state = state();

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

    #[test]
    fn test_record_sequence_counts_the_parts_the_hub_skipped() {
        let mut state = state();

        for sequence in 1..=3 {
            state.record_sequence(sequence);
        }
        assert_eq!(state.sequence_gap_count, 0);
        assert_eq!(state.missing_part_count, 0);

        // 4, 5 and 6 never arrived.
        state.record_sequence(7);
        assert_eq!(state.sequence_gap_count, 1);
        assert_eq!(state.missing_part_count, 3);

        // A repeated sequence is not a gap.
        state.record_sequence(7);
        state.record_sequence(8);
        assert_eq!(state.sequence_gap_count, 1);
        assert_eq!(state.missing_part_count, 3);

        state.record_sequence(10);
        assert_eq!(state.sequence_gap_count, 2);
        assert_eq!(state.missing_part_count, 4);
    }
}
