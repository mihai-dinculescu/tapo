//! RTSP MJPEG snapshot helper.
//!
//! Captures a single JPEG frame from a Tapo camera's MJPEG RTSP stream.
//!
//! Tapo PTZ cameras (verified on the C220) advertise an MJPEG profile but
//! deviate from [RFC 2435](https://www.rfc-editor.org/rfc/rfc2435.txt): the
//! payload of every RTP packet (after the 8-byte RTP/JPEG main header and any
//! optional sub-headers) carries a slice of an already-complete JFIF file —
//! `FF D8` … `FF D9`, including DQT/SOF/DHT — rather than the bare entropy-
//! coded scan that the RFC requires. Retina's `Depacketizer` faithfully
//! reconstructs JPEG headers from the RTP/JPEG main header fields and prepends
//! them to the body, which here produces a malformed double-headered JPEG that
//! decoders reject (the symptom is a near-black image with a thin band of
//! valid pixels at the top).
//!
//! To avoid that, this module subscribes to raw RTP packets and concatenates
//! the post-header bytes of all fragments belonging to one frame. The result
//! is the camera's original JPEG, byte-for-byte.

use std::time::Duration;

use anyhow::{Context, anyhow};
use retina::client::{
    Credentials, PacketItem, PlayOptions, Session, SessionOptions, SetupOptions,
    TcpTransportOptions, Transport,
};
use tokio_stream::StreamExt;

use crate::error::Error;

/// Capture a single JPEG frame from an MJPEG-encoded RTSP stream.
pub(crate) async fn grab_mjpeg_frame(
    url: &str,
    creds: Credentials,
    timeout: Duration,
) -> Result<Vec<u8>, Error> {
    let parsed_url = reqwest::Url::parse(url).context("parse RTSP URL")?;

    let opts = SessionOptions::default()
        .creds(Some(creds))
        .user_agent("tapo".into());

    let mut session = Session::describe(parsed_url, opts)
        .await
        .context("RTSP DESCRIBE")?;

    let video_idx = session
        .streams()
        .iter()
        .position(|s| (s.media() == "video" || s.media() == "image") && s.encoding_name() == "jpeg")
        .ok_or_else(|| anyhow!("no MJPEG video stream in RTSP SDP"))?;

    session
        .setup(
            video_idx,
            SetupOptions::default().transport(Transport::Tcp(TcpTransportOptions::default())),
        )
        .await
        .context("RTSP SETUP")?;

    let mut session = session
        .play(PlayOptions::default())
        .await
        .context("RTSP PLAY")?;

    tokio::time::timeout(timeout, async {
        let mut frame: Vec<u8> = Vec::with_capacity(64 * 1024);
        let mut have_start = false;

        loop {
            let item = session
                .next()
                .await
                .ok_or_else(|| anyhow!("RTSP stream ended before a frame arrived"))?
                .context("RTSP packet")?;

            let pkt = match item {
                PacketItem::Rtp(p) => p,
                _ => continue,
            };

            let mark = pkt.mark();
            let payload = pkt.payload();
            // RFC 2435 main header is 8 bytes:
            //   0: type-specific
            //   1..4: fragment offset (24-bit big-endian)
            //   4: type
            //   5: Q
            //   6: width / 8
            //   7: height / 8
            if payload.len() < 8 {
                continue;
            }
            let frag_offset = u32::from_be_bytes([0, payload[1], payload[2], payload[3]]);
            let type_field = payload[4];
            let q = payload[5];
            let mut body_offset = 8usize;
            // Restart-marker header (4 bytes), present when type >= 64.
            if type_field >= 64 {
                if payload.len() < body_offset + 4 {
                    continue;
                }
                body_offset += 4;
            }
            // Quantization-table header on the first fragment when Q >= 128.
            if frag_offset == 0 && q >= 128 {
                if payload.len() < body_offset + 4 {
                    continue;
                }
                let length = ((payload[body_offset + 2] as usize) << 8)
                    | (payload[body_offset + 3] as usize);
                body_offset += 4 + length;
                if payload.len() < body_offset {
                    continue;
                }
            }

            if frag_offset == 0 {
                frame.clear();
                have_start = true;
            } else if !have_start {
                // Joined mid-frame; wait for the next first fragment.
                continue;
            }

            frame.extend_from_slice(&payload[body_offset..]);

            if mark && have_start {
                // Tapo cameras embed a full JFIF (SOI..EOI) in the RTP body,
                // so `frame` is already a complete JPEG. Verify the SOI/EOI
                // bookends and emit it; otherwise wait for the next frame.
                let n = frame.len();
                let has_soi = n >= 4 && frame[0] == 0xff && frame[1] == 0xd8;
                let has_eoi = n >= 4 && frame[n - 2] == 0xff && frame[n - 1] == 0xd9;
                if has_soi && has_eoi {
                    return Ok::<Vec<u8>, anyhow::Error>(frame);
                }
                frame.clear();
                have_start = false;
            }
        }
    })
    .await
    .map_err(|_| anyhow!("RTSP snapshot timed out after {timeout:?}"))?
    .map_err(Into::into)
}
