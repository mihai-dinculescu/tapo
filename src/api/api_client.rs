use std::fmt;

use async_trait::async_trait;
use isahc::config::Configurable;
use isahc::cookies::CookieJar;
use isahc::{AsyncReadResponseExt, HttpClient, Request};
use log::debug;
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::api::{
    ColorLightHandler, ColorLightStripHandler, EnergyMonitoringPlugHandler, GenericDeviceHandler,
    HubHandler, LightHandler, PlugHandler,
};
use crate::encryption::{KeyPair, TpLinkCipher};
use crate::error::{Error, TapoResponseError};
use crate::requests::{
    ControlChildParams, EmptyParams, EnergyDataInterval, GetEnergyDataParams, HandshakeParams,
    LightingEffect, LoginDeviceParams, MultipleRequestParams, SecurePassthroughParams, TapoParams,
    TapoRequest,
};
use crate::responses::{
    validate_response, ControlChildResult, DecodableResultExt, DeviceUsageResult, EnergyDataResult,
    EnergyUsageResult, HandshakeResult, TapoMultipleResponse, TapoResponse, TapoResponseExt,
    TapoResult, TokenResult,
};

const TERMINAL_UUID: &str = "00-00-00-00-00-00";

#[async_trait]
pub(crate) trait ApiClientExt: std::fmt::Debug + Send + Sync {
    async fn set_device_info(&self, device_info_params: serde_json::Value) -> Result<(), Error>;
}

/// Tapo API Client. See [examples](https://github.com/mihai-dinculescu/tapo/tree/main/examples).
///
/// # Example
///
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
///     .l530("192.168.1.100")
///     .await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct ApiClient {
    client: HttpClient,
    username: String,
    password: String,
    key_pair: KeyPair,
    session: Option<Session>,
}

#[derive(Debug)]
pub(crate) struct Session {
    pub url: String,
    pub cookie_jar: CookieJar,
    pub tp_link_cipher: TpLinkCipher,
    pub token: Option<String>,
}

/// Tapo API Client constructor.
impl ApiClient {
    /// Returns a new instance of [`ApiClient`].
    /// It it cheaper to [`ApiClient::clone`] an existing instance than to create a new one when multiple devices need to be controller.
    /// This is because [`ApiClient::clone`] reuses the underlying [`isahc::HttpClient`] and [`openssl::rsa::Rsa`] key.
    ///
    /// # Arguments
    ///
    /// * `tapo_username` - the Tapo username
    /// * `tapo_password` - the Tapo password
    pub fn new(
        tapo_username: impl Into<String>,
        tapo_password: impl Into<String>,
    ) -> Result<ApiClient, Error> {
        let client = HttpClient::builder().title_case_headers(true).build()?;

        Ok(Self {
            client,
            username: tapo_username.into(),
            password: tapo_password.into(),
            key_pair: KeyPair::new()?,
            session: None,
        })
    }
}

/// Device handler builders.
impl ApiClient {
    /// Specializes the given [`ApiClient`] into an authenticated [`GenericDeviceHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .generic_device("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generic_device(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<GenericDeviceHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(GenericDeviceHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`LightHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .l510("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l510(mut self, ip_address: impl Into<String>) -> Result<LightHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(LightHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`ColorLightHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .l530("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l530(mut self, ip_address: impl Into<String>) -> Result<ColorLightHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(ColorLightHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`LightHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .l610("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l610(mut self, ip_address: impl Into<String>) -> Result<LightHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(LightHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`ColorLightHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .l630("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l630(mut self, ip_address: impl Into<String>) -> Result<ColorLightHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(ColorLightHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`ColorLightHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .l900("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l900(mut self, ip_address: impl Into<String>) -> Result<ColorLightHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(ColorLightHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`ColorLightStripHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .l920("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l920(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<ColorLightStripHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(ColorLightStripHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`ColorLightStripHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .l930("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l930(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<ColorLightStripHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(ColorLightStripHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`PlugHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .p100("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn p100(mut self, ip_address: impl Into<String>) -> Result<PlugHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(PlugHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`PlugHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .p105("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn p105(mut self, ip_address: impl Into<String>) -> Result<PlugHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(PlugHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`EnergyMonitoringPlugHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .p110("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn p110(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<EnergyMonitoringPlugHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(EnergyMonitoringPlugHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`EnergyMonitoringPlugHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .p115("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn p115(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<EnergyMonitoringPlugHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(EnergyMonitoringPlugHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`HubHandler`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")?
    ///     .h100("192.168.1.100")
    ///     .await?;
    ///
    /// let child_device_list = device.get_child_device_list().await?;
    /// println!("Child device list: {child_device_list:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn h100(mut self, ip_address: impl Into<String>) -> Result<HubHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(HubHandler::new(self))
    }
}

