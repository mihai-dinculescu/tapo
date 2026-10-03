use std::fmt;

use base64::{Engine as _, engine::general_purpose};
use log::{debug, trace};
use rand::RngExt as _;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::requests::TapoRequest;
use crate::responses::{TapoResponse, TapoResponseExt, validate_response};
use crate::{Error, TapoResponseError};

use super::crypto;
use super::tpap_cipher::TpapCipher;
use super::tpap_spake2p::{self, Proof};

/// The only cipher suite the Tapo app offers: P-256, SHA-256 and HMAC-SHA256.
const CIPHER_SUITE: i64 = 1;
const ENCRYPTION: &str = "aes_128_ccm";
/// The upper bound keeps a rogue device from making the client spin.
const MAX_ITERATIONS: u32 = 100_000;

/// How a device in TPAP mode wants to be logged in to.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct TpapInfo {
    /// 0 for plain HTTP, anything else for TLS.
    #[serde(default)]
    tls: i64,
    port: Option<i64>,
    /// The passcode types the device takes: 0 for the default passcode and
    /// 2 for the account password, among others.
    #[serde(default)]
    pake: Vec<i64>,
}

impl TpapInfo {
    fn url(&self, ip_address: &str) -> Result<String, Error> {
        if self.tls != 0 {
            return Err(Error::unsupported_tpap(format!("tls: {}", self.tls)));
        }

        // A caller that names a port reaches the device through it, as with
        // a forwarded port, and the device does not know about that one.
        if has_port(ip_address) {
            return Ok(format!("http://{ip_address}"));
        }

        let port = self.port.filter(|port| *port > 0).unwrap_or(80);

        Ok(format!("http://{ip_address}:{port}"))
    }

    fn passcode_type(&self) -> Result<&'static str, Error> {
        // The Tapo app logs in with the default passcode whenever the device
        // takes it, even if it takes the account password as well, and the
        // default passcode is not supported yet.
        if self.pake.contains(&2) && !self.pake.contains(&0) {
            Ok("userpw")
        } else {
            Err(Error::unsupported_tpap(format!("pake: {:?}", self.pake)))
        }
    }
}

struct TpapSession {
    url_with_token: String,
    cipher: TpapCipher,
}

pub(super) struct TpapProtocol {
    client: Client,
    info: TpapInfo,
    url: Option<String>,
    session: Option<TpapSession>,
}

impl TpapProtocol {
    pub fn new(client: Client, info: TpapInfo) -> Self {
        Self {
            client,
            info,
            url: None,
            session: None,
        }
    }

    /// Asks the device whether it is in TPAP mode. A device that does not
    /// answer with how to log in to it over TPAP is not.
    ///
    /// # Errors
    ///
    /// Returns an error if the device cannot be reached, if it is locked
    /// after too many failed logins, or if it announces TPAP in a way that
    /// cannot be read.
    pub async fn discover(client: &Client, ip_address: &str) -> Result<Option<TpapInfo>, Error> {
        debug!("Performing discover...");

        let body = json!({
            "method": "login",
            "params": {
                "sub_method": "discover",
            }
        });

        let response = match client
            .post(format!("http://{ip_address}/"))
            .json(&body)
            .send()
            .await
        {
            Ok(response) => response,
            // KLAP uses the same address and port, so it cannot reach the
            // device either and would only wait for the same time again.
            Err(error) if error.is_connect() || error.is_timeout() => {
                debug!("Discover error: {error}");
                return Err(error.into());
            }
            Err(error) => {
                debug!("Discover error: {error}");
                return Ok(None);
            }
        };

        if !response.status().is_success() {
            debug!("Discover error: {}", response.status());
            return Ok(None);
        }

        let Ok(response_body) = response.json::<serde_json::Value>().await else {
            debug!("Discover response is not JSON");
            return Ok(None);
        };
        debug!("Device responded with: {response_body}");

        parse_discover_response(&response_body)
    }

    /// The URL that the device takes TPAP requests on.
    ///
    /// # Errors
    ///
    /// Returns an error if the device takes them over TLS, which is not supported.
    pub fn url(&self, ip_address: &str) -> Result<String, Error> {
        self.info.url(ip_address)
    }

    pub async fn login(
        &mut self,
        url: String,
        _username: String,
        password: String,
    ) -> Result<(), Error> {
        self.url = Some(url);
        self.handshake(password).await
    }

