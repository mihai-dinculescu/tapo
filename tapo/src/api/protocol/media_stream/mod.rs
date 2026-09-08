//! Media stream sessions with camera hubs.
//!
//! Camera hubs (H200, H500) serve live view and playback of hub-stored
//! recordings over a proprietary, stateful media session on TCP port 8800.
//! The session opens with a plain HTTP `POST /stream` request, which the hub
//! challenges with RFC 2617 Digest authentication. On success, the hub answers
//! `200` with the start of a `multipart/mixed` response whose body carries the
//! control and media parts for the rest of the connection. Verified against an
//! H200: the `200` is `HTTP/1.0`, advertises `X-Encrypt-Type: PLAIN` and a
//! `Key-Exchange` header, and omits both `X-Session-Id` and `X-Hb`. The Tapo
//! app tolerates the missing headers and judges success on the status alone.
//!
//! This module implements the handshake. [`multipart`] frames the parts that
//! flow in both directions afterwards and [`playback`] drives the control
//! channel to play back a recording.
//!
//! The password is pre-hashed before it enters the Digest computation: the
//! hub advertises `encrypt_type`, where `"3"` selects an upper-case hex SHA-256
//! of the password and anything else falls back to upper-case hex MD5.

mod multipart;
pub(crate) mod playback;

use std::collections::HashMap;
use std::io::{self, ErrorKind};
use std::time::Duration;

use anyhow::{Context, anyhow, bail};
use log::{debug, trace};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::error::{Error, TapoResponseError};
use crate::responses::MediaStreamSession;

use super::aes_ssl_cipher::generate_nonce;
use super::crypto;

const PORT: u16 = 8800;
const METHOD: &str = "POST";
const PATH: &str = "/stream";
/// The media stream authenticates as the hub's local `admin` account.
const USERNAME: &str = "admin";
const CLIENT_BOUNDARY: &str = "--client-stream-boundary--";
const NONCE_COUNT: &str = "00000001";
/// Upper bound on the size of an HTTP response head, so that a misbehaving
/// peer cannot grow the read buffer without bound.
const MAX_HEAD_SIZE: usize = 64 * 1024;
/// Response bodies larger than this are not drained; the connection is
/// re-established for the next request instead.
const MAX_DRAIN_SIZE: usize = 64 * 1024;

/// An authenticated media stream connection.
pub(crate) struct MediaStreamConnection {
    /// The socket, positioned somewhere inside the multipart body that
    /// follows the `200 OK` head.
    pub stream: TcpStream,
    /// Body bytes that were read together with the `200 OK` head. They are
    /// the start of the multipart body and must be parsed before anything
    /// else read from `stream`.
    pub buffered: Vec<u8>,
    pub session: MediaStreamSession,
}

/// Connects to the hub's media stream port and completes the Digest
/// authentication handshake.
///
/// `client_uuid` identifies this client to the hub (`X-Client-UUID`). It must
/// match the `player_id` of the recording searches whose results are played
/// back over the session. `timeout` bounds the whole handshake: connecting,
/// both request rounds, and a possible reconnect in between.
pub(crate) async fn authenticate(
    ip_address: &str,
    password: &str,
    client_uuid: &str,
    timeout: Duration,
) -> Result<MediaStreamConnection, Error> {
    match tokio::time::timeout(timeout, handshake(ip_address, password, client_uuid)).await {
        Ok(result) => result,
        Err(_) => Err(anyhow!("media stream handshake timed out after {timeout:?}").into()),
    }
}