/// Tapo API Client private methods.
impl ApiClient {
    pub(crate) fn get_session_ref(&self) -> Result<&Session, Error> {
        self.session
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("The session shouldn't be None").into())
    }

    pub(crate) fn get_session_mut(&mut self) -> Result<&mut Session, Error> {
        self.session
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("The session shouldn't be None").into())
    }

    pub(crate) async fn login(&mut self, url: String) -> Result<(), Error> {
        self.handshake(url).await?;
        self.login_request().await?;

        Ok(())
    }

    pub(crate) async fn get_device_info<R>(&self) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt + DecodableResultExt,
    {
        debug!("Get Device info...");
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.execute_secure_passthrough_request::<R>(request, true)
            .await?
            .map(|result| result.decode())
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
    }

    pub(crate) async fn get_device_usage(&self) -> Result<DeviceUsageResult, Error> {
        debug!("Get Device usage...");
        let request = TapoRequest::GetDeviceUsage(TapoParams::new(EmptyParams));

        self.execute_secure_passthrough_request::<DeviceUsageResult>(request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn set_lighting_effect(
        &self,
        lighting_effect: LightingEffect,
    ) -> Result<(), Error> {
        debug!("Lighting effect will change to: {lighting_effect:?}");

        let request = TapoRequest::SetLightingEffect(Box::new(
            TapoParams::new(lighting_effect)
                .set_request_time_mils()?
                .set_terminal_uuid(TERMINAL_UUID),
        ));

        self.execute_secure_passthrough_request::<TapoResult>(request, true)
            .await?;

        Ok(())
    }

    pub(crate) async fn get_energy_usage(&self) -> Result<EnergyUsageResult, Error> {
        debug!("Get Energy usage...");
        let request = TapoRequest::GetEnergyUsage(TapoParams::new(EmptyParams));

        self.execute_secure_passthrough_request::<EnergyUsageResult>(request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn get_energy_data(
        &self,
        interval: EnergyDataInterval,
    ) -> Result<EnergyDataResult, Error> {
        debug!("Get Energy data...");
        let params = GetEnergyDataParams::new(interval);
        let request = TapoRequest::GetEnergyData(TapoParams::new(params));

        self.execute_secure_passthrough_request::<EnergyDataResult>(request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn get_child_device_list<R>(&self) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt + DecodableResultExt,
    {
        debug!("Get Child device list...");
        let request = TapoRequest::GetChildDeviceList(TapoParams::new(EmptyParams));

        self.execute_secure_passthrough_request::<R>(request, true)
            .await?
            .map(|result| result.decode())
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
    }

    pub(crate) async fn get_child_device_component_list<R>(&self) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt + DecodableResultExt,
    {
        debug!("Get Child device component list...");
        let request = TapoRequest::GetChildDeviceComponentList(TapoParams::new(EmptyParams));

        self.execute_secure_passthrough_request::<R>(request, true)
            .await?
            .map(|result| result.decode())
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
    }

    pub(crate) async fn control_child<R>(
        &self,
        device_id: String,
        child_request: TapoRequest,
    ) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        debug!("Control child...");
        let params = MultipleRequestParams::new(vec![child_request]);
        let request = TapoRequest::MultipleRequest(Box::new(TapoParams::new(params)));

        let params = ControlChildParams::new(device_id, request);
        let request = TapoRequest::ControlChild(Box::new(TapoParams::new(params)));

        let responses = self
            .execute_secure_passthrough_request::<ControlChildResult<TapoMultipleResponse<R>>>(
                request, true,
            )
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .response_data
            .result
            .responses;

        let response = responses
            .into_iter()
            .next()
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        validate_response(&response)?;

        response
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    async fn handshake(&mut self, url: String) -> Result<(), Error> {
        debug!("Performing handshake...");

        let cookie_jar = CookieJar::new();

        let params = HandshakeParams::new(self.key_pair.get_public_key()?);
        let request = TapoRequest::Handshake(TapoParams::new(params));
        debug!("Handshake request: {}", json!(request));

        let body = serde_json::to_vec(&request)?;

        let request = Request::post(&url)
            .cookie_jar(cookie_jar.clone())
            .body(body)
            .map_err(isahc::Error::from)?;

        let response: TapoResponse<HandshakeResult> =
            self.client.send_async(request).await?.json().await?;

        debug!("Device responded with: {response:?}");

        validate_response(&response)?;

        let handshake_key = response
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .key;

        let tp_link_cipher = TpLinkCipher::new(&handshake_key, &self.key_pair)?;

        self.session.replace(Session {
            url,
            cookie_jar,
            tp_link_cipher,
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
            .execute_secure_passthrough_request::<TokenResult>(request, false)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        let session = self.get_session_mut()?;
        session.token.replace(result.token);

        Ok(())
    }

    async fn execute_secure_passthrough_request<R>(
        &self,
        request: TapoRequest,
        with_token: bool,
    ) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        let request_string = serde_json::to_string(&request)?;
        debug!("Request to passthrough: {request_string}");

        let session = self.get_session_ref()?;

        let request_encrypted = session.tp_link_cipher.encrypt(&request_string)?;

        let secure_passthrough_params = SecurePassthroughParams::new(&request_encrypted);
        let secure_passthrough_request =
            TapoRequest::SecurePassthrough(TapoParams::new(secure_passthrough_params));
        debug!("Secure passthrough request: {secure_passthrough_request:?}",);

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

        let request = Request::post(url)
            .cookie_jar(session.cookie_jar.clone())
            .body(serde_json::to_vec(&secure_passthrough_request)?)
            .map_err(isahc::Error::from)?;

        let response: TapoResponse<TapoResult> =
            self.client.send_async(request).await?.json().await?;

        debug!("Device responded with: {response:?}");

        validate_response(&response)?;

        let inner_response_encrypted = response
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .response;

        let inner_response_decrypted = session.tp_link_cipher.decrypt(&inner_response_encrypted)?;

        debug!("Device inner response decrypted: {inner_response_decrypted}");

        let inner_response: TapoResponse<R> = serde_json::from_str(&inner_response_decrypted)?;

        debug!("Device inner response: {inner_response:?}");

        validate_response(&inner_response)?;

        let result = inner_response.result;

        Ok(result)
    }
}

#[async_trait]
impl ApiClientExt for ApiClient {
    async fn set_device_info(&self, device_info_params: serde_json::Value) -> Result<(), Error> {
        debug!("Device info will change to: {device_info_params:?}");

        let set_device_info_request = TapoRequest::SetDeviceInfo(Box::new(
            TapoParams::new(device_info_params)
                .set_request_time_mils()?
                .set_terminal_uuid(TERMINAL_UUID),
        ));

        self.execute_secure_passthrough_request::<TapoResult>(set_device_info_request, true)
            .await?;

        Ok(())
    }
}

impl Clone for ApiClient {
    /// Clones an instance of [`ApiClient`].
    /// This is a reasonably cheap operation because the underlying [`isahc::HttpClient`] and [`openssl::rsa::Rsa`] key are reused.
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
            key_pair: self.key_pair.clone(),
            session: None,
        }
    }
}

fn build_url(ip_address: &str) -> String {
    let url = format!("http://{}/app", ip_address);
    debug!("Device url: {url}");

    url
}
