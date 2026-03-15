use std::fmt;

use log::{debug, error, trace};
use rand::RngExt as _;
use reqwest::header::COOKIE;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;

use crate::api::protocol::TapoProtocol;
use crate::requests::TapoRequest;
use crate::responses::{TapoResponse, TapoResponseExt, validate_response};
use crate::{Error, TapoResponseError};

use super::klap_cipher::KlapCipher;

#[derive(Debug)]
struct KlapSession {
    url: String,
    cookie: String,
    cipher: KlapCipher,
}

#[derive(Debug)]
pub(super) struct KlapProtocol {
    client: Client,
    session: Option<KlapSession>,
}

impl KlapProtocol {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            session: None,
        }
    }

    pub async fn login(
        &mut self,
        url: String,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        self.handshake(url, username, password).await
    }

    pub async fn refresh_session(
        &mut self,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        let url = self.session_ref()?.url.clone();
        self.handshake(url, username, password).await
    }

    pub async fn execute_request<R>(
        &self,
        request: TapoRequest,
        _with_token: bool,
    ) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        let session = self.session_ref()?;

        let request_string = serde_json::to_string(&request)?;
        debug!("Request: {request_string}");

        let (payload, seq) = session.cipher.encrypt(request_string)?;

        let response = self
            .client
            .post(format!("{}/request?seq={seq}", session.url))
            .header(COOKIE, session.cookie.clone())
            .body(payload)
            .send()
            .await?;

        if !response.status().is_success() {
            error!("Response error: {}", response.status());

            let error = match response.status() {
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    TapoResponseError::SessionTimeout
                }
                _ => TapoResponseError::InvalidResponse,
            };

            return Err(Error::Tapo(error));
        }

        let response_body = response.bytes().await.map_err(anyhow::Error::from)?;

        let response_decrypted = session.cipher.decrypt(seq, response_body.to_vec())?;
        trace!("Device responded with (raw): {response_decrypted}");

        let response: TapoResponse<R> = serde_json::from_str(&response_decrypted)?;
        debug!("Device responded with: {response:?}");

        validate_response(&response)?;
        let result = response.result;

        Ok(result)
    }

    fn session_ref(&self) -> Result<&KlapSession, Error> {
        self.session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("KLAP session not initialized (login first)").into())
    }

    async fn handshake(
        &mut self,
        url: String,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        let auth_hash = KlapCipher::sha256(
            &[
                KlapCipher::sha1(username.as_bytes()),
                KlapCipher::sha1(password.as_bytes()),
            ]
            .concat(),
        )
        .to_vec();

        let local_seed: [u8; 16] = rand::rng().random();

        let (remote_seed, cookie) = self.handshake1(&url, &local_seed, &auth_hash).await?;

        self.handshake2(&url, &cookie, &local_seed, &remote_seed, &auth_hash)
            .await?;

        let cipher = KlapCipher::new(local_seed.to_vec(), remote_seed, auth_hash)?;

        self.session = Some(KlapSession {
            url,
            cookie,
            cipher,
        });

        Ok(())
    }

    async fn handshake1(
        &self,
        url: &str,
        local_seed: &[u8],
        auth_hash: &[u8],
    ) -> Result<(Vec<u8>, String), Error> {
        debug!("Performing handshake1...");
        let url = format!("{url}/handshake1");

        let response = self
            .client
            .post(&url)
            .body(local_seed.to_vec())
            .send()
            .await?;

        if !response.status().is_success() {
            error!("Handshake1 error: {}", response.status());

            if response.status() == StatusCode::FORBIDDEN {
                return Err(Error::Tapo(TapoResponseError::Forbidden {
                    code: "FORBIDDEN".to_string(),
                    description: r"Make sure Third-Party Compatibility is turned on in the Tapo app. If it's already enabled, try switching it off and then back on again. You can find this option by navigating to Me > Third-Party Services in the app."
                        .to_string(),
                }));
            }
            return Err(Error::Tapo(TapoResponseError::InvalidResponse));
        }

        let cookie = TapoProtocol::get_cookie(response.cookies())?;

        let response_body = response.bytes().await.map_err(anyhow::Error::from)?;

        let (remote_seed, server_hash) = response_body.split_at(16);
        let local_hash = KlapCipher::sha256(&[local_seed, remote_seed, auth_hash].concat());

        if local_hash != server_hash {
            error!("Local hash does not match server hash");
            return Err(Error::Tapo(TapoResponseError::Unauthorized {
                code: "HASH_MISMATCH".to_string(),
                description: "The device response did not match the challenge issued by the library. Make sure that your email and password are correct -— both are case-sensitive. Before adding a new device, disconnect any existing TP-Link/Tapo devices on the network. The TP-Link Simple Setup (TSS) protocol, which shares credentials from previously configured devices, may interfere with authentication. If the problem continues, perform a factory reset on the new device and add it again with no other TP-Link devices active during setup.".to_string(),
             }));
        }

        debug!("Handshake1 OK");

        Ok((remote_seed.to_vec(), cookie))
    }

    async fn handshake2(
        &self,
        url: &str,
        cookie: &str,
        local_seed: &[u8],
        remote_seed: &[u8],
        auth_hash: &[u8],
    ) -> Result<(), Error> {
        debug!("Performing handshake2...");
        let url = format!("{url}/handshake2");

        let payload = KlapCipher::sha256(&[remote_seed, local_seed, auth_hash].concat());

        let response = self
            .client
            .post(&url)
            .header(COOKIE, cookie)
            .body(payload.to_vec())
            .send()
            .await?;

        if !response.status().is_success() {
            error!("Handshake2 error: {}", response.status());
            return Err(Error::Tapo(TapoResponseError::InvalidResponse));
        }

        debug!("Handshake2 OK");

        Ok(())
    }
}
