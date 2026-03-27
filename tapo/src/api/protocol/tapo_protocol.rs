use std::fmt;

use log::debug;
use reqwest::Client;
use reqwest::cookie::Cookie;
use serde::de::DeserializeOwned;

use crate::Error;
use crate::TapoResponseError;
use crate::requests::{EmptyParams, TapoParams, TapoRequest};
use crate::responses::{TapoResponse, TapoResponseExt, validate_response};

use super::aes_protocol::AesProtocol;
use super::aes_ssl_protocol::AesSslProtocol;
use super::klap_protocol::KlapProtocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeviceFamily {
    Smart,
    SmartCam,
}

/// The authentication protocol used to communicate with a Tapo device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AuthProtocol {
    /// AES-based protocol. The client sends encrypted JSON
    /// requests over HTTP and the device returns encrypted JSON responses.
    Aes,
    /// AES-based protocol over HTTPS with nonce-based authentication.
    /// Used by IP cameras, hubs, and doorbells.
    AesSsl,
    /// KLAP (Key-Length-Authentication Protocol). Uses a handshake-derived
    /// symmetric cipher for request/response encryption.
    Klap,
    /// Protocol type could not be determined.
    Unknown,
}

#[derive(Debug)]
enum ActiveProtocol {
    Aes(AesProtocol),
    AesSsl(AesSslProtocol),
    Klap(KlapProtocol),
}

#[derive(Debug)]
pub(crate) struct TapoProtocol {
    client: Client,
    device_family: DeviceFamily,
    active: Option<ActiveProtocol>,
}

impl Clone for TapoProtocol {
    fn clone(&self) -> Self {
        // Intentionally drops session — cloned clients must re-discover and re-login.
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
        let url = match auth_protocol {
            AuthProtocol::AesSsl => format!("https://{ip_address}"),
            _ => format!("http://{ip_address}/app"),
        };
        debug!("Device url: {url}");

        if self.active.is_none() {
            self.active = Some(match auth_protocol {
                AuthProtocol::Aes => {
                    debug!("Using AES protocol (from discovery hint)...");
                    ActiveProtocol::Aes(AesProtocol::new(self.client.clone())?)
                }
                AuthProtocol::AesSsl => {
                    debug!("Using AES SSL protocol (from discovery hint)...");
                    ActiveProtocol::AesSsl(AesSslProtocol::new(self.client.clone()))
                }
                AuthProtocol::Klap => {
                    debug!("Using KLAP protocol (from discovery hint)...");
                    ActiveProtocol::Klap(KlapProtocol::new(self.client.clone()))
                }
                AuthProtocol::Unknown => self.discover_protocol_type(&url).await?,
            });
        }

        match &mut self.active {
            Some(ActiveProtocol::Aes(p)) => p.login(url, username, password).await,
            Some(ActiveProtocol::AesSsl(p)) => p.login(url, username, password).await,
            Some(ActiveProtocol::Klap(p)) => p.login(url, username, password).await,
            None => unreachable!(),
        }
    }

    pub async fn refresh_session(
        &mut self,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        match &mut self.active {
            Some(ActiveProtocol::Aes(p)) => p.refresh_session(username, password).await,
            Some(ActiveProtocol::AesSsl(p)) => p.refresh_session(username, password).await,
            Some(ActiveProtocol::Klap(p)) => p.refresh_session(username, password).await,
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
            Some(ActiveProtocol::Aes(p)) => p.execute_request(request).await,
            Some(ActiveProtocol::AesSsl(p)) => p.execute_request(request).await,
            Some(ActiveProtocol::Klap(p)) => p.execute_request(request).await,
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

    async fn discover_protocol_type(&self, url: &str) -> Result<ActiveProtocol, Error> {
        debug!("Testing the AES protocol...");
        if self.is_aes_supported(url).await? {
            debug!("Supported. Setting up the AES protocol...");
            Ok(ActiveProtocol::Aes(AesProtocol::new(self.client.clone())?))
        } else {
            debug!("Not supported. Setting up the KLAP protocol...");
            Ok(ActiveProtocol::Klap(KlapProtocol::new(self.client.clone())))
        }
    }

    async fn is_aes_supported(&self, url: &str) -> Result<bool, Error> {
        match self.test_aes(url).await {
            Err(Error::Tapo(TapoResponseError::DeviceError { code, .. })) => Ok(code != 1003),
            Err(err) => Err(err),
            Ok(_) => Ok(true),
        }
    }

    async fn test_aes(&self, url: &str) -> Result<(), Error> {
        let request = TapoRequest::ComponentNegotiation(TapoParams::new(EmptyParams));
        let request_string = serde_json::to_string(&request)?;
        debug!("Component negotiation request: {request_string}");

        let response = self
            .client
            .post(url)
            .body(request_string)
            .send()
            .await?
            .json::<TapoResponse<serde_json::Value>>()
            .await?;

        debug!("Device responded with: {response:?}");

        validate_response(response.error_code)?;

        Ok(())
    }
}
