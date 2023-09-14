use std::fmt;

use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use isahc::cookies::CookieJar;
use isahc::prelude::Configurable;
use isahc::{AsyncReadResponseExt, HttpClient, Request};
use log::debug;
use serde::de::DeserializeOwned;

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
    client: HttpClient,
    username: String,
    password: String,
    key_pair: PassthroughKeyPair,
    session: Option<Session>,
}

#[derive(Debug)]
struct Session {
    pub url: String,
    pub cookie_jar: CookieJar,
    pub cipher: PassthroughCipher,
    pub token: Option<String>,
}

#[async_trait]
impl TapoProtocolExt for PassthroughProtocol {
    async fn login(&mut self, url: String) -> Result<(), Error> {
        self.handshake(url).await?;
        self.login_request().await?;

        Ok(())
    }

    async fn refresh_session(&mut self) -> Result<(), Error> {
        let url = self.get_session_ref().url.clone();
        self.login(url).await
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
                    .ok_or_else(|| anyhow::anyhow!("token shouldn't be None"))?
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

        let request = Request::post(url)
            .cookie_jar(session.cookie_jar.clone())
            .body(secure_passthrough_request_string)
            .map_err(isahc::Error::from)?;

        let response: TapoResponse<TapoResult> =
            self.client.send_async(request).await?.json().await?;

        debug!("Device responded with: {response:?}");

        validate_response(&response)?;

        let inner_response_encrypted = response
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .response;

        let inner_response_decrypted = session.cipher.decrypt(&inner_response_encrypted)?;

        debug!("Device inner response decrypted: {inner_response_decrypted}");

        let inner_response: TapoResponse<R> = serde_json::from_str(&inner_response_decrypted)?;

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

impl PassthroughProtocol {
    pub fn new(client: HttpClient, username: String, password: String) -> Result<Self, Error> {
        let username_digest = PassthroughCipher::sha1_digest_username(username);
        debug!("Username digest: {username_digest}");

        Ok(Self {
            client,
            username: general_purpose::STANDARD.encode(username_digest),
            password: general_purpose::STANDARD.encode(password),
            key_pair: PassthroughKeyPair::new()?,
            session: None,
        })
    }

    async fn handshake(&mut self, url: String) -> Result<(), Error> {
        debug!("Performing handshake...");

        let cookie_jar = CookieJar::new();

        let params = HandshakeParams::new(self.key_pair.get_public_key()?);
        let request = TapoRequest::Handshake(TapoParams::new(params));
        let request_string = serde_json::to_string(&request)?;

        let request = Request::post(&url)
            .cookie_jar(cookie_jar.clone())
            .body(request_string)
            .map_err(isahc::Error::from)?;

        let response: TapoResponse<HandshakeResult> =
            self.client.send_async(request).await?.json().await?;

        validate_response(&response)?;

        let handshake_key = response
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .key;

        debug!("Handshake OK");

        let cipher = PassthroughCipher::new(&handshake_key, &self.key_pair)?;

        self.session.replace(Session {
            url,
            cookie_jar,
            cipher,
            token: None,
        });

        Ok(())
    }

    async fn login_request(&mut self) -> Result<(), Error> {
        debug!("Will login with username '{}'...", self.username);

        let params = TapoParams::new(LoginDeviceParams::new(&self.username, &self.password))
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
