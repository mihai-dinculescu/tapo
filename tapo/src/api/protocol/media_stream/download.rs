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
//! 3. Heartbeat notifications every 15 seconds, and a `do` `stop` request
//!    when the client is done.
//!
//! The hub ends the clip itself with a `stream_status` notification of
//! `finished`, so nothing here has to work out where the footage stops. The
//! MPEG-TS clock only measures how much media arrived (see
//! [`super::mpeg_ts`]). The hub encrypts every media part, so each one is
//! checked against its HMAC and decrypted, and a part without an HMAC ends
//! the download with an error; see [`super::cipher`].
//!
//! Shapes and defaults follow the Tapo app's clip save: its download
//! request, its stop request, and its read loop. Verified against an H200
//! on 2026-09-20: a 9 s clip arrived as 224 `video/mp2t` parts (2.7 MB)
//! with a clock reading of 8.428 s, and the hub ended it about 2 s after
//! the request.

use std::time::{Duration, Instant};

use anyhow::{Context, anyhow};
use log::{debug, trace, warn};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::error::{Error, TapoResponseError};
use crate::responses::RecordingDownloadResult;

use super::MediaStreamConnection;
use super::cipher::MediaCipher;
use super::mpeg_ts::StreamClock;
use super::multipart::{Part, PartParser, encode_client_part};

const CONTENT_TYPE_JSON: (&str, &str) = ("Content-Type", "application/json");
/// Requested by the Tapo app on the download request.
const DATA_WINDOW_SIZE: &str = "50";
/// The Tapo app acknowledges every 25th media part.
const ACK_EVERY: u64 = 25;
/// The heartbeat interval the Tapo app uses when the hub does not ask for
/// one, which an H200 never does.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
/// Spare capacity reserved before each socket read.
const READ_CHUNK_SIZE: usize = 64 * 1024;

/// Selects the recording to download.
#[derive(Debug)]
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
/// Gaps in `X-Data-Sequence` and control messages that need no action are
/// only logged.
///
/// # Errors
///
/// Returns an error if the hub rejects the request, sends no media at all, or
/// sends something that cannot be read: a control message or part that does
/// not parse, or a media part that does not carry a matching `X-Data-Hmac` or
/// cannot be decrypted. Also returns an error if the hub closes the stream or
/// the time limit runs out before the hub reports the end of the recording,
/// leaving only part of it in `sink`, and if reading from the hub, writing to
/// it, or writing to `sink` fails.
pub(crate) async fn download<W: AsyncWrite + Unpin>(
    connection: MediaStreamConnection,
    request: DownloadRequest,
    time_limit: Duration,
    sink: &mut W,
) -> Result<RecordingDownloadResult, Error> {
    let MediaStreamConnection {
        stream,
        buffered,
        cipher,
    } = connection;
    let (mut reader, mut writer) = stream.into_split();
    let mut parser = PartParser::new(buffered);

    let mut state = State {
        session_id: None,
        seq: 0,
        cipher,
        clock: StreamClock::default(),
        byte_count: 0,
        part_count: 0,
        last_sequence: None,
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
    let mut next_heartbeat = Instant::now() + HEARTBEAT_INTERVAL;

    'session: loop {
        while let Some(part) = parser.next_part()? {
            if !state
                .handle_part(&mut writer, sink, part, request_seq)
                .await?
            {
                break 'session;
            }
        }

        let now = Instant::now();
        if now >= deadline {
            debug!("Download time limit elapsed");
            return Err(anyhow!(
                "the recording did not finish downloading within {time_limit:?}, after {} bytes",
                state.byte_count
            )
            .into());
        }
        if now >= next_heartbeat {
            debug!("Sending a media stream heartbeat...");
            send(
                &mut writer,
                &state.session_headers(&[CONTENT_TYPE_JSON]),
                &Notification::new("heartbeat"),
            )
            .await?;
            next_heartbeat = now + HEARTBEAT_INTERVAL;
        }

        let wait = deadline.min(next_heartbeat).saturating_duration_since(now);
        match read(&mut reader, &mut parser, wait).await? {
            ReadOutcome::Data => {}
            // The read is cancel safe, so nothing is lost when it times out;
            // the loop then re-evaluates the heartbeat and the deadline.
            ReadOutcome::TimedOut => {}
            ReadOutcome::Eof => {
                debug!("The hub closed the media stream");
                return Err(anyhow!(
                    "the hub closed the media stream after {} bytes, before the end of the recording",
                    state.byte_count
                )
                .into());
            }
        }
    }

    // Best effort: the whole recording has arrived, so the session is over
    // either way.
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

    state.finish()
}