    pub async fn refresh_session(
        &mut self,
        _username: String,
        password: String,
    ) -> Result<(), Error> {
        self.handshake(password).await
    }

    pub async fn execute_request<R>(&self, request: TapoRequest) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        let session = self.session()?;

        let request_string = serde_json::to_string(&request)?;
        debug!("Request: {request_string}");

        let (payload, seq) = session.cipher.encrypt(&request_string)?;

        let response = self
            .client
            .post(&session.url_with_token)
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(payload)
            .send()
            .await?;

        if !response.status().is_success() {
            debug!("Response error: {}", response.status());

            let error = match response.status() {
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    TapoResponseError::session_expired("SESSION_TIMEOUT")
                }
                _ => TapoResponseError::HttpError {
                    status_code: response.status().as_u16(),
                    description: "Request failed".to_string(),
                },
            };

            return Err(Error::Tapo(error));
        }

        let response_body = response.bytes().await?;

        // Decrypt first and look for JSON second: a ciphertext starts with
        // `{` whenever the first byte of its sequence number is 0x7B.
        let response_decrypted = match session.cipher.decrypt(seq, &response_body) {
            Ok(response_decrypted) => response_decrypted,
            Err(error) => return Err(undecryptable_response_error(&response_body, &error)),
        };
        trace!("Device responded with (raw): {response_decrypted}");

        let response: TapoResponse<R> = serde_json::from_str(&response_decrypted)?;
        debug!("Device responded with: {response:?}");

        validate_response(response.error_code)?;
        let result = response.result;

        Ok(result)
    }

    fn session(&self) -> Result<&TpapSession, Error> {
        self.session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("TPAP session not initialized (login first)").into())
    }

    // Of the request and response bodies of the handshake only the error
    // code is logged, and nothing that is derived from them.
    async fn handshake(&mut self, password: String) -> Result<(), Error> {
        let url = self
            .url
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("TPAP session not initialized (login first)"))?;
        let passcode_type = self.info.passcode_type()?;
        let user_random: [u8; 32] = rand::rng().random();

        let registration = self.pake_register(url, passcode_type, &user_random).await?;

        let credential = tpap_spake2p::credential(&password, registration.extra_crypt.as_ref())?;
        // The key derivation takes long enough to stall the other tasks of
        // the runtime, so it runs on the blocking pool.
        let proof = tokio::task::spawn_blocking(move || {
            tpap_spake2p::prove(
                &credential,
                &registration.salt,
                registration.iterations,
                &user_random,
                &registration.device_random,
                &registration.device_share,
            )
        })
        .await
        .map_err(anyhow::Error::from)??;

        let share_result = self.pake_share(url, &proof).await?;

        let session = TpapSession {
            url_with_token: format!("{url}/stok={}/ds", share_result.token),
            cipher: TpapCipher::new(&proof.shared_key, share_result.start_sequence)?,
        };
        self.session = Some(session);

        Ok(())
    }

    async fn pake_register(
        &self,
        url: &str,
        passcode_type: &str,
        user_random: &[u8],
    ) -> Result<Registration, Error> {
        let body = json!({
            "method": "login",
            "params": {
                "sub_method": "pake_register",
                // Always the `admin` user, never the account email.
                "username": crypto::md5_hex_lower(b"admin"),
                "user_random": general_purpose::STANDARD.encode(user_random),
                "cipher_suites": [CIPHER_SUITE],
                "encryption": [ENCRYPTION],
                "passcode_type": passcode_type,
            }
        });

        let result: RegisterResult = self.login_request(url, "pake_register", &body).await?;
        let registration = result.validate()?;

        debug!("pake_register OK");

        Ok(registration)
    }

    async fn pake_share(&self, url: &str, proof: &Proof) -> Result<ShareResult, Error> {
        let body = json!({
            "method": "login",
            "params": {
                "sub_method": "pake_share",
                "user_share": general_purpose::STANDARD.encode(&proof.share),
                "user_confirm": general_purpose::STANDARD.encode(proof.confirm),
            }
        });

        let result: ShareResult = self.login_request(url, "pake_share", &body).await?;

        let device_confirm = decode_base64("dev_confirm", &result.device_confirm)?;
        if !proof.verify(&device_confirm) {
            debug!("Device confirm does not match in pake_share");
            return Err(Error::Tapo(TapoResponseError::tpap_hash_mismatch()));
        }

        debug!("pake_share OK");

        Ok(result)
    }

    async fn login_request<T: DeserializeOwned>(
        &self,
        url: &str,
        sub_method: &str,
        body: &serde_json::Value,
    ) -> Result<T, Error> {
        debug!("Performing {sub_method}...");

        let response = self
            .client
            .post(format!("{url}/"))
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            debug!("{sub_method} error: {}", response.status());
            return Err(Error::Tapo(TapoResponseError::HttpError {
                status_code: response.status().as_u16(),
                description: format!("{sub_method} failed"),
            }));
        }

        let response_body = response.json::<LoginResponse>().await?;
        debug!("{sub_method} error code: {}", response_body.error_code);

        validate_response(response_body.error_code)?;

        let result = response_body
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        Ok(serde_json::from_value(result)?)
    }
}

