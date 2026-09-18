//! Media stream sessions with camera hubs.
//!
//! Camera hubs (H200, H500) serve live view and playback of hub-stored
//! recordings over a proprietary, stateful media session on TCP port 8800.
//! The session opens with a plain HTTP `POST /stream` request, which the hub
//! challenges with RFC 2617 Digest authentication. On success, the hub answers
//! `200` with the start of a `multipart/mixed` response whose body carries the
//! control and media parts for the rest of the connection. Verified against an
//! H200: the `200` is `HTTP/1.0`, advertises `X-Encrypt-Type: PLAIN` and a
//! `Key-Exchange` header, and omits both `X-Session-Id` and `X-Hb`. Like the
//! Tapo app, the handshake judges success on the status alone.
//!
//! [`handshake`] implements the handshake. [`multipart`] frames the parts that
//! flow in both directions afterwards and [`playback`] drives the control
//! channel to play back a recording.

mod cipher;
mod handshake;
mod mpeg_ts;
mod multipart;

pub(crate) mod playback;
pub(crate) use handshake::*;
