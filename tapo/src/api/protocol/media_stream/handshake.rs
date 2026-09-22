//! The Digest authentication handshake that opens a media stream session.
//!
//! The request URI names what to stream
//! (`/stream?deviceId=…&type=download&playerId=…&media_type=0`) and the Digest
//! `uri` covers path and query, the way the app asks. The app also has a
//! pre-connected session it keeps idle (`X-Preconn: 1`, bare `/stream`), but
//! an H200 accepted a recording request as JSON on such a session and never
//! answered it, so it is of no use here.
//!
//! Only the challenge an H200 sends is supported: `algorithm="SHA-256"`,
//! `qop="auth"` and `encrypt_type="3"`. The last is a non-standard parameter
//! that selects how the password is pre-hashed before it enters the Digest
//! computation, and `"3"` means upper-case hex SHA-256.

use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Context, anyhow, bail};
use log::{debug, trace};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::api::protocol::aes_ssl_cipher::generate_nonce;
use crate::api::protocol::crypto;
use crate::error::{Error, TapoResponseError};

use super::cipher::{KeyExchange, MediaCipher};

const PORT: u16 = 8800;
const METHOD: &str = "POST";
const PATH: &str = "/stream";
/// The media stream authenticates as the hub's local `admin` account.
const USERNAME: &str = "admin";
const CLIENT_BOUNDARY: &str = "--client-stream-boundary--";
const ALGORITHM: &str = "SHA-256";
const QOP: &str = "auth";
const ENCRYPT_TYPE: &str = "3";
const NONCE_COUNT: &str = "00000001";
/// Upper bound on the size of an HTTP response head, so that a misbehaving
/// peer cannot grow the read buffer without bound.
const MAX_HEAD_SIZE: usize = 64 * 1024;

/// An authenticated media stream connection.
pub(crate) struct MediaStreamConnection {
    /// The socket, positioned somewhere inside the multipart body that
    /// follows the `200 OK` head.
    pub(super) stream: TcpStream,
    /// Body bytes that were read together with the `200 OK` head. They are
    /// the start of the multipart body and must be parsed before anything
    /// else read from `stream`.
    pub(super) buffered: Vec<u8>,
    /// The media cipher, derived from the `Key-Exchange` header of the
    /// `200 OK`.
    pub(super) cipher: MediaCipher,
}

/// Connects to the hub's media stream port and completes the Digest
/// authentication handshake for downloading from the camera `device_id`,
/// with `player_id` identifying the player.
///
/// `timeout` bounds the whole handshake: connecting, both request rounds,
/// and the reconnect in between.
pub(crate) async fn authenticate(
    ip_address: &str,
    password: &str,
    device_id: &str,
    player_id: &str,
    timeout: Duration,
) -> Result<MediaStreamConnection, Error> {
    let uri = stream_uri(device_id, player_id);
    match tokio::time::timeout(timeout, handshake(ip_address, password, &uri)).await {
        Ok(result) => result,
        Err(_) => Err(anyhow!("media stream handshake timed out after {timeout:?}").into()),
    }
}

/// The request URI: path plus query. The Digest `uri` covers all of it.
fn stream_uri(device_id: &str, player_id: &str) -> String {
    // The app names a hub's camera by `camera_mac` here and falls back to
    // `deviceId` for other sub-devices; an H200 accepts either.
    // `media_type` 0 selects video.
    format!("{PATH}?deviceId={device_id}&type=download&playerId={player_id}&media_type=0")
}