fn parse_discover_response(response_body: &serde_json::Value) -> Result<Option<TpapInfo>, Error> {
    let Some(error_code) = response_body.get("error_code").and_then(|v| v.as_i64()) else {
        return Ok(None);
    };

    match validate_response(error_code) {
        Ok(()) => {}
        // A locked device answers every request with this error, and KLAP
        // would only hide it behind a 403.
        Err(
            error @ Error::Tapo(TapoResponseError::Unauthorized {
                kind: "TPAP_AUTH_ATTEMPTS_LIMIT",
                ..
            }),
        ) => return Err(error),
        Err(_) => return Ok(None),
    }

    let Some(tpap) = response_body
        .pointer("/result/tpap")
        .filter(|tpap| tpap.is_object())
    else {
        return Ok(None);
    };

    // A device that announces a `tpap` object is in TPAP mode, and KLAP
    // would only hide that behind a 403.
    let info = TpapInfo::deserialize(tpap)
        .map_err(|_| Error::unsupported_tpap(format!("tpap: {tpap}")))?;

    Ok(Some(info))
}

/// Whether `address` names a port after its host, as in `192.168.1.100:8080`
/// or `[::1]:8080`.
fn has_port(address: &str) -> bool {
    address.rsplit_once(':').is_some_and(|(host, port)| {
        !port.is_empty()
            && port.bytes().all(|b| b.is_ascii_digit())
            && (!host.contains(':') || host.ends_with(']'))
    })
}

/// The error for a response that does not decrypt. A device that no longer
/// knows the session answers with an `error_code` in plain JSON, whichever
/// code that is.
fn undecryptable_response_error(response_body: &[u8], error: &anyhow::Error) -> Error {
    let error_code = serde_json::from_slice::<serde_json::Value>(response_body)
        .ok()
        .and_then(|body| body.get("error_code")?.as_i64())
        .filter(|error_code| *error_code != 0);

    match error_code {
        Some(error_code) => {
            debug!("Device responded with error code {error_code} in plain text");
            Error::Tapo(TapoResponseError::session_expired("TPAP_SESSION_INVALID"))
        }
        None => Error::Tapo(TapoResponseError::ResponseError {
            description: format!("The response could not be decrypted: {error}"),
        }),
    }
}

fn decode_base64(field: &str, value: &str) -> Result<Vec<u8>, Error> {
    general_purpose::STANDARD.decode(value).map_err(|_| {
        Error::Tapo(TapoResponseError::ResponseError {
            description: format!("The device's `{field}` is not valid base64"),
        })
    })
}

