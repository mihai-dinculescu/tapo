use std::fmt;

use log::debug;
use reqwest::Client;
use reqwest::cookie::Cookie;
use serde::de::DeserializeOwned;

use crate::Error;
use crate::TapoResponseError;
use crate::requests::TapoRequest;
use crate::responses::TapoResponseExt;

use super::aes_ssl_protocol::AesSslProtocol;
use super::klap_protocol::KlapProtocol;
use super::tpap_protocol::TpapProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeviceFamily {
    Smart,
    SmartCam,
}

/// The authentication protocol used to communicate with a Tapo device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AuthProtocol {
    /// AES-based protocol over HTTPS with nonce-based authentication.
    /// Used by IP cameras, hubs, and doorbells.
    AesSsl,
    /// KLAP (Key-Length-Authentication Protocol). Uses a handshake-derived
    /// symmetric cipher for request/response encryption.
    Klap,
    /// TPAP. Logs in with SPAKE2+ and encrypts requests and responses with
    /// AES-CCM. Discovery reporting TPAP is not proof: some devices that
    /// report it still speak KLAP. The device is therefore probed first,
    /// and KLAP is used if it does not answer as a TPAP device.
    Tpap,
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
            self.active = Some(match auth_protocol {
                AuthProtocol::AesSsl => {
                    debug!("Using AES SSL protocol (from discovery hint)...");
                    ActiveProtocol::AesSsl(AesSslProtocol::new(self.client.clone()))
                }
                AuthProtocol::Klap => {
                    debug!("Using KLAP protocol (from discovery hint)...");
                    ActiveProtocol::Klap(KlapProtocol::new(self.client.clone()))
                }
                AuthProtocol::Tpap | AuthProtocol::Unknown => {
                    match TpapProtocol::discover(&self.client, &ip_address).await? {
                        Some(info) => {
                            debug!("Using TPAP protocol (negotiated)...");
                            ActiveProtocol::Tpap(Box::new(TpapProtocol::new(
                                self.client.clone(),
                                info,
                            )))
                        }
                        None => {
                            debug!("Using KLAP protocol (negotiated)...");
                            ActiveProtocol::Klap(KlapProtocol::new(self.client.clone()))
                        }
                    }
                }
            });
        }

        let url = match &self.active {
            Some(ActiveProtocol::AesSsl(_)) => format!("https://{ip_address}"),
            Some(ActiveProtocol::Klap(_)) => format!("http://{ip_address}/app"),
            Some(ActiveProtocol::Tpap(p)) => p.url(&ip_address)?,
            None => unreachable!(),
        };
        debug!("Device url: {url}");

        match &mut self.active {
            Some(ActiveProtocol::AesSsl(p)) => p.login(url, username, password).await,
            Some(ActiveProtocol::Klap(p)) => p.login(url, username, password).await,
            Some(ActiveProtocol::Tpap(p)) => p.login(url, username, password).await,
            None => unreachable!(),
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
