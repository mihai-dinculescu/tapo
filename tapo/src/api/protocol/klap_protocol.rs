use std::fmt;

use log::{debug, trace};
use rand::RngExt as _;
use reqwest::header::COOKIE;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;

use crate::api::protocol::TapoProtocol;
use crate::requests::TapoRequest;
use crate::responses::{TapoResponse, TapoResponseExt, validate_response};
use crate::{Error, TapoResponseError};

use super::crypto;
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
        let url = self.session()?.url.clone();
        self.handshake(url, username, password).await
    }

    pub async fn execute_request<R>(&self, request: TapoRequest) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        let session = self.session()?;

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

        let response_body = response.bytes().await.map_err(anyhow::Error::from)?;

        let response_decrypted = session.cipher.decrypt(seq, response_body.to_vec())?;
        trace!("Device responded with (raw): {response_decrypted}");

        let response: TapoResponse<R> = serde_json::from_str(&response_decrypted)?;
        debug!("Device responded with: {response:?}");

        validate_response(response.error_code)?;
        let result = response.result;

        Ok(result)
    }

    fn session(&self) -> Result<&KlapSession, Error> {
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
        let auth_hash = crypto::sha256(
            &[
                crypto::sha1(username.as_bytes()),
                crypto::sha1(password.as_bytes()),
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
            debug!("Handshake1 error: {}", response.status());

            if response.status() == StatusCode::FORBIDDEN {
                return Err(Error::Tapo(TapoResponseError::Unauthorized {
                    kind: "FORBIDDEN",
                    description: r"Make sure Third-Party Compatibility is turned on in the Tapo app. If it's already enabled, try switching it off and then back on again. You can find this option by navigating to Me > Third-Party Services in the app."
                        .to_string(),
                }));
            }
            return Err(Error::Tapo(TapoResponseError::HttpError {
                status_code: response.status().as_u16(),
                description: "Handshake1 failed".to_string(),
            }));
        }

        let cookie = TapoProtocol::get_cookie(response.cookies())?;

        let response_body = response.bytes().await.map_err(anyhow::Error::from)?;

        let (remote_seed, server_hash) = response_body.split_at(16);
        let local_hash = crypto::sha256(&[local_seed, remote_seed, auth_hash].concat());

        if local_hash != server_hash {
            debug!("Local hash does not match server hash");
            return Err(Error::Tapo(TapoResponseError::hash_mismatch()));
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

        let payload = crypto::sha256(&[remote_seed, local_seed, auth_hash].concat());

        let response = self
            .client
            .post(&url)
            .header(COOKIE, cookie)
            .body(payload.to_vec())
            .send()
            .await?;

        if !response.status().is_success() {
            debug!("Handshake2 error: {}", response.status());
            return Err(Error::Tapo(TapoResponseError::HttpError {
                status_code: response.status().as_u16(),
                description: "Handshake2 failed".to_string(),
            }));
        }

        debug!("Handshake2 OK");

        Ok(())
    }
}