async fn handshake(
    ip_address: &str,
    password: &str,
    uri: &str,
) -> Result<MediaStreamConnection, Error> {
    let mut stream = connect(ip_address).await?;

    // Round 1: an unauthenticated request, which the hub answers with a
    // Digest challenge.
    debug!("Requesting the media stream Digest challenge for {uri}...");
    let request = build_request(ip_address, uri, None);
    let challenge_response = exchange(&mut stream, &request)
        .await
        .context("request the media stream Digest challenge")?;
    challenge_response.log("challenge");

    // Like the Tapo app, give up on anything but a challenge.
    if challenge_response.status != 401 {
        return Err(Error::Tapo(TapoResponseError::HttpError {
            status_code: challenge_response.status,
            description: "The hub did not challenge the media stream request".to_string(),
        }));
    }

    let challenge = DigestChallenge::parse(&challenge_response)?;
    debug!("Media stream Digest challenge: {challenge:?}");

    // `encrypt_type` 3, the only one the challenge accepts.
    let password_hash = crypto::sha256_hex(password.as_bytes());
    let cnonce = generate_nonce();
    let authorization = authorization_header(&challenge, USERNAME, &password_hash, uri, &cnonce);

    // Round 2: the same request, now carrying the Digest credentials. The hub
    // closes the connection after the challenge (`Connection: close`), so the
    // request goes out on a fresh one.
    debug!("Sending the media stream Digest credentials...");
    stream = connect(ip_address).await?;
    let request = build_request(ip_address, uri, Some(&authorization));
    let response = exchange(&mut stream, &request)
        .await
        .context("send the media stream Digest credentials")?;
    response.log("authentication");

    match response.status {
        200 => {
            // The Tapo app uses the pre-hashed password as the secret of the
            // media cipher too.
            let cipher = media_cipher(&response, &password_hash)?;
            debug!("Media stream session established");

            Ok(MediaStreamConnection {
                stream,
                buffered: response.body,
                cipher,
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

fn build_request(ip_address: &str, uri: &str, authorization: Option<&str>) -> String {
    // The header set mirrors the Tapo app, minus the generic ones it sends
    // (`User-Agent`, `Connection`, `Accept-Encoding`) that carry no
    // hub-specific meaning.
    let mut request = format!(
        "{METHOD} {uri} HTTP/1.1\r\n\
         Host: {ip_address}:{PORT}\r\n\
         Content-Type: multipart/mixed; boundary={CLIENT_BOUNDARY}\r\n\
         X-Key-Exchange: 1\r\n\
         Content-Length: 0\r\n"
    );

    if let Some(authorization) = authorization {
        request.push_str(&format!("Authorization: {authorization}\r\n"));
    }

    request.push_str("\r\n");
    request
}

/// Derives the media cipher from the `Key-Exchange` header of the `200 OK`,
/// e.g. `cipher="AES_128_CBC" username="admin" padding="PKCS7_16"
/// algorithm="HKDF" nonce="…" salt="…"`.
fn media_cipher(response: &HttpResponse, secret: &str) -> anyhow::Result<MediaCipher> {
    let key_exchange = response
        .header("key-exchange")
        .ok_or_else(|| anyhow!("the hub accepted the media stream but sent no Key-Exchange"))?;
    let key_exchange =
        KeyExchange::parse(key_exchange).context("invalid media stream Key-Exchange")?;

    MediaCipher::derive(&key_exchange, secret).context("derive the media stream keys")
}

/// Sends one HTTP request and reads the response head. Bytes read past the
/// head are kept as the start of the body.
async fn exchange(stream: &mut TcpStream, request: &str) -> anyhow::Result<HttpResponse> {
    trace!("Media stream request (raw):\n{request}");

    stream
        .write_all(request.as_bytes())
        .await
        .context("write media stream request")?;

    let mut buffer = Vec::with_capacity(4096);
    let head_end = loop {
        if let Some(end) = find_head_end(&buffer) {
            break end;
        }
        if buffer.len() >= MAX_HEAD_SIZE {
            bail!("media stream response head exceeds {MAX_HEAD_SIZE} bytes");
        }

        let mut chunk = [0u8; 4096];
        let read = stream
            .read(&mut chunk)
            .await
            .context("read media stream response")?;
        if read == 0 {
            if buffer.is_empty() {
                bail!("the hub closed the connection without responding");
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

    HttpResponse::parse(&buffer[..head_end], body)
}

fn find_head_end(buffer: &[u8]) -> Option<usize> {
    buffer.windows(4).position(|window| window == b"\r\n\r\n")
}

#[derive(Debug)]
struct HttpResponse {
    version: String,
    status: u16,
    /// Header names are lower-cased.
    headers: Vec<(String, String)>,
    /// Bytes received after the head, i.e. the start of the body.
    body: Vec<u8>,
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

        let headers: Vec<(String, String)> = lines
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
        })
    }

    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(header, _)| header == name)
            .map(|(_, value)| value.as_str())
    }

    /// Logs the status line and every header at debug level. At that level
    /// they are the only record of the headers, since `exchange` logs the raw
    /// head at trace level.
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

#[derive(Debug)]
struct DigestChallenge {
    realm: String,
    nonce: String,
    opaque: String,
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
        let opaque = params
            .remove("opaque")
            .ok_or_else(|| anyhow!("Digest challenge is missing the opaque"))?;

        match params.remove("algorithm") {
            Some(algorithm) if algorithm.eq_ignore_ascii_case(ALGORITHM) => {}
            Some(algorithm) => bail!("unsupported Digest algorithm `{algorithm}`"),
            None => bail!("Digest challenge is missing the algorithm"),
        }
        match params.remove("qop") {
            Some(qop) if qop.eq_ignore_ascii_case(QOP) => {}
            Some(qop) => bail!("unsupported Digest qop `{qop}`"),
            None => bail!("Digest challenge is missing the qop"),
        }
        match params.remove("encrypt_type") {
            Some(encrypt_type) if encrypt_type == ENCRYPT_TYPE => {}
            Some(encrypt_type) => bail!("unsupported Digest encrypt_type `{encrypt_type}`"),
            None => bail!("Digest challenge is missing the encrypt_type"),
        }

        Ok(Self {
            realm,
            nonce,
            opaque,
        })
    }
}

/// Lower-case hex SHA-256, as RFC 7616 requires.
fn hash_hex(data: &str) -> String {
    base16ct::lower::encode_string(&crypto::sha256(data.as_bytes()))
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

fn digest_response(
    challenge: &DigestChallenge,
    username: &str,
    password: &str,
    method: &str,
    uri: &str,
    cnonce: &str,
) -> String {
    let ha1 = hash_hex(&format!("{username}:{}:{password}", challenge.realm));
    let ha2 = hash_hex(&format!("{method}:{uri}"));

    hash_hex(&format!(
        "{ha1}:{}:{NONCE_COUNT}:{cnonce}:{QOP}:{ha2}",
        challenge.nonce
    ))
}

fn authorization_header(
    challenge: &DigestChallenge,
    username: &str,
    password: &str,
    uri: &str,
    cnonce: &str,
) -> String {
    let response = digest_response(challenge, username, password, METHOD, uri, cnonce);

    let parts = [
        format!("username=\"{username}\""),
        format!("realm=\"{}\"", challenge.realm),
        format!("uri=\"{uri}\""),
        format!("algorithm={ALGORITHM}"),
        format!("nonce=\"{}\"", challenge.nonce),
        format!("nc={NONCE_COUNT}"),
        format!("cnonce=\"{cnonce}\""),
        format!("qop={QOP}"),
        format!("response=\"{response}\""),
        format!("opaque=\"{}\"", challenge.opaque),
    ];

    format!("Digest {}", parts.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The inputs of the worked example of RFC 7616, section 3.9.1.
    fn rfc7616_challenge() -> DigestChallenge {
        DigestChallenge {
            realm: "http-auth@example.org".to_string(),
            nonce: "7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v".to_string(),
            opaque: "FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS".to_string(),
        }
    }

    const RFC7616_CNONCE: &str = "f2/wE4q74E6j5s5f8s/9XFUfoMmMU3f4uJc/j+9F5H/f7HQ";

    #[test]
    fn test_digest_response_sha256() {
        let challenge = rfc7616_challenge();

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
    fn test_authorization_header_lists_the_digest_fields_in_order() {
        let challenge = rfc7616_challenge();

        let header = authorization_header(&challenge, "admin", "HASH", "/stream", "CNONCE");

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

    /// The challenge an H200 (firmware 1.6.5) actually sends.
    #[test]
    fn test_digest_challenge_parse_h200() {
        let response = HttpResponse::parse(
            b"HTTP/1.1 401 Unauthorized\r\n\
              Content-Length: 14\r\n\
              Cache-Control: no-cache\r\n\
              Connection: close\r\n\
              WWW-Authenticate: Digest realm=\"TP-Link IP-Camera\",qop=\"auth\",nonce=\"fe4aeebd2a29d6f6b5fb962b41bbff90\",opaque=\"e34536e09a6eb618a5e12649897e7440\",algorithm=\"SHA-256\",encrypt_type=\"3\"\r\n\
              Set-Cookie: TP_HTTP_COOKIE=fe4aeebd2a29d6f6b5fb962b41bbff90; path=/",
            b"HTTP ERROR 401".to_vec(),
        )
        .unwrap();

        assert_eq!(response.status, 401);

        let challenge = DigestChallenge::parse(&response).unwrap();

        assert_eq!(challenge.realm, "TP-Link IP-Camera");
        assert_eq!(challenge.nonce, "fe4aeebd2a29d6f6b5fb962b41bbff90");
        assert_eq!(challenge.opaque, "e34536e09a6eb618a5e12649897e7440");
    }

    /// Only the `algorithm`, `qop` and `encrypt_type` an H200 sends are
    /// accepted, and `opaque` is required.
    #[test]
    fn test_digest_challenge_rejects_other_parameters() {
        for params in [
            "qop=\"auth\", encrypt_type=\"3\", opaque=\"O\"",
            "qop=\"auth\", algorithm=MD5, encrypt_type=\"3\", opaque=\"O\"",
            "algorithm=SHA-256, encrypt_type=\"3\", opaque=\"O\"",
            "qop=\"auth-int\", algorithm=SHA-256, encrypt_type=\"3\", opaque=\"O\"",
            "qop=\"auth\", algorithm=SHA-256, opaque=\"O\"",
            "qop=\"auth\", algorithm=SHA-256, encrypt_type=\"1\", opaque=\"O\"",
            "qop=\"auth\", algorithm=SHA-256, encrypt_type=\"3\"",
        ] {
            let head = format!(
                "HTTP/1.1 401 Unauthorized\r\n\
                 WWW-Authenticate: Digest realm=\"hub\", nonce=\"N\", {params}"
            );
            let response = HttpResponse::parse(head.as_bytes(), Vec::new()).unwrap();

            assert!(DigestChallenge::parse(&response).is_err(), "{params}");
        }
    }

    #[test]
    fn test_http_response_parse() {
        let response = HttpResponse::parse(
            b"HTTP/1.1 200 OK\r\nKey-Exchange: KEY\r\nConnection: close",
            b"body".to_vec(),
        )
        .unwrap();

        assert_eq!(response.version, "HTTP/1.1");
        assert_eq!(response.status, 200);
        assert_eq!(response.header("key-exchange"), Some("KEY"));
        assert_eq!(response.header("connection"), Some("close"));
        assert_eq!(response.body, b"body");
    }

    #[test]
    fn test_http_response_parse_rejects_invalid_status_line() {
        assert!(HttpResponse::parse(b"HTTP/1.1 OK", Vec::new()).is_err());
        assert!(HttpResponse::parse(b"", Vec::new()).is_err());
    }

    /// The `200` an H200 (firmware 1.6.5) actually sends, with the
    /// `Key-Exchange` the media cipher is derived from.
    #[test]
    fn test_media_cipher_from_h200_response() {
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

        assert!(media_cipher(&response, "HASH").is_ok());
    }

    #[test]
    fn test_media_cipher_requires_key_exchange() {
        let response =
            HttpResponse::parse(b"HTTP/1.0 200 OK\r\nConnection: close", Vec::new()).unwrap();

        assert!(media_cipher(&response, "HASH").is_err());
    }

    #[test]
    fn test_find_head_end() {
        assert_eq!(find_head_end(b"HTTP/1.1 200 OK\r\n\r\nbody"), Some(15));
        assert_eq!(find_head_end(b"HTTP/1.1 200 OK\r\n"), None);
    }

    /// The request names the camera in the URI and identifies the player
    /// there too, so, like the app's request, it carries neither `X-Preconn`
    /// nor `X-Client-UUID`.
    #[test]
    fn test_build_request() {
        let uri = stream_uri(
            "8021C49D25C63A54F99462569B4759D3238B6891",
            "6d198157-565a-4adb-aaa5-85b81bc5c918",
        );
        assert_eq!(
            uri,
            "/stream?deviceId=8021C49D25C63A54F99462569B4759D3238B6891&type=download\
             &playerId=6d198157-565a-4adb-aaa5-85b81bc5c918&media_type=0"
        );

        let request = build_request("192.168.1.100", &uri, None);
        assert_eq!(
            request,
            format!(
                "POST {uri} HTTP/1.1\r\n\
                 Host: 192.168.1.100:8800\r\n\
                 Content-Type: multipart/mixed; boundary=--client-stream-boundary--\r\n\
                 X-Key-Exchange: 1\r\n\
                 Content-Length: 0\r\n\
                 \r\n"
            )
        );
    }
}