#[derive(Deserialize)]
struct LoginResponse {
    error_code: i64,
    result: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct RegisterResult {
    dev_random: String,
    dev_salt: String,
    dev_share: String,
    cipher_suites: i64,
    iterations: i64,
    encryption: String,
    extra_crypt: Option<serde_json::Value>,
}

impl RegisterResult {
    fn validate(self) -> Result<Registration, Error> {
        if self.cipher_suites != CIPHER_SUITE {
            return Err(Error::Tapo(TapoResponseError::ResponseError {
                description: format!(
                    "The device picked cipher suite {}, which is not supported",
                    self.cipher_suites
                ),
            }));
        }

        if self.encryption != ENCRYPTION {
            return Err(Error::Tapo(TapoResponseError::ResponseError {
                description: format!(
                    "The device picked the `{}` encryption, which is not supported",
                    self.encryption
                ),
            }));
        }

        let iterations = u32::try_from(self.iterations)
            .ok()
            .filter(|iterations| (1..=MAX_ITERATIONS).contains(iterations))
            .ok_or_else(|| {
                Error::Tapo(TapoResponseError::ResponseError {
                    description: format!(
                        "The device asks for {} PBKDF2 iterations, which is outside of 1 to {MAX_ITERATIONS}",
                        self.iterations
                    ),
                })
            })?;

        let device_random = decode_base64("dev_random", &self.dev_random)?;
        if device_random.len() != 32 {
            return Err(Error::Tapo(TapoResponseError::ResponseError {
                description: format!(
                    "The device's `dev_random` of {} bytes is not 32 bytes long",
                    device_random.len()
                ),
            }));
        }

        Ok(Registration {
            device_random,
            salt: decode_base64("dev_salt", &self.dev_salt)?,
            device_share: decode_base64("dev_share", &self.dev_share)?,
            iterations,
            extra_crypt: self.extra_crypt,
        })
    }
}

struct Registration {
    device_random: Vec<u8>,
    salt: Vec<u8>,
    device_share: Vec<u8>,
    iterations: u32,
    extra_crypt: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct ShareResult {
    #[serde(rename = "dev_confirm")]
    device_confirm: String,
    #[serde(rename = "stok")]
    token: String,
    #[serde(rename = "start_seq")]
    start_sequence: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info(tpap: serde_json::Value) -> TpapInfo {
        serde_json::from_value(tpap).unwrap()
    }

    fn register_result() -> RegisterResult {
        RegisterResult {
            dev_random: general_purpose::STANDARD.encode([1; 32]),
            dev_salt: general_purpose::STANDARD.encode([2; 16]),
            dev_share: general_purpose::STANDARD.encode([3; 65]),
            cipher_suites: 1,
            iterations: 3000,
            encryption: "aes_128_ccm".to_string(),
            extra_crypt: None,
        }
    }

    fn response_error(result: Result<Registration, Error>) -> String {
        match result.err() {
            Some(Error::Tapo(TapoResponseError::ResponseError { description })) => description,
            other => panic!("expected a response error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn discover_of_an_unreachable_device_is_an_error() {
        let result = TpapProtocol::discover(&Client::new(), "127.0.0.1:0").await;

        assert!(matches!(result, Err(Error::Http(_))));
    }

    #[test]
    fn discover_response_in_tpap_mode_has_tpap_info() {
        let response_body = json!({
            "result": {
                "sub_method": "discover",
                "tpap_preferred": true,
                "mac": "AABBCCDDEEFF",
                "tpap": { "tls": 0, "dac": 0, "noc": 0, "pake": [2], "port": 80 },
            },
            "error_code": 0,
        });

        let info = parse_discover_response(&response_body).unwrap().unwrap();

        assert_eq!(
            info.url("192.168.1.100").unwrap(),
            "http://192.168.1.100:80"
        );
        assert_eq!(info.passcode_type().unwrap(), "userpw");
    }

    #[test]
    fn discover_response_in_klap_mode_has_no_tpap_info() {
        let response_body = json!({
            "result": { "sub_method": "discover", "tpap_preferred": false },
            "error_code": 0,
        });

        assert!(parse_discover_response(&response_body).unwrap().is_none());
    }

    #[test]
    fn discover_response_with_an_error_code_has_no_tpap_info() {
        // What an L530 on firmware that predates TPAP answers.
        let response_body = json!({ "error_code": 1003 });

        assert!(parse_discover_response(&response_body).unwrap().is_none());
    }

    #[test]
    fn discover_response_of_a_locked_device_is_an_error() {
        let response_body = json!({ "error_code": -2101 });

        assert!(matches!(
            parse_discover_response(&response_body).err(),
            Some(Error::Tapo(TapoResponseError::Unauthorized {
                kind: "TPAP_AUTH_ATTEMPTS_LIMIT",
                ..
            }))
        ));
    }

    #[test]
    fn discover_response_with_an_unreadable_tpap_object_is_unsupported() {
        let response_body = json!({
            "result": { "sub_method": "discover", "tpap": { "tls": 0, "pake": "2" } },
            "error_code": 0,
        });

        assert!(matches!(
            parse_discover_response(&response_body).err(),
            Some(Error::UnsupportedProtocol { protocol, description })
                if protocol == "TPAP" && description.contains(r#""pake":"2""#)
        ));
    }

    #[test]
    fn url_defaults_to_port_80() {
        assert_eq!(
            info(json!({ "tls": 0, "pake": [2] }))
                .url("192.168.1.100")
                .unwrap(),
            "http://192.168.1.100:80"
        );
        assert_eq!(
            info(json!({ "tls": 0, "pake": [2], "port": 8080 }))
                .url("192.168.1.100")
                .unwrap(),
            "http://192.168.1.100:8080"
        );
    }

    #[test]
    fn url_keeps_the_port_of_the_address() {
        let info = info(json!({ "tls": 0, "pake": [2], "port": 80 }));

        assert_eq!(
            info.url("192.168.1.100:8080").unwrap(),
            "http://192.168.1.100:8080"
        );
        assert_eq!(info.url("plug.local:80").unwrap(), "http://plug.local:80");
        assert_eq!(info.url("[::1]:8080").unwrap(), "http://[::1]:8080");
        assert_eq!(info.url("[::1]").unwrap(), "http://[::1]:80");
    }

    #[test]
    fn url_over_tls_is_unsupported() {
        let error = info(json!({ "tls": 2, "pake": [2], "port": 4433 }))
            .url("192.168.1.100")
            .unwrap_err();

        assert!(matches!(
            &error,
            Error::UnsupportedProtocol { protocol, description }
                if *protocol == "TPAP" && description.contains("tls: 2")
        ));
    }

    #[test]
    fn passcode_type_other_than_the_account_password_is_unsupported() {
        for pake in [json!([0]), json!([0, 2]), json!([1]), json!([])] {
            let error = info(json!({ "tls": 0, "pake": pake }))
                .passcode_type()
                .unwrap_err();

            assert!(matches!(
                &error,
                Error::UnsupportedProtocol { protocol, description }
                    if *protocol == "TPAP" && description.contains("pake: [")
            ));
        }
    }

    #[test]
    fn register_result_is_validated() {
        let registration = register_result().validate().unwrap();

        assert_eq!(registration.device_random, [1; 32]);
        assert_eq!(registration.salt, [2; 16]);
        assert_eq!(registration.device_share, [3; 65]);
        assert_eq!(registration.iterations, 3000);
    }

    #[test]
    fn register_result_names_the_value_it_rejects() {
        let result = RegisterResult {
            cipher_suites: 2,
            ..register_result()
        };
        assert!(response_error(result.validate()).contains("cipher suite 2"));

        let result = RegisterResult {
            encryption: "aes_256_gcm".to_string(),
            ..register_result()
        };
        assert!(response_error(result.validate()).contains("aes_256_gcm"));

        for iterations in [0, -1, 100_001] {
            let result = RegisterResult {
                iterations,
                ..register_result()
            };
            assert!(response_error(result.validate()).contains(&format!("{iterations} PBKDF2")));
        }

        let result = RegisterResult {
            dev_random: general_purpose::STANDARD.encode([1; 16]),
            ..register_result()
        };
        assert!(response_error(result.validate()).contains("16 bytes"));

        let result = RegisterResult {
            dev_share: "not base64".to_string(),
            ..register_result()
        };
        assert!(response_error(result.validate()).contains("dev_share"));
    }

    #[test]
    fn undecryptable_response_with_an_error_code_is_an_expired_session() {
        let error = anyhow::anyhow!("Decryption error");

        // On this path -2203 does not mean a wrong password either.
        for response_body in [r#"{"error_code":-2402}"#, r#"{"error_code": -2203}"#] {
            assert!(matches!(
                undecryptable_response_error(response_body.as_bytes(), &error),
                Error::Tapo(TapoResponseError::Unauthorized {
                    kind: "TPAP_SESSION_INVALID",
                    ..
                })
            ));
        }
    }

    #[test]
    fn undecryptable_response_without_an_error_code_is_a_response_error() {
        let error = anyhow::anyhow!("Decryption error");

        for response_body in [b"".as_slice(), b"{\x00\x01\x02", br#"{"result":{}}"#] {
            assert!(matches!(
                undecryptable_response_error(response_body, &error),
                Error::Tapo(TapoResponseError::ResponseError { .. })
            ));
        }
    }
}
