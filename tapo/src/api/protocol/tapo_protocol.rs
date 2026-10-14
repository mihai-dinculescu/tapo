use std::fmt;

use log::debug;
use reqwest::cookie::Cookie;
use reqwest::{Client, StatusCode, Url};
use serde::de::DeserializeOwned;

use crate::Error;
use crate::TapoResponseError;
use crate::requests::TapoRequest;
use crate::responses::TapoResponseExt;

use super::aes_ssl_protocol::{AesSslLogin, AesSslProtocol};
use super::klap_protocol::KlapProtocol;
use super::tpap_protocol::{TpapInfo, TpapProtocol};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeviceFamily {
    Smart,
    SmartCam,
}

/// The authentication protocol used to communicate with a Tapo device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AuthProtocol {
    /// AES-based protocol over HTTPS with nonce-based authentication.
    /// Used by IP cameras, hubs, and doorbells. A camera hub that is reached
    /// by its IP address, and a device that announces it in discovery, is
    /// logged in to over this protocol only.
    AesSsl,
    /// KLAP (Key-Length-Authentication Protocol). Uses a handshake-derived
    /// symmetric cipher for request/response encryption.
    Klap,
    /// TPAP. Logs in with SPAKE2+ and encrypts requests and responses with
    /// AES-CCM. A device that announces it in discovery says how it wants
    /// to be logged in to as well, and is logged in to over this protocol
    /// only.
    TpapAnnounced(TpapInfo),
    /// TPAP first, and AES SSL if the device does not take that login. For
    /// a camera that is reached by its IP address, which gives no hint of
    /// the protocol it takes.
    TpapThenAesSsl,
    /// No protocol hint is available. The device is asked whether it speaks
    /// TPAP, and KLAP is used when it does not.
    Unknown,
}

enum ActiveProtocol {
    AesSsl(AesSslProtocol),
    Klap(KlapProtocol),
    Tpap(Box<TpapProtocol>),
}

pub(crate) struct TapoProtocol {
    client: Client,
    device_family: DeviceFamily,
    active: Option<ActiveProtocol>,
}

impl Clone for TapoProtocol {
    fn clone(&self) -> Self {
        // Intentionally drops session: cloned clients must re-discover and re-login.
        Self {
            client: self.client.clone(),
            device_family: self.device_family,
            active: None,
        }
    }
}