async fn handshake(
    ip_address: &str,
    password: &str,
    client_uuid: &str,
) -> Result<MediaStreamConnection, Error> {
    let mut stream = connect(ip_address).await?;

    // Round 1: an unauthenticated request, which the hub answers with a
    // Digest challenge.
    debug!("Requesting the media stream Digest challenge...");
    let request = build_request(ip_address, client_uuid, None);
    let challenge_response = exchange(&mut stream, &request)
        .await?
        .ok_or_else(|| anyhow!("the hub closed the connection without sending a challenge"))?;
    challenge_response.log("challenge");

    let response = match challenge_response.status {
        401 => {
            let challenge = DigestChallenge::parse(&challenge_response)?;
            debug!("Media stream Digest challenge: {challenge:?}");

            let hashed_password = prehash_password(password, challenge.encrypt_type.as_deref());
            let cnonce = generate_nonce();
            let authorization =
                authorization_header(&challenge, USERNAME, &hashed_password, &cnonce);

            // Round 2: the same request, now carrying the Digest credentials.
            // The hub may have closed the connection after the challenge, in
            // which case the request is retried on a fresh one.
            debug!("Sending the media stream Digest credentials...");
            let request = build_request(ip_address, client_uuid, Some(&authorization));
            let mut response = if challenge_response.keep_alive() {
                exchange(&mut stream, &request).await?
            } else {
                None
            };
            if response.is_none() {
                debug!("Reconnecting for the authenticated media stream request...");
                stream = connect(ip_address).await?;
                response = exchange(&mut stream, &request).await?;
            }

            response.ok_or_else(|| {
                anyhow!("the hub closed the connection without answering the authenticated request")
            })?
        }
        // No challenge: the hub accepted the request as-is.
        _ => challenge_response,
    };
    response.log("authentication");

    match response.status {
        200 => {
            let session = MediaStreamSession::from(&response);
            debug!("Media stream session established: {session:?}");
            Ok(MediaStreamConnection {
                stream,
                buffered: response.body,
                session,
            })
        }
        401 => Err(Error::Tapo(TapoResponseError::Unauthorized {
            kind: "MEDIA_STREAM_DIGEST",
            description: "The hub rejected the Digest credentials for the media stream. Make sure that the TP-Link cloud password is correct.".to_string(),
        })),
        status => Err(Error::Tapo(TapoResponseError::HttpError {
            status_code: status,
            description: "Media stream handshake failed".to_string(),
        })),
    }
}

async fn connect(ip_address: &str) -> anyhow::Result<TcpStream> {
    let address = format!("{ip_address}:{PORT}");
    debug!("Connecting to the media stream at {address}...");

    TcpStream::connect(&address)
        .await
        .with_context(|| format!("connect to the media stream at {address}"))
}

fn build_request(ip_address: &str, client_uuid: &str, authorization: Option<&str>) -> String {
    // The header set mirrors the Tapo app, minus the generic ones it sends
    // (`User-Agent`, `Connection`, `Accept-Encoding`) that carry no
    // hub-specific meaning.
    let mut request = format!(
        "{METHOD} {PATH} HTTP/1.1\r\n\
         Host: {ip_address}:{PORT}\r\n\
         Content-Type: multipart/mixed; boundary={CLIENT_BOUNDARY}\r\n\
         X-Client-UUID: {client_uuid}\r\n\
         X-Preconn: 1\r\n\
         X-Key-Exchange: 1\r\n\
         Content-Length: 0\r\n"
    );

    if let Some(authorization) = authorization {
        request.push_str(&format!("Authorization: {authorization}\r\n"));
    }

    request.push_str("\r\n");
    request
}

