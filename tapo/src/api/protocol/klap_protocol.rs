use std::fmt;

use async_trait::async_trait;
use isahc::cookies::CookieJar;
use isahc::prelude::Configurable;
use isahc::{AsyncReadResponseExt, HttpClient, Request};
use log::{debug, warn};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use serde::de::DeserializeOwned;

use crate::requests::TapoRequest;
use crate::responses::{validate_response, TapoResponse, TapoResponseExt};
use crate::{Error, TapoResponseError};

use super::discovery_protocol::DiscoveryProtocol;
use super::klap_cipher::KlapCipher;
use super::TapoProtocolExt;

#[derive(Debug)]
pub(crate) struct KlapProtocol {
    client: HttpClient,
    cookie_jar: CookieJar,
    username: String,
    password: String,
    rng: StdRng,
    url: Option<String>,
    cipher: Option<KlapCipher>,
}

#[async_trait]
impl TapoProtocolExt for KlapProtocol {
    async fn login(&mut self, url: String) -> Result<(), Error> {
        self.handshake(url).await?;
        Ok(())
    }

    async fn refresh_session(&mut self) -> Result<(), Error> {
        let url = self.url.as_ref().expect("This should never happen").clone();
        self.handshake(url).await?;
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
        debug!("Request to passthrough: {request_string}");

        let (payload, seq) = cipher.encrypt(request_string)?;

        let request = Request::post(format!("{url}/request?seq={seq}"))
            .cookie_jar(self.cookie_jar.clone())
            .body(payload)
            .map_err(isahc::Error::from)?;

        let mut response = self.client.send_async(request).await?;

        if !response.status().is_success() {
            warn!("Response error: {}", response.status());

            let error = match response.status() {
                isahc::http::StatusCode::UNAUTHORIZED | isahc::http::StatusCode::FORBIDDEN => {
                    TapoResponseError::SessionTimeout
                }
                _ => TapoResponseError::InvalidResponse,
            };

            return Err(Error::Tapo(error));
        }

        let response_body = response.bytes().await.map_err(anyhow::Error::from)?;

        let response_decrypted = cipher.decrypt(seq, response_body)?;
        debug!("Device responded with: {response_decrypted:?}");

        let inner_response: TapoResponse<R> = serde_json::from_str(&response_decrypted)?;
        debug!("Device inner response: {inner_response:?}");

        validate_response(&inner_response)?;
        let result = inner_response.result;

        Ok(result)
    }

    fn clone_as_discovery(&self) -> DiscoveryProtocol {
        DiscoveryProtocol::new(
            self.client.clone(),
            self.username.clone(),
            self.password.clone(),
        )
    }
}

impl KlapProtocol {
    pub fn new(client: HttpClient, username: String, password: String) -> Self {
        Self {
            client,
            cookie_jar: CookieJar::new(),
            username,
            password,
            rng: StdRng::from_entropy(),
            url: None,
            cipher: None,
        }
    }

    async fn handshake(&mut self, url: String) -> Result<(), Error> {
        self.cookie_jar.clear();

        let auth_hash = KlapCipher::sha256(
            &[
                KlapCipher::sha1(self.username.as_bytes()),
                KlapCipher::sha1(self.password.as_bytes()),
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
        &self,
        url: &str,
        local_seed: &[u8],
        auth_hash: &[u8],
    ) -> Result<Vec<u8>, Error> {
        debug!("Performing handshake1...");
        let url = format!("{url}/handshake1");

        let request = Request::post(&url)
            .cookie_jar(self.cookie_jar.clone())
            .body(local_seed)
            .map_err(isahc::Error::from)?;

        let mut response = self.client.send_async(request).await?;

        if !response.status().is_success() {
            warn!("Handshake1 error: {}", response.status());
            return Err(Error::Tapo(TapoResponseError::InvalidResponse));
        }

        let response_body = response.bytes().await.map_err(anyhow::Error::from)?;

        let (remote_seed, server_hash) = response_body.split_at(16);
        let local_hash = KlapCipher::sha256(&[local_seed, remote_seed, auth_hash].concat());

        if local_hash != server_hash {
            warn!("Local hash does not match server hash");
            return Err(Error::Tapo(TapoResponseError::InvalidCredentials));
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

        let request = Request::post(&url)
            .cookie_jar(self.cookie_jar.clone())
            .body(payload.to_vec())
            .map_err(isahc::Error::from)?;

        let response = self.client.send_async(request).await?;

        if !response.status().is_success() {
            warn!("Handshake2 error: {}", response.status());
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