impl TapoProtocol {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            // Overwritten by login() before any caller can read it,
            // because ApiClient::protocol() guards against pre-login access.
            device_family: DeviceFamily::Smart,
            active: None,
        }
    }

    pub fn device_family(&self) -> DeviceFamily {
        self.device_family
    }

    /// Logs in to the device at `ip_address` and returns the IP address and
    /// the port that it was reached at. An `http_port` is the port that the
    /// device announces its web server on, and only a KLAP login and the
    /// TPAP discover, both over plain HTTP, take it. A TPAP login takes the
    /// port that the device announces for TPAP.
    pub async fn login(
        &mut self,
        ip_address: &str,
        http_port: Option<u16>,
        username: &str,
        password: &str,
        device_family: DeviceFamily,
        auth_protocol: AuthProtocol,
    ) -> Result<(String, u16), Error> {
        self.device_family = device_family;

        if self.active.is_none() {
            let active = match auth_protocol {
                AuthProtocol::AesSsl => {
                    debug!("Using AES SSL protocol...");
                    ActiveProtocol::AesSsl(AesSslProtocol::new(self.client.clone()))
                }
                // A camera that is reached by its IP address is logged in
                // to while finding out which protocol it takes, so there is
                // nothing left to do after that.
                AuthProtocol::TpapThenAesSsl => {
                    return self
                        .login_tpap_then_aes_ssl(ip_address, username, password)
                        .await;
                }
                AuthProtocol::Klap => {
                    debug!("Using KLAP protocol (from discovery hint)...");
                    ActiveProtocol::Klap(KlapProtocol::new(self.client.clone()))
                }
                AuthProtocol::TpapAnnounced(info) => {
                    debug!("Using TPAP protocol (from discovery hint)...");
                    ActiveProtocol::Tpap(Box::new(TpapProtocol::new(
                        self.client.clone(),
                        info,
                        device_family,
                    )))
                }
                AuthProtocol::Unknown => {
                    let address = host_with_port(ip_address, http_port, 80);
                    match TpapProtocol::discover(&self.client, &address).await? {
                        Some(info) => {
                            debug!("Using TPAP protocol (negotiated)...");
                            ActiveProtocol::Tpap(Box::new(TpapProtocol::new(
                                self.client.clone(),
                                info,
                                device_family,
                            )))
                        }
                        None => {
                            debug!("Using KLAP protocol (negotiated)...");
                            ActiveProtocol::Klap(KlapProtocol::new(self.client.clone()))
                        }
                    }
                }
            };
            self.active = Some(active);
        }

        let url = match &self.active {
            Some(ActiveProtocol::AesSsl(_)) => format!("https://{ip_address}"),
            // A TPAP discover and a KLAP login reach the device at the same
            // address, so a KLAP login that follows a discover that found no
            // TPAP must take the address that the discover took.
            Some(ActiveProtocol::Klap(_)) => {
                format!("http://{}/app", host_with_port(ip_address, http_port, 80))
            }
            Some(ActiveProtocol::Tpap(p)) => p.info().url(ip_address)?,
            None => unreachable!(),
        };
        debug!("Device url: {url}");
        let address = url_address(&url)?;

        match &mut self.active {
            Some(ActiveProtocol::AesSsl(p)) => {
                p.login(url, username, password).await?.into_result()?
            }
            Some(ActiveProtocol::Klap(p)) => p.login(url, username, password).await?,
            Some(ActiveProtocol::Tpap(p)) => p.login(url, username, password).await?,
            None => unreachable!(),
        }

        Ok(address)
    }

    /// Logs in to a camera: over TPAP, and over AES SSL if the device does
    /// not take that login. A camera that takes TPAP does so whether or not
    /// it takes AES SSL, at the same URL, so both are tried at the one that
    /// TPAP names. It does not announce TPAP when asked, so it is not asked.
    async fn login_tpap_then_aes_ssl(
        &mut self,
        ip_address: &str,
        username: &str,
        password: &str,
    ) -> Result<(String, u16), Error> {
        let info = TpapInfo::camera();
        let url = info.url(ip_address)?;
        debug!("Device url: {url}");
        let address = url_address(&url)?;

        debug!("Using TPAP protocol (tried first)...");
        let mut tpap = TpapProtocol::new(self.client.clone(), info, self.device_family);

        let tpap_error = match tpap.login(url.clone(), username, password).await {
            Ok(()) => {
                self.active = Some(ActiveProtocol::Tpap(Box::new(tpap)));
                return Ok(address);
            }
            Err(error) if !falls_back_to_aes_ssl(&error) => return Err(error),
            Err(error) => error,
        };

        debug!("Using AES SSL protocol (TPAP login failed: {tpap_error})...");
        let mut aes_ssl = AesSslProtocol::new(self.client.clone());

        match aes_ssl.login(url, username, password).await {
            Ok(AesSslLogin::LoggedIn) => {
                self.active = Some(ActiveProtocol::AesSsl(aes_ssl));
                Ok(address)
            }
            // The device takes neither login, and of the two errors the one
            // of the TPAP login says why.
            Ok(AesSslLogin::Refused(_)) => Err(tpap_error),
            // Both logins took the password as wrong, and the error of the
            // TPAP login also warns that the device locks itself.
            Err(Error::Tapo(TapoResponseError::Unauthorized {
                kind: "HASH_MISMATCH",
                ..
            })) if is_tpap_credentials(&tpap_error) => Err(tpap_error),
            Err(error) => Err(error),
        }
    }

    pub async fn refresh_session(&mut self, username: &str, password: &str) -> Result<(), Error> {
        match &mut self.active {
            Some(ActiveProtocol::AesSsl(p)) => p.refresh_session(username, password).await,
            Some(ActiveProtocol::Klap(p)) => p.refresh_session(username, password).await,
            Some(ActiveProtocol::Tpap(p)) => p.refresh_session(username, password).await,
            None => Err(anyhow::anyhow!(
                "Cannot refresh session: protocol not yet initialized (login first)"
            )
            .into()),
        }
    }

    pub async fn execute_request<R>(&self, request: TapoRequest) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        match &self.active {
            Some(ActiveProtocol::AesSsl(p)) => p.execute_request(request).await,
            Some(ActiveProtocol::Klap(p)) => p.execute_request(request).await,
            Some(ActiveProtocol::Tpap(p)) => p.execute_request(request).await,
            None => Err(anyhow::anyhow!(
                "Cannot execute request: protocol not yet initialized (login first)"
            )
            .into()),
        }
    }

    pub fn get_cookie<'a>(mut cookies: impl Iterator<Item = Cookie<'a>>) -> Result<String, Error> {
        let cookie = cookies.find(|c| c.name() == "TP_SESSIONID");

        match cookie {
            Some(cookie) => Ok(format!("{}={}", cookie.name(), cookie.value())),
            None => Err(Error::Tapo(TapoResponseError::ResponseError {
                description: "TP_SESSIONID cookie not found in response".to_string(),
            })),
        }
    }
}

/// `ip_address`, followed by `port` unless it is `default_port`, the port of
/// the scheme that the address is used with.
fn host_with_port(ip_address: &str, port: Option<u16>, default_port: u16) -> String {
    match port.filter(|port| *port != default_port) {
        Some(port) => format!("{ip_address}:{port}"),
        None => ip_address.to_string(),
    }
}

