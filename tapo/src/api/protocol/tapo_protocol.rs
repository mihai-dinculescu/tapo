use std::fmt;

use log::debug;
use reqwest::cookie::Cookie;
use reqwest::{Client, StatusCode};
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

    pub async fn login(
        &mut self,
        ip_address: impl Into<String>,
        username: String,
        password: String,
        device_family: DeviceFamily,
        auth_protocol: AuthProtocol,
    ) -> Result<(), Error> {
        let ip_address = ip_address.into();
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
                        .login_tpap_then_aes_ssl(&ip_address, username, password)
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
                    match TpapProtocol::discover(&self.client, &ip_address).await? {
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
            Some(ActiveProtocol::Klap(_)) => format!("http://{ip_address}/app"),
            Some(ActiveProtocol::Tpap(p)) => p.url(&ip_address)?,
            None => unreachable!(),
        };
        debug!("Device url: {url}");

        match &mut self.active {
            Some(ActiveProtocol::AesSsl(p)) => {
                p.login(url, username, password).await?.into_result()
            }
            Some(ActiveProtocol::Klap(p)) => p.login(url, username, password).await,
            Some(ActiveProtocol::Tpap(p)) => p.login(url, username, password).await,
            None => unreachable!(),
        }
    }

    /// Logs in to a camera: over TPAP, and over AES SSL if the device does
    /// not take that login. A camera that takes TPAP does so whether or not
    /// it takes AES SSL, at the same address. It does not announce TPAP when
    /// asked, so it is not asked.
    async fn login_tpap_then_aes_ssl(
        &mut self,
        ip_address: &str,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        debug!("Using TPAP protocol (tried first)...");
        let mut tpap =
            TpapProtocol::new(self.client.clone(), TpapInfo::camera(), self.device_family);

        let url = tpap.url(ip_address)?;
        debug!("Device url: {url}");

        let tpap_error = match tpap.login(url, username.clone(), password.clone()).await {
            Ok(()) => {
                self.active = Some(ActiveProtocol::Tpap(Box::new(tpap)));
                return Ok(());
            }
            Err(error) if !falls_back_to_aes_ssl(&error) => return Err(error),
            Err(error) => error,
        };

        debug!("Using AES SSL protocol (TPAP login failed: {tpap_error})...");
        let mut aes_ssl = AesSslProtocol::new(self.client.clone());

        let url = format!("https://{ip_address}");
        debug!("Device url: {url}");

        match aes_ssl.login(url, username, password).await? {
            AesSslLogin::LoggedIn => {
                self.active = Some(ActiveProtocol::AesSsl(aes_ssl));
                Ok(())
            }
            // The device takes neither login, and of the two errors the one
            // of the TPAP login says why.
            AesSslLogin::Refused(_) => Err(tpap_error),
        }
    }

    pub async fn refresh_session(
        &mut self,
        username: String,
        password: String,
    ) -> Result<(), Error> {
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
