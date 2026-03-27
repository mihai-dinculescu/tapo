use std::fmt;

use base64::{Engine as _, engine::general_purpose};
use log::{debug, trace};
use reqwest::Client;
use reqwest::header::COOKIE;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::api::protocol::TapoProtocol;
use crate::requests::{
    HandshakeParams, LoginDeviceParams, SecurePassthroughParams, TapoParams, TapoRequest,
};
use crate::responses::{TapoResponse, TapoResponseExt, TapoResult, TokenResult, validate_response};

use crate::{Error, TapoResponseError};

use super::aes_cipher::{AesCipher, AesKeyPair};

#[derive(Debug)]
pub(super) struct AesProtocol {
    client: Client,
    key_pair: AesKeyPair,
    session: Option<Session>,
}

#[derive(Debug)]
struct Session {
    url: String,
    cookie: String,
    cipher: AesCipher,
    token: Option<String>,
}

impl AesProtocol {
    pub fn new(client: Client) -> Result<Self, Error> {
        Ok(Self {
            client,
            key_pair: AesKeyPair::new(&mut rand::rng())?,
            session: None,
        })
    }

    pub async fn login(
        &mut self,
        url: String,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        self.handshake(url).await?;
        self.login_request(username, password).await
    }

    pub async fn refresh_session(
        &mut self,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        let url = self.session()?.url.clone();
        self.login(url, username, password).await
    }

    pub async fn execute_request<R>(&self, request: TapoRequest) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        let session = self.session()?;
        let url = match &session.token {
            Some(token) => format!("{}?token={token}", &session.url),
            None => session.url.clone(),
        };

        let request_string = serde_json::to_string(&request)?;
        debug!("Request to passthrough: {request_string}");

        let request_encrypted = session.cipher.encrypt(&request_string)?;

        let secure_passthrough_params = SecurePassthroughParams::new(&request_encrypted);
        let secure_passthrough_request =
            TapoRequest::SecurePassthrough(TapoParams::new(secure_passthrough_params));
        let secure_passthrough_request_string = serde_json::to_string(&secure_passthrough_request)?;

        let request = self
            .client
            .post(url)
            .header(COOKIE, session.cookie.clone())
            .body(secure_passthrough_request_string);

        let response = request
            .send()
            .await?
            .json::<TapoResponse<TapoResult>>()
            .await?;

        debug!("Device responded with: {response:?}");

        validate_response(response.error_code)?;

        let inner_response_encrypted = response
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .response;

        let inner_response_decrypted = session.cipher.decrypt(&inner_response_encrypted)?;

        trace!("Device inner response (raw): {inner_response_decrypted}");

        let inner_response: TapoResponse<R> = serde_json::from_str(&inner_response_decrypted)?;

        debug!("Device inner response: {inner_response:?}");

        validate_response(inner_response.error_code)?;

        let result = inner_response.result;

        Ok(result)
    }

    fn session(&self) -> Result<&Session, Error> {
        self.session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Session not initialized (login first)").into())
    }

    fn session_mut(&mut self) -> Result<&mut Session, Error> {
        self.session
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Session not initialized (login first)").into())
    }

    async fn handshake(&mut self, url: String) -> Result<(), Error> {
        debug!("Performing handshake...");

        let params = HandshakeParams::new(self.key_pair.get_public_key()?);
        let request = TapoRequest::Handshake(TapoParams::new(params));
        let request_string = serde_json::to_string(&request)?;

        let response = self.client.post(&url).body(request_string).send().await?;
        let cookie = TapoProtocol::get_cookie(response.cookies())?;
        let response_json = response.json::<TapoResponse<HandshakeResult>>().await?;

        validate_response(response_json.error_code)?;

        let handshake_key = response_json
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .key;

        debug!("Handshake OK");

        let cipher = AesCipher::new(&handshake_key, &self.key_pair)?;

        self.session.replace(Session {
            url,
            cookie,
            cipher,
            token: None,
        });

        Ok(())
    }

    async fn login_request(&mut self, username: String, password: String) -> Result<(), Error> {
        let username_digest = AesCipher::sha1_digest_username(username);
        debug!("Username digest: {username_digest}");

        let username = general_purpose::STANDARD.encode(username_digest);
        let password = general_purpose::STANDARD.encode(password);

        debug!("Will login with username '{username}'...");

        let params = TapoParams::new(LoginDeviceParams::new(&username, &password))
            .set_request_time_mils()?;
        let request = TapoRequest::LoginDevice(params);

        let result = self
            .execute_request::<TokenResult>(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        let session = self.session_mut()?;
        session.token.replace(result.token);

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HandshakeResult {
    key: String,
}

impl TapoResponseExt for HandshakeResult {}