/// The host of `url` and its port, or the default port of its scheme when it
/// names none.
///
/// # Errors
///
/// Returns an error if `url` cannot be parsed or lacks a host or a port.
fn url_address(url: &str) -> Result<(String, u16), Error> {
    let parsed = Url::parse(url).map_err(|e| anyhow::anyhow!("Invalid URL {url}: {e}"))?;
    let host = parsed
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("URL {url} does not name a host"))?;
    let port = parsed
        .port_or_known_default()
        .ok_or_else(|| anyhow::anyhow!("URL {url} does not name a port"))?;

    Ok((host.to_string(), port))
}

/// Whether a camera whose TPAP login failed with `error` is tried over AES
/// SSL. A device that cannot be reached is out of reach for AES SSL too,
/// and a device that has locked itself is left alone.
fn falls_back_to_aes_ssl(error: &Error) -> bool {
    match error {
        Error::Http(error) => !(error.is_connect() || error.is_timeout()),
        Error::Tapo(TapoResponseError::Unauthorized {
            kind: "TPAP_AUTH_ATTEMPTS_LIMIT",
            ..
        }) => false,
        _ => true,
    }
}

/// Whether a TPAP login failed with `error` because the password is wrong.
fn is_tpap_credentials(error: &Error) -> bool {
    matches!(
        error,
        Error::Tapo(TapoResponseError::Unauthorized {
            kind: "TPAP_CREDENTIALS",
            ..
        })
    )
}

/// The error for a request that the device answered with `status`. A device
/// that no longer knows the session answers with 401 or 403.
pub(super) fn request_error(status: StatusCode) -> Error {
    let error = match status {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            TapoResponseError::session_expired("SESSION_TIMEOUT")
        }
        _ => TapoResponseError::HttpError {
            status_code: status.as_u16(),
            description: "Request failed".to_string(),
        },
    };

    Error::Tapo(error)
}

#[cfg(test)]
mod tests {
    use crate::responses::validate_response;

    use super::*;

    #[test]
    fn host_with_port_names_a_port_other_than_the_default() {
        assert_eq!(host_with_port("192.168.1.100", None, 80), "192.168.1.100");
        assert_eq!(
            host_with_port("192.168.1.100", Some(80), 80),
            "192.168.1.100"
        );
        assert_eq!(
            host_with_port("192.168.1.100", Some(443), 443),
            "192.168.1.100"
        );
        assert_eq!(
            host_with_port("192.168.1.100", Some(8080), 80),
            "192.168.1.100:8080"
        );
    }

    #[test]
    fn url_address_is_the_host_and_the_port_of_the_url_or_the_default_of_its_scheme() {
        let address = |ip: &str, port| (ip.to_string(), port);

        assert_eq!(
            url_address("http://192.168.1.100/app").unwrap(),
            address("192.168.1.100", 80)
        );
        assert_eq!(
            url_address("https://192.168.1.100").unwrap(),
            address("192.168.1.100", 443)
        );
        assert_eq!(
            url_address("http://192.168.1.100:8080/app").unwrap(),
            address("192.168.1.100", 8080)
        );
    }

    #[tokio::test]
    async fn unreachable_device_does_not_fall_back_to_aes_ssl() {
        let error: Error = Client::new()
            .post("https://127.0.0.1:0/")
            .send()
            .await
            .unwrap_err()
            .into();

        assert!(!falls_back_to_aes_ssl(&error));
    }

    #[test]
    fn locked_device_does_not_fall_back_to_aes_ssl() {
        let error = validate_response(-2101).unwrap_err();

        assert!(!falls_back_to_aes_ssl(&error));
    }

    #[test]
    fn refused_tpap_login_falls_back_to_aes_ssl() {
        // A wrong password is among them: the first request of the AES SSL
        // login does not send one, so it costs no failed login.
        for error_code in [-40209, -40210, -40401, -2203] {
            let error = validate_response(error_code).unwrap_err();

            assert!(falls_back_to_aes_ssl(&error), "{error_code}");
        }

        // What `pake_share` turns the -40401 of a camera into.
        let error = Error::Tapo(TapoResponseError::tpap_credentials());

        assert!(falls_back_to_aes_ssl(&error));
    }

    #[test]
    fn request_error_of_an_unknown_session_is_an_expired_session() {
        // A C220 answers a request of an ended session with 401.
        for status in [StatusCode::UNAUTHORIZED, StatusCode::FORBIDDEN] {
            assert!(matches!(
                request_error(status),
                Error::Tapo(TapoResponseError::Unauthorized {
                    kind: "SESSION_TIMEOUT",
                    ..
                })
            ));
        }
    }

    #[test]
    fn request_error_of_another_status_is_an_http_error() {
        assert!(matches!(
            request_error(StatusCode::INTERNAL_SERVER_ERROR),
            Error::Tapo(TapoResponseError::HttpError {
                status_code: 500,
                ..
            })
        ));
    }
}