/// Sends one HTTP request and reads the response head, draining any
/// `Content-Length` body so that the connection can be reused.
///
/// Returns `None` when the peer closed the connection before sending any
/// response bytes, which happens when the previous response was not
/// keep-alive; the caller can then retry on a fresh connection.
async fn exchange(stream: &mut TcpStream, request: &str) -> anyhow::Result<Option<HttpResponse>> {
    trace!("Media stream request (raw):\n{request}");

    if let Err(err) = stream.write_all(request.as_bytes()).await {
        if is_connection_closed(&err) {
            return Ok(None);
        }
        return Err(anyhow::Error::new(err).context("write media stream request"));
    }

    let mut buffer = Vec::with_capacity(4096);
    let head_end = loop {
        if let Some(end) = find_head_end(&buffer) {
            break end;
        }
        if buffer.len() >= MAX_HEAD_SIZE {
            bail!("media stream response head exceeds {MAX_HEAD_SIZE} bytes");
        }

        let mut chunk = [0u8; 4096];
        let read = match stream.read(&mut chunk).await {
            Ok(read) => read,
            Err(err) if buffer.is_empty() && is_connection_closed(&err) => return Ok(None),
            Err(err) => return Err(anyhow::Error::new(err).context("read media stream response")),
        };
        if read == 0 {
            if buffer.is_empty() {
                return Ok(None);
            }
            bail!("the hub closed the connection before the response head was complete");
        }
        buffer.extend_from_slice(&chunk[..read]);
    };

    let body = buffer.split_off(head_end + 4);
    trace!(
        "Media stream response (raw):\n{}",
        String::from_utf8_lossy(&buffer)
    );

    let mut response = HttpResponse::parse(&buffer[..head_end], body)?;

    // Only the challenge carries a body worth draining; a successful response
    // is followed by the multipart stream, which must be left in the socket.
    if response.status != 200 {
        drain_body(stream, &mut response).await?;
    }

    Ok(Some(response))
}

async fn drain_body(stream: &mut TcpStream, response: &mut HttpResponse) -> anyhow::Result<()> {
    if response.header("transfer-encoding").is_some() {
        // Not worth parsing a chunked challenge body: reconnect instead.
        response.reusable = false;
        return Ok(());
    }

    let content_length = response
        .header("content-length")
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(0);

    if content_length > MAX_DRAIN_SIZE {
        response.reusable = false;
        return Ok(());
    }

    let remaining = content_length.saturating_sub(response.body.len());
    if remaining > 0 {
        let mut rest = vec![0u8; remaining];
        stream
            .read_exact(&mut rest)
            .await
            .context("read media stream response body")?;
        response.body.extend_from_slice(&rest);
    }

    if !response.body.is_empty() {
        trace!(
            "Media stream response body (raw):\n{}",
            String::from_utf8_lossy(&response.body)
        );
    }

    Ok(())
}

fn find_head_end(buffer: &[u8]) -> Option<usize> {
    buffer.windows(4).position(|window| window == b"\r\n\r\n")
}

fn is_connection_closed(err: &io::Error) -> bool {
    matches!(
        err.kind(),
        ErrorKind::ConnectionReset
            | ErrorKind::ConnectionAborted
            | ErrorKind::BrokenPipe
            | ErrorKind::UnexpectedEof
    )
}

#[derive(Debug)]
struct HttpResponse {
    version: String,
    status: u16,
    /// Header names are lower-cased.
    headers: Vec<(String, String)>,
    /// Bytes received after the head, i.e. the start of the body.
    body: Vec<u8>,
    /// Whether the connection is still in a state where another request can
    /// be sent on it.
    reusable: bool,
}

impl HttpResponse {
    fn parse(head: &[u8], body: Vec<u8>) -> anyhow::Result<Self> {
        let head = std::str::from_utf8(head).context("media stream response head is not UTF-8")?;
        let mut lines = head.split("\r\n");

        let status_line = lines
            .next()
            .ok_or_else(|| anyhow!("media stream response is missing the status line"))?;
        let mut parts = status_line.split_whitespace();
        let version = parts
            .next()
            .ok_or_else(|| anyhow!("media stream response has an empty status line"))?
            .to_string();
        let status = parts
            .next()
            .and_then(|status| status.parse::<u16>().ok())
            .ok_or_else(|| {
                anyhow!("media stream response has an invalid status line: {status_line}")
            })?;

        let headers = lines
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let (name, value) = line.split_once(':')?;
                Some((name.trim().to_ascii_lowercase(), value.trim().to_string()))
            })
            .collect();

        Ok(Self {
            version,
            status,
            headers,
            body,
            reusable: true,
        })
    }

    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(header, _)| header == name)
            .map(|(_, value)| value.as_str())
    }

    fn keep_alive(&self) -> bool {
        if !self.reusable {
            return false;
        }

        match self.header("connection") {
            Some(value) if value.eq_ignore_ascii_case("close") => false,
            Some(value) if value.eq_ignore_ascii_case("keep-alive") => true,
            _ => self.version.eq_ignore_ascii_case("HTTP/1.1"),
        }
    }

    /// Logs the status line and every header, so that the exact shape of the
    /// hub's responses is visible while the protocol is being explored.
    fn log(&self, stage: &str) {
        debug!(
            "Media stream {stage} response: {} {}",
            self.version, self.status
        );
        for (name, value) in &self.headers {
            debug!("Media stream {stage} response header: {name}: {value}");
        }
    }
}

