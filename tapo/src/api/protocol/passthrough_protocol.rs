use std::fmt;

use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use log::{debug, trace};
use rand::rngs::StdRng;
use rand::SeedableRng;
use reqwest::header::COOKIE;
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::api::protocol::TapoProtocol;
use crate::requests::{
    HandshakeParams, LoginDeviceParams, SecurePassthroughParams, TapoParams, TapoRequest,
};
use crate::responses::{
    validate_response, HandshakeResult, TapoResponse, TapoResponseExt, TapoResult, TokenResult,
};

use crate::{Error, TapoResponseError};

use super::discovery_protocol::DiscoveryProtocol;
use super::passthrough_cipher::{PassthroughCipher, PassthroughKeyPair};
use super::tapo_protocol::TapoProtocolExt;

#[derive(Debug)]
pub(crate) struct PassthroughProtocol {
    client: Client,
    key_pair: PassthroughKeyPair,
    session: Option<Session>,
}

#[derive(Debug)]
struct Session {
    pub url: String,
    pub cookie: String,
    pub cipher: PassthroughCipher,
    pub token: Option<String>,
}

#[async_trait]
impl TapoProtocolExt for PassthroughProtocol {
    async fn login(
        &mut self,
        url: String,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        self.handshake(url).await?;
        self.login_request(username, password).await?;

        Ok(())
    }

    async fn refresh_session(&mut self, username: String, password: String) -> Result<(), Error> {
        let url = self.get_session_ref().url.clone();
        self.login(url, username, password).await
    }

    async fn execute_request<R>(
        &self,
        request: TapoRequest,
        with_token: bool,
    ) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        let session = self.get_session_ref();
        let url = if with_token {
            format!(
                "{}?token={}",
                &session.url,
                session
                    .token
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("Token shouldn not be None"))?
            )
        } else {
            session.url.clone()
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

        validate_response(&response)?;

        let inner_response_encrypted = response
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .response;

        let inner_response_decrypted = session.cipher.decrypt(&inner_response_encrypted)?;

        trace!("Device inner response (raw): {inner_response_decrypted}");

        let inner_response: TapoResponse<R> = serde_json::from_str(&inner_response_decrypted)?;

        debug!("Device inner response: {inner_response:?}");

        validate_response(&inner_response)?;

        let result = inner_response.result;

        Ok(result)
    }

    fn clone_as_discovery(&self) -> DiscoveryProtocol {
        DiscoveryProtocol::new(self.client.clone())
    }
}

impl PassthroughProtocol {
    pub fn new(client: Client) -> Result<Self, Error> {
        Ok(Self {
            client,
            key_pair: PassthroughKeyPair::new(StdRng::from_entropy())?,
            session: None,
        })
    }

    async fn handshake(&mut self, url: String) -> Result<(), Error> {
        debug!("Performing handshake...");

        let params = HandshakeParams::new(self.key_pair.get_public_key()?);
        let request = TapoRequest::Handshake(TapoParams::new(params));
        let request_string = serde_json::to_string(&request)?;

        let response = self.client.post(&url).body(request_string).send().await?;
        let cookie = TapoProtocol::get_cookie(response.cookies())?;
        let response_json = response.json::<TapoResponse<HandshakeResult>>().await?;

        validate_response(&response_json)?;

        let handshake_key = response_json
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .key;

        debug!("Handshake OK");

        let cipher = PassthroughCipher::new(&handshake_key, &self.key_pair)?;

        self.session.replace(Session {
            url,
            cookie,
            cipher,
            token: None,
        });

        Ok(())
    }

    async fn login_request(&mut self, username: String, password: String) -> Result<(), Error> {
        let username_digest = PassthroughCipher::sha1_digest_username(username);
        debug!("Username digest: {username_digest}");

        let username = general_purpose::STANDARD.encode(username_digest);
        let password = general_purpose::STANDARD.encode(password);

        debug!("Will login with username '{}'...", username);

        let params = TapoParams::new(LoginDeviceParams::new(&username, &password))
            .set_request_time_mils()?;
        let request = TapoRequest::LoginDevice(params);

        let result = self
            .execute_request::<TokenResult>(request, false)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        let session = self.get_session_mut();
        session.token.replace(result.token);

        Ok(())
    }

    fn get_session_ref(&self) -> &Session {
        self.session.as_ref().expect("This should never happen")
    }

    fn get_session_mut(&mut self) -> &mut Session {
        self.session.as_mut().expect("This should never happen")
    }
}