struct State {
    /// The session id issued for the download, echoed as `X-Session-Id` on
    /// client parts once known.
    session_id: Option<String>,
    seq: u32,
    cipher: MediaCipher,
    clock: StreamClock,
    byte_count: u64,
    part_count: u64,
    /// The last `X-Data-Sequence` seen, which the hub numbers from 1 without
    /// gaps.
    last_sequence: Option<u64>,
}

impl State {
    /// Turns the bookkeeping of a download the hub reported as finished into
    /// the caller's result, or into an error when it sent no media.
    fn finish(self) -> Result<RecordingDownloadResult, Error> {
        let duration_s = self.clock.elapsed().map(|elapsed| elapsed.as_secs_f64());
        debug!(
            "Download finished: {} parts, {} bytes, duration {:?}",
            self.part_count, self.byte_count, duration_s,
        );

        if self.part_count == 0 {
            return Err(anyhow!(
                "the hub reported the end of the recording without sending any media"
            )
            .into());
        }

        Ok(RecordingDownloadResult {
            byte_count: self.byte_count,
            duration_s,
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
                    debug!("Media stream notification: {event_type}");

                    if event_type == "stream_status"
                        && notification.status.as_deref() == Some("finished")
                    {
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

        if self.part_count == 0 {
            debug!("First media part headers: {:?}", part.headers);
        }

        let sequence = part
            .header("x-data-sequence")
            .and_then(|value| value.trim().parse::<u64>().ok());
        if let Some(sequence) = sequence {
            self.record_sequence(sequence);
        }

        let body = self.decrypt(&part)?;

        self.part_count += 1;
        self.clock.observe(&body);
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
            && sequence.saturating_sub(previous) > 1
        {
            warn!(
                "The media stream sequence jumped from {previous} to {sequence}, so the recording is missing footage"
            );
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

    /// Decrypts a media part after checking it against its `X-Data-Hmac`.
    ///
    /// # Errors
    ///
    /// Returns an error if the part has no HMAC or nonce, if the HMAC does not
    /// match, or if the body cannot be decrypted.
    fn decrypt(&self, part: &Part) -> Result<Vec<u8>, Error> {
        let Some(hmac) = part.header("x-data-hmac") else {
            return Err(anyhow!("media stream part without an X-Data-Hmac").into());
        };
        if !self.cipher.verify_hmac(&part.body, hmac) {
            return Err(anyhow!(
                "cannot decrypt the recording: the X-Data-Hmac of media stream part {} does not match the media stream keys",
                part.header("x-data-sequence").unwrap_or("?")
            )
            .into());
        }

        let Some(nonce) = part.header("x-nonce") else {
            return Err(anyhow!("encrypted media stream part without an X-Nonce").into());
        };

        Ok(self.cipher.decrypt(nonce, &part.body)?)
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
    use super::super::cipher::KeyExchange;
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
        let key_exchange =
            KeyExchange::parse("cipher=\"AES_128_CBC\" algorithm=\"HKDF\" nonce=\"N\" salt=\"S\"")
                .unwrap();

        State {
            session_id: None,
            seq: 0,
            cipher: MediaCipher::derive(&key_exchange, "HASH").unwrap(),
            clock: StreamClock::default(),
            byte_count: 0,
            part_count: 0,
            last_sequence: None,
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

    /// The sequence comes from the hub, so its largest value must not
    /// overflow the gap check.
    #[test]
    fn test_record_sequence_after_the_largest_sequence() {
        let mut state = state();

        state.record_sequence(u64::MAX);
        state.record_sequence(1);

        assert_eq!(state.last_sequence, Some(1));
    }
}
