use std::fmt;

use async_trait::async_trait;
use log::{debug, error, trace};
use reqwest::header::COOKIE;
use reqwest::{Client, StatusCode};
use rsa::rand_core::{OsRng, RngCore as _};
use serde::de::DeserializeOwned;

use crate::api::protocol::TapoProtocol;
use crate::requests::TapoRequest;
use crate::responses::{TapoResponse, TapoResponseExt, validate_response};
use crate::{Error, TapoResponseError};

use super::TapoProtocolExt;
use super::discovery_protocol::DiscoveryProtocol;
use super::klap_cipher::KlapCipher;

#[derive(Debug)]
pub(crate) struct KlapProtocol {
    client: Client,
    cookie: String,
    rng: OsRng,
    url: Option<String>,
    cipher: Option<KlapCipher>,
}

#[async_trait]
impl TapoProtocolExt for KlapProtocol {
    async fn login(
        &mut self,
        url: String,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        self.handshake(url, username, password).await?;
        Ok(())
    }

    async fn refresh_session(&mut self, username: String, password: String) -> Result<(), Error> {
        let url = self.url.as_ref().expect("This should never happen").clone();
        self.handshake(url, username, password).await?;
        Ok(())
    }

    async fn execute_request<R>(
        &self,
        request: TapoRequest,
        _with_token: bool,
    ) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        let url = self.url.as_ref().expect("This should never happen");
        let cipher = self.get_cipher_ref();

        let request_string = serde_json::to_string(&request)?;
        debug!("Request: {request_string}");

        let (payload, seq) = cipher.encrypt(request_string)?;

        let response = self
            .client
            .post(format!("{url}/request?seq={seq}"))
            .header(COOKIE, self.cookie.clone())
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

        let response_decrypted = cipher.decrypt(seq, response_body.to_vec())?;
        trace!("Device responded with (raw): {response_decrypted}");

        let response: TapoResponse<R> = serde_json::from_str(&response_decrypted)?;
        debug!("Device responded with: {response:?}");

        validate_response(&response)?;
        let result = response.result;

        Ok(result)
    }

    fn clone_as_discovery(&self) -> DiscoveryProtocol {
        DiscoveryProtocol::new(self.client.clone())
    }
}

impl KlapProtocol {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            cookie: String::new(),
            rng: OsRng,
            url: None,
            cipher: None,
        }
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

        let local_seed = self.get_local_seed().to_vec();
        let remote_seed = self.handshake1(&url, &local_seed, &auth_hash).await?;

        self.handshake2(&url, &local_seed, &remote_seed, &auth_hash)
            .await?;

        let cipher = KlapCipher::new(local_seed, remote_seed, auth_hash)?;

        self.url.replace(url);
        self.cipher.replace(cipher);

        Ok(())
    }

    async fn handshake1(
        &mut self,
        url: &str,
        local_seed: &[u8],
        auth_hash: &[u8],
    ) -> Result<Vec<u8>, Error> {
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

        self.cookie = TapoProtocol::get_cookie(response.cookies())?;

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

        Ok(remote_seed.to_vec())
    }

    async fn handshake2(
        &self,
        url: &str,
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
            .header(COOKIE, self.cookie.clone())
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

    fn get_local_seed(&mut self) -> [u8; 16] {
        let mut buffer = [0u8; 16];
        self.rng.fill_bytes(&mut buffer);
        buffer
    }

    fn get_cipher_ref(&self) -> &KlapCipher {
        self.cipher.as_ref().expect("This should never happen")
    }
}