impl From<&HttpResponse> for MediaStreamSession {
    fn from(response: &HttpResponse) -> Self {
        // Every header is optional: the Tapo app defaults a missing session id
        // to an empty string and a missing heartbeat interval to 15 seconds,
        // and the H200 sends neither.
        let session_id = response.header("x-session-id").map(str::to_string);

        let heartbeat_interval_s = response
            .header("x-hb")
            .and_then(|value| value.trim().parse().ok());

        let key_exchange = response.header("key-exchange").map(str::to_string);

        Self {
            session_id,
            heartbeat_interval_s,
            key_exchange,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DigestAlgorithm {
    Md5,
    Sha256,
}

impl DigestAlgorithm {
    fn parse(token: Option<&str>) -> anyhow::Result<Self> {
        match token
            .map(|token| token.trim().to_ascii_uppercase())
            .as_deref()
        {
            None | Some("MD5") => Ok(Self::Md5),
            Some("SHA-256") => Ok(Self::Sha256),
            Some(other) => bail!("unsupported Digest algorithm `{other}`"),
        }
    }

    /// Lower-case hex digest, as RFC 2617 requires.
    fn hash_hex(self, data: &str) -> String {
        match self {
            Self::Md5 => base16ct::lower::encode_string(&crypto::md5(data.as_bytes())),
            Self::Sha256 => base16ct::lower::encode_string(&crypto::sha256(data.as_bytes())),
        }
    }
}

#[derive(Debug)]
struct DigestChallenge {
    realm: String,
    nonce: String,
    /// The `qop` selected from the ones the hub offers, if any.
    qop: Option<String>,
    opaque: Option<String>,
    algorithm: DigestAlgorithm,
    /// The `algorithm` token exactly as the hub sent it, echoed back.
    algorithm_token: Option<String>,
    /// The password pre-hash selector, which the hub sends as a non-standard
    /// parameter of the Digest challenge.
    encrypt_type: Option<String>,
}

impl DigestChallenge {
    fn parse(response: &HttpResponse) -> anyhow::Result<Self> {
        let header = response.header("www-authenticate").ok_or_else(|| {
            anyhow!("WWW-Authenticate header not found in the media stream challenge")
        })?;

        let mut params = parse_digest_params(header)?;

        let realm = params
            .remove("realm")
            .ok_or_else(|| anyhow!("Digest challenge is missing the realm"))?;
        let nonce = params
            .remove("nonce")
            .ok_or_else(|| anyhow!("Digest challenge is missing the nonce"))?;

        let qop = match params.remove("qop") {
            None => None,
            Some(offered) => {
                let supported = offered
                    .split(',')
                    .map(str::trim)
                    .find(|qop| qop.eq_ignore_ascii_case("auth"));
                match supported {
                    Some(qop) => Some(qop.to_string()),
                    None => bail!("unsupported Digest qop `{offered}`"),
                }
            }
        };

        let algorithm_token = params.remove("algorithm");
        let algorithm = DigestAlgorithm::parse(algorithm_token.as_deref())?;

        Ok(Self {
            realm,
            nonce,
            qop,
            opaque: params.remove("opaque"),
            algorithm,
            algorithm_token,
            encrypt_type: params.remove("encrypt_type"),
        })
    }
}

/// Parses the parameters of a `Digest` challenge into a map with lower-cased
/// keys. Quoted values may contain commas and backslash-escaped quotes.
fn parse_digest_params(header: &str) -> anyhow::Result<HashMap<String, String>> {
    let header = header.trim();
    let (scheme, rest) = header
        .split_once(char::is_whitespace)
        .unwrap_or((header, ""));
    if !scheme.eq_ignore_ascii_case("digest") {
        bail!("unsupported authentication scheme `{scheme}`");
    }

    let mut params = HashMap::new();
    let mut rest = rest.trim_start();

    while !rest.is_empty() {
        rest = rest.trim_start_matches(|c: char| c == ',' || c.is_whitespace());
        if rest.is_empty() {
            break;
        }

        let Some(separator) = rest.find('=') else {
            break;
        };
        let key = rest[..separator].trim().to_ascii_lowercase();
        rest = rest[separator + 1..].trim_start();

        let value = if let Some(quoted) = rest.strip_prefix('"') {
            let mut value = String::new();
            let mut escaped = false;
            let mut end = quoted.len();

            for (index, c) in quoted.char_indices() {
                if escaped {
                    value.push(c);
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == '"' {
                    end = index;
                    break;
                } else {
                    value.push(c);
                }
            }

            rest = quoted[end..].strip_prefix('"').unwrap_or("");
            value
        } else {
            let end = rest.find(',').unwrap_or(rest.len());
            let value = rest[..end].trim().to_string();
            rest = &rest[end..];
            value
        };

        params.insert(key, value);
    }

    Ok(params)
}

/// Pre-hashes the password the way the hub expects it in the Digest
/// computation.
fn prehash_password(password: &str, encrypt_type: Option<&str>) -> String {
    match encrypt_type.map(str::trim) {
        Some("3") => crypto::sha256_hex(password.as_bytes()),
        _ => crypto::md5_hex(password.as_bytes()),
    }
}

fn digest_response(
    challenge: &DigestChallenge,
    username: &str,
    password: &str,
    method: &str,
    uri: &str,
    cnonce: &str,
) -> String {
    let algorithm = challenge.algorithm;
    let ha1 = algorithm.hash_hex(&format!("{username}:{}:{password}", challenge.realm));
    let ha2 = algorithm.hash_hex(&format!("{method}:{uri}"));

    match &challenge.qop {
        Some(qop) => algorithm.hash_hex(&format!(
            "{ha1}:{}:{NONCE_COUNT}:{cnonce}:{qop}:{ha2}",
            challenge.nonce
        )),
        None => algorithm.hash_hex(&format!("{ha1}:{}:{ha2}", challenge.nonce)),
    }
}

fn authorization_header(
    challenge: &DigestChallenge,
    username: &str,
    password: &str,
    cnonce: &str,
) -> String {
    let response = digest_response(challenge, username, password, METHOD, PATH, cnonce);

    let mut parts = vec![
        format!("username=\"{username}\""),
        format!("realm=\"{}\"", challenge.realm),
        format!("uri=\"{PATH}\""),
    ];
    if let Some(algorithm) = &challenge.algorithm_token {
        parts.push(format!("algorithm={algorithm}"));
    }
    parts.push(format!("nonce=\"{}\"", challenge.nonce));
    if let Some(qop) = &challenge.qop {
        parts.push(format!("nc={NONCE_COUNT}"));
        parts.push(format!("cnonce=\"{cnonce}\""));
        parts.push(format!("qop={qop}"));
    }
    parts.push(format!("response=\"{response}\""));
    if let Some(opaque) = &challenge.opaque {
        parts.push(format!("opaque=\"{opaque}\""));
    }

    format!("Digest {}", parts.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The worked example of RFC 2617, section 3.5.
    fn rfc2617_challenge() -> DigestChallenge {
        DigestChallenge {
            realm: "testrealm@host.com".to_string(),
            nonce: "dcd98b7102dd2f0e8b11d0f600bfb0c093".to_string(),
            qop: Some("auth".to_string()),
            opaque: Some("5ccc069c403ebaf9f0171e9517f40e41".to_string()),
            algorithm: DigestAlgorithm::Md5,
            algorithm_token: Some("MD5".to_string()),
            encrypt_type: None,
        }
    }

    /// The inputs of the worked example of RFC 7616, section 3.9.1.
    fn rfc7616_challenge(algorithm: DigestAlgorithm, token: &str) -> DigestChallenge {
        DigestChallenge {
            realm: "http-auth@example.org".to_string(),
            nonce: "7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v".to_string(),
            qop: Some("auth".to_string()),
            opaque: Some("FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS".to_string()),
            algorithm,
            algorithm_token: Some(token.to_string()),
            encrypt_type: None,
        }
    }

    const RFC7616_CNONCE: &str = "f2/wE4q74E6j5s5f8s/9XFUfoMmMU3f4uJc/j+9F5H/f7HQ";

    #[test]
    fn test_digest_response_md5_matches_rfc2617() {
        let challenge = rfc2617_challenge();

        let response = digest_response(
            &challenge,
            "Mufasa",
            "Circle Of Life",
            "GET",
            "/dir/index.html",
            "0a4f113b",
        );

        assert_eq!(response, "6629fae49393a05397450978507c4ef1");
    }

    #[test]
    fn test_digest_response_sha256() {
        let challenge = rfc7616_challenge(DigestAlgorithm::Sha256, "SHA-256");

        let response = digest_response(
            &challenge,
            "Mufasa",
            "Circle of Life",
            "GET",
            "/dir/index.html",
            RFC7616_CNONCE,
        );

        // The response printed in RFC 7616 does not follow from its own
        // inputs, so the expected value was computed independently with
        // Python's hashlib instead.
        assert_eq!(
            response,
            "8b24653bbf0362a826d8dff97426f25c08fe5a440099c7a227bcc34f7b909352"
        );
    }

    #[test]
    fn test_digest_response_without_qop_uses_rfc2069_form() {
        let mut challenge = rfc2617_challenge();
        challenge.qop = None;

        let response = digest_response(&challenge, "Mufasa", "Circle Of Life", "GET", "/", "x");

        let ha1 = DigestAlgorithm::Md5.hash_hex("Mufasa:testrealm@host.com:Circle Of Life");
        let ha2 = DigestAlgorithm::Md5.hash_hex("GET:/");
        let expected = DigestAlgorithm::Md5.hash_hex(&format!("{ha1}:{}:{ha2}", challenge.nonce));
        assert_eq!(response, expected);
    }

    #[test]
    fn test_authorization_header_lists_the_digest_fields_in_order() {
        let challenge = rfc7616_challenge(DigestAlgorithm::Sha256, "SHA-256");

        let header = authorization_header(&challenge, "admin", "HASH", "CNONCE");

        let expected_response =
            digest_response(&challenge, "admin", "HASH", "POST", "/stream", "CNONCE");
        assert_eq!(
            header,
            format!(
                "Digest username=\"admin\", realm=\"http-auth@example.org\", uri=\"/stream\", \
                 algorithm=SHA-256, nonce=\"7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v\", \
                 nc=00000001, cnonce=\"CNONCE\", qop=auth, response=\"{expected_response}\", \
                 opaque=\"FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS\""
            )
        );
    }

    #[test]
    fn test_authorization_header_omits_absent_fields() {
        let challenge = DigestChallenge {
            realm: "hub".to_string(),
            nonce: "NONCE".to_string(),
            qop: None,
            opaque: None,
            algorithm: DigestAlgorithm::Md5,
            algorithm_token: None,
            encrypt_type: None,
        };

        let header = authorization_header(&challenge, "admin", "HASH", "CNONCE");

        assert!(header.starts_with(
            "Digest username=\"admin\", realm=\"hub\", uri=\"/stream\", nonce=\"NONCE\", response=\""
        ));
        assert!(!header.contains("algorithm="));
        assert!(!header.contains("nc="));
        assert!(!header.contains("cnonce="));
        assert!(!header.contains("qop="));
        assert!(!header.contains("opaque="));
    }

    #[test]
    fn test_prehash_password() {
        assert_eq!(
            prehash_password("hello", Some("3")),
            "2CF24DBA5FB0A30E26E83B2AC5B9E29E1B161E5C1FA7425E73043362938B9824"
        );
        assert_eq!(
            prehash_password("hello", None),
            "5D41402ABC4B2A76B9719D911017C592"
        );
        assert_eq!(
            prehash_password("hello", Some("1")),
            "5D41402ABC4B2A76B9719D911017C592"
        );
    }

    #[test]
    fn test_parse_digest_params() {
        let params = parse_digest_params(
            "Digest realm=\"TP-Link IP-Camera\", nonce=\"abc,def\", qop=\"auth,auth-int\", \
             Algorithm=SHA-256 , opaque=\"say \\\"hi\\\"\", encrypt_type=\"3\"",
        )
        .unwrap();

        assert_eq!(params["realm"], "TP-Link IP-Camera");
        assert_eq!(params["nonce"], "abc,def");
        assert_eq!(params["qop"], "auth,auth-int");
        assert_eq!(params["algorithm"], "SHA-256");
        assert_eq!(params["opaque"], "say \"hi\"");
        assert_eq!(params["encrypt_type"], "3");
    }

    #[test]
    fn test_parse_digest_params_rejects_other_schemes() {
        assert!(parse_digest_params("Basic realm=\"hub\"").is_err());
    }

    #[test]
    fn test_digest_challenge_parse() {
        let response = HttpResponse::parse(
            b"HTTP/1.1 401 Unauthorized\r\n\
              WWW-Authenticate: Digest realm=\"hub\", nonce=\"N\", qop=\"auth-int,auth\", opaque=\"O\", algorithm=MD5\r\n\
              Content-Length: 0",
            Vec::new(),
        )
        .unwrap();

        let challenge = DigestChallenge::parse(&response).unwrap();

        assert_eq!(challenge.realm, "hub");
        assert_eq!(challenge.nonce, "N");
        assert_eq!(challenge.qop.as_deref(), Some("auth"));
        assert_eq!(challenge.opaque.as_deref(), Some("O"));
        assert_eq!(challenge.algorithm, DigestAlgorithm::Md5);
        assert_eq!(challenge.algorithm_token.as_deref(), Some("MD5"));
        assert_eq!(challenge.encrypt_type, None);
    }

    #[test]
    fn test_digest_challenge_parse_minimal() {
        let response = HttpResponse::parse(
            b"HTTP/1.1 401 Unauthorized\r\n\
              WWW-Authenticate: Digest realm=\"hub\", nonce=\"N\", encrypt_type=\"3\"",
            Vec::new(),
        )
        .unwrap();

        let challenge = DigestChallenge::parse(&response).unwrap();

        assert_eq!(challenge.encrypt_type.as_deref(), Some("3"));
        assert_eq!(challenge.qop, None);
        assert_eq!(challenge.opaque, None);
        assert_eq!(challenge.algorithm, DigestAlgorithm::Md5);
        assert_eq!(challenge.algorithm_token, None);
    }

    #[test]
    fn test_digest_challenge_rejects_unsupported_algorithm_and_qop() {
        let response = HttpResponse::parse(
            b"HTTP/1.1 401 Unauthorized\r\n\
              WWW-Authenticate: Digest realm=\"hub\", nonce=\"N\", algorithm=MD5-sess",
            Vec::new(),
        )
        .unwrap();
        assert!(DigestChallenge::parse(&response).is_err());

        let response = HttpResponse::parse(
            b"HTTP/1.1 401 Unauthorized\r\n\
              WWW-Authenticate: Digest realm=\"hub\", nonce=\"N\", qop=\"auth-int\"",
            Vec::new(),
        )
        .unwrap();
        assert!(DigestChallenge::parse(&response).is_err());
    }

    #[test]
    fn test_http_response_parse() {
        let response = HttpResponse::parse(
            b"HTTP/1.1 200 OK\r\nX-Session-Id: 42\r\nx-hb: 10\r\nKey-Exchange: KEY\r\nConnection: close",
            b"body".to_vec(),
        )
        .unwrap();

        assert_eq!(response.version, "HTTP/1.1");
        assert_eq!(response.status, 200);
        assert_eq!(response.header("x-session-id"), Some("42"));
        assert_eq!(response.header("x-hb"), Some("10"));
        assert_eq!(response.body, b"body");
        assert!(!response.keep_alive());

        let session = MediaStreamSession::from(&response);
        assert_eq!(session.session_id.as_deref(), Some("42"));
        assert_eq!(session.heartbeat_interval_s, Some(10));
        assert_eq!(session.key_exchange.as_deref(), Some("KEY"));
    }

    #[test]
    fn test_http_response_keep_alive() {
        let http11 = HttpResponse::parse(b"HTTP/1.1 401 Unauthorized", Vec::new()).unwrap();
        assert!(http11.keep_alive());

        let http10 = HttpResponse::parse(b"HTTP/1.0 401 Unauthorized", Vec::new()).unwrap();
        assert!(!http10.keep_alive());

        let http10_keep_alive = HttpResponse::parse(
            b"HTTP/1.0 401 Unauthorized\r\nConnection: keep-alive",
            Vec::new(),
        )
        .unwrap();
        assert!(http10_keep_alive.keep_alive());
    }

    #[test]
    fn test_http_response_parse_rejects_invalid_status_line() {
        assert!(HttpResponse::parse(b"HTTP/1.1 OK", Vec::new()).is_err());
        assert!(HttpResponse::parse(b"", Vec::new()).is_err());
    }

    /// The `200` an H200 (firmware 1.6.5) actually sends: no session id, no
    /// heartbeat interval, and a structured `Key-Exchange` value.
    #[test]
    fn test_media_stream_session_from_h200_response() {
        let response = HttpResponse::parse(
            b"HTTP/1.0 200 OK\r\n\
              Server: streamd\r\n\
              Content-Type: multipart/mixed;boundary=--device-stream-boundary--\r\n\
              X-Encrypt-Type: PLAIN\r\n\
              Pragma: no-cache\r\n\
              Cache-Control: no-cache\r\n\
              Key-Exchange: cipher=\"AES_128_CBC\" username=\"admin\" padding=\"PKCS7_16\" algorithm=\"HKDF\" nonce=\"4514f88f1148a6735bdc6a7d7b93b0b0\" salt=\"f9192a9ee24bc7db8df141bf2bd56af4\"\r\n\
              Connection: close",
            Vec::new(),
        )
        .unwrap();

        let session = MediaStreamSession::from(&response);

        assert_eq!(session.session_id, None);
        assert_eq!(session.heartbeat_interval_s, None);
        assert_eq!(
            session.key_exchange.as_deref(),
            Some(
                "cipher=\"AES_128_CBC\" username=\"admin\" padding=\"PKCS7_16\" algorithm=\"HKDF\" nonce=\"4514f88f1148a6735bdc6a7d7b93b0b0\" salt=\"f9192a9ee24bc7db8df141bf2bd56af4\""
            )
        );
    }

    #[test]
    fn test_find_head_end() {
        assert_eq!(find_head_end(b"HTTP/1.1 200 OK\r\n\r\nbody"), Some(15));
        assert_eq!(find_head_end(b"HTTP/1.1 200 OK\r\n"), None);
    }

    #[test]
    fn test_build_request() {
        let request = build_request("192.168.1.100", "UUID", None);
        assert_eq!(
            request,
            "POST /stream HTTP/1.1\r\n\
             Host: 192.168.1.100:8800\r\n\
             Content-Type: multipart/mixed; boundary=--client-stream-boundary--\r\n\
             X-Client-UUID: UUID\r\n\
             X-Preconn: 1\r\n\
             X-Key-Exchange: 1\r\n\
             Content-Length: 0\r\n\
             \r\n"
        );

        let request = build_request("192.168.1.100", "UUID", Some("Digest x"));
        assert!(request.ends_with("Content-Length: 0\r\nAuthorization: Digest x\r\n\r\n"));
    }
}
