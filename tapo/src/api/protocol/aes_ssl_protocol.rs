use std::fmt;

use log::{debug, error, trace};
use reqwest::Client;
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::requests::{SecurePassthroughParams, TapoParams, TapoRequest};
use crate::responses::{TapoResponse, TapoResponseExt, validate_response};
use crate::{Error, TapoResponseError};

use super::aes_ssl_cipher::{
    AesSslCipher, compute_password_digest, generate_nonce, validate_device_confirm,
};
use super::crypto;

#[derive(Debug)]
struct AesSslSession {
    cipher: AesSslCipher,
    url_with_token: String,
    url: String,
}

#[derive(Debug)]
pub(super) struct AesSslProtocol {
    client: Client,
    session: Option<AesSslSession>,
}

impl AesSslProtocol {
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

        let request_encrypted = session.cipher.encrypt(&request_string)?;

        let secure_passthrough_request = TapoRequest::SecurePassthrough(TapoParams::new(
            SecurePassthroughParams::new(&request_encrypted),
        ));
        let secure_passthrough_request_string = serde_json::to_string(&secure_passthrough_request)?;

        let sequence = session.cipher.next_sequence();
        let tag = session
            .cipher
            .generate_tag(&secure_passthrough_request_string, sequence);

        let response = self
            .client
            .post(&session.url_with_token)
            .header("Seq", sequence.to_string())
            .header("Tapo_tag", tag)
            .body(secure_passthrough_request_string)
            .send()
            .await?;

        if !response.status().is_success() {
            error!("Response error: {}", response.status());
            return Err(Error::Tapo(TapoResponseError::HttpError {
                status_code: response.status().as_u16(),
                description: "Request failed".to_string(),
            }));
        }

        let response_body: serde_json::Value = response.json().await?;
        trace!("Device responded with (raw): {response_body}");

        let error_code = response_body
            .get("error_code")
            .and_then(|v| v.as_i64())
            .unwrap_or(-1);

        validate_response(error_code)?;

        // SmartCam responses place data under a single section key
        // (e.g. "device_info": {"basic_info": {...}}).
        // Extract the leaf object and deserialize R from it.
        let leaf = response_body
            .as_object()
            .and_then(|obj| {
                obj.iter()
                    .find(|(k, _)| *k != "error_code")
                    .and_then(|(_, section)| section.as_object())
            })
            .and_then(|section| section.values().next().and_then(|v| v.as_object()));

        let Some(leaf) = leaf else {
            return Ok(None);
        };

        let result: R = serde_json::from_value(serde_json::Value::Object(leaf.clone()))?;
        debug!("Device responded with: {result:?}");

        Ok(Some(result))
    }

    fn session(&self) -> Result<&AesSslSession, Error> {
        self.session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("AES SSL session not initialized (login first)").into())
    }

    async fn handshake(
        &mut self,
        url: String,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        let local_nonce = generate_nonce();

        let handshake1_result = self
            .handshake1(&url, &username, &password, &local_nonce)
            .await?;

        let handshake2_result = self
            .handshake2(
                &url,
                &username,
                &local_nonce,
                &handshake1_result.server_nonce,
                &handshake1_result.password_hash,
            )
            .await?;

        let cipher = AesSslCipher::new(
            local_nonce,
            &handshake1_result.server_nonce,
            handshake1_result.password_hash,
            handshake2_result.start_sequence,
        );

        self.session = Some(AesSslSession {
            cipher,
            url_with_token: format!("{url}/stok={}/ds", handshake2_result.token),
            url,
        });

        Ok(())
    }

    async fn handshake1(
        &self,
        url: &str,
        username: &str,
        password: &str,
        local_nonce: &str,
    ) -> Result<Handshake1Result, Error> {
        debug!("Performing handshake1...");

        let body = serde_json::json!({
            "method": "login",
            "params": {
                "cnonce": local_nonce,
                "encrypt_type": "3",
                "username": username,
            }
        });

        let response = self.client.post(url).json(&body).send().await?;

        if !response.status().is_success() {
            error!("Handshake1 error: {}", response.status());
            return Err(Error::Tapo(TapoResponseError::HttpError {
                status_code: response.status().as_u16(),
                description: "Handshake1 failed".to_string(),
            }));
        }

        let response_body = response.json::<TapoResponse<Handshake1Response>>().await?;
        debug!("Handshake1 response: {response_body:?}");

        // -40413 (INVALID_NONCE) is the expected response indicating the device
        // is ready for a nonce-based handshake.
        if response_body.error_code != -40413 {
            let description = format!(
                "Expected INVALID_NONCE (-40413) during handshake1, got {}",
                response_body.error_code
            );
            error!("{description}");
            return Err(Error::Tapo(TapoResponseError::Unauthorized {
                kind: "EXPECTED_INVALID_NONCE",
                description,
            }));
        }

        let data = response_body
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .data;

        // Try SHA256 password hash first, then MD5 fallback.
        let password_hash_sha256 = crypto::sha256_hex(password.as_bytes());
        if validate_device_confirm(
            local_nonce,
            &data.nonce,
            &password_hash_sha256,
            &data.device_confirm,
        ) {
            debug!("Handshake1 OK (SHA256)");
            return Ok(Handshake1Result {
                server_nonce: data.nonce,
                password_hash: password_hash_sha256,
            });
        }

        let password_hash_md5 = crypto::md5_hex(password.as_bytes());
        if validate_device_confirm(
            local_nonce,
            &data.nonce,
            &password_hash_md5,
            &data.device_confirm,
        ) {
            debug!("Handshake1 OK (MD5 fallback)");
            return Ok(Handshake1Result {
                server_nonce: data.nonce,
                password_hash: password_hash_md5,
            });
        }

        error!("Device confirm hash mismatch in handshake1");
        Err(Error::Tapo(TapoResponseError::hash_mismatch()))
    }

    async fn handshake2(
        &self,
        url: &str,
        username: &str,
        local_nonce: &str,
        server_nonce: &str,
        password_hash: &str,
    ) -> Result<Handshake2Result, Error> {
        debug!("Performing handshake2...");

        let password_digest = compute_password_digest(local_nonce, server_nonce, password_hash);

        let body = serde_json::json!({
            "method": "login",
            "params": {
                "cnonce": local_nonce,
                "encrypt_type": "3",
                "digest_passwd": password_digest,
                "username": username,
            }
        });

        let response = self.client.post(url).json(&body).send().await?;

        if !response.status().is_success() {
            error!("Handshake2 error: {}", response.status());
            return Err(Error::Tapo(TapoResponseError::HttpError {
                status_code: response.status().as_u16(),
                description: "Handshake2 failed".to_string(),
            }));
        }

        let response_body = response.json::<TapoResponse<Handshake2Result>>().await?;
        debug!("Handshake2 response: {response_body:?}");

        validate_response(response_body.error_code)?;

        let result = response_body
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        debug!("Handshake2 OK");

        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
struct Handshake1Response {
    data: Handshake1ResponseData,
}

impl TapoResponseExt for Handshake1Response {}

#[derive(Debug, Deserialize)]
struct Handshake1ResponseData {
    nonce: String,
    device_confirm: String,
}

#[derive(Debug)]
struct Handshake1Result {
    server_nonce: String,
    password_hash: String,
}

#[derive(Debug, Deserialize)]
struct Handshake2Result {
    #[serde(rename = "stok")]
    token: String,
    #[serde(rename = "start_seq")]
    start_sequence: i32,
}

impl TapoResponseExt for Handshake2Result {}
