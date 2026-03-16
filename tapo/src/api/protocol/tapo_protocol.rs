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
use super::klap_protocol::KlapProtocol;

/// The authentication protocol used to communicate with a Tapo device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AuthProtocol {
    /// AES-based protocol. The client sends encrypted JSON
    /// requests over HTTP and the device returns encrypted JSON responses.
    Aes,
    /// KLAP (Key-Length-Authentication Protocol). Uses a handshake-derived
    /// symmetric cipher for request/response encryption.
    Klap,
    /// Protocol type could not be determined.
    Unknown,
}

#[derive(Debug)]
enum ActiveProtocol {
    Aes(AesProtocol),
    Klap(KlapProtocol),
}

#[derive(Debug)]
pub(crate) struct TapoProtocol {
    client: Client,
    active: Option<ActiveProtocol>,
}

impl Clone for TapoProtocol {
    fn clone(&self) -> Self {
        // Intentionally drops session — cloned clients must re-discover and re-login.
        Self {
            client: self.client.clone(),
            active: None,
        }
    }
}

impl TapoProtocol {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            active: None,
        }
    }

    pub async fn login(
        &mut self,
        url: String,
        username: String,
        password: String,
        auth_protocol: AuthProtocol,
    ) -> Result<(), Error> {
        if self.active.is_none() {
            self.active = Some(match auth_protocol {
                AuthProtocol::Aes => {
                    debug!("Using AES protocol (from discovery hint)...");
                    ActiveProtocol::Aes(AesProtocol::new(self.client.clone())?)
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
            Some(ActiveProtocol::Klap(p)) => p.refresh_session(username, password).await,
            None => Err(anyhow::anyhow!(
                "Cannot refresh session: protocol not yet initialized (login first)"
            )
            .into()),
        }
    }

    pub async fn execute_request<R>(
        &self,
        request: TapoRequest,
        with_token: bool,
    ) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        match &self.active {
            Some(ActiveProtocol::Aes(p)) => p.execute_request(request, with_token).await,
            Some(ActiveProtocol::Klap(p)) => p.execute_request(request, with_token).await,
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
            None => Err(Error::Tapo(TapoResponseError::InvalidResponse)),
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
            Err(Error::Tapo(TapoResponseError::Unknown(code))) => Ok(code != 1003),
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

        validate_response(&response)?;

        Ok(())
    }
}
