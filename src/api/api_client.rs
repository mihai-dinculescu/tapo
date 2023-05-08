use std::fmt;

use async_trait::async_trait;
use isahc::{config::Configurable, cookies::CookieJar, AsyncReadResponseExt, HttpClient};
use log::debug;
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::api::{
    ColorLightHandler, ColorLightStripHandler, EnergyMonitoringPlugHandler, GenericDeviceHandler,
    LightHandler, PlugHandler,
};
use crate::encryption::{KeyPair, TpLinkCipher};
use crate::error::{Error, TapoResponseError};
use crate::requests::{
    EnergyDataInterval, GetDeviceInfoParams, GetDeviceUsageParams, GetEnergyDataParams,
    GetEnergyUsageParams, HandshakeParams, LightingEffect, LoginDeviceParams,
    SecurePassthroughParams, TapoParams, TapoRequest,
};
use crate::responses::{
    validate_response, DeviceInfoResultExt, DeviceUsageResult, EnergyDataResult, EnergyUsageResult,
    HandshakeResult, TapoResponse, TapoResponseExt, TapoResult, TokenResult,
};

const TERMINAL_UUID: &str = "00-00-00-00-00-00";

/// Unauthenticated handler. Call `login` to authenticate.
pub struct Unauthenticated;
/// Authenticated handler. The session can be refreshed by calling `login` again.
pub struct Authenticated;

#[async_trait]
pub(crate) trait ApiClientExt: std::fmt::Debug {
    async fn set_device_info(&self, device_info_params: serde_json::Value) -> Result<(), Error>;
}

/// Tapo API Client. See [examples](https://github.com/mihai-dinculescu/tapo/tree/main/examples).
///
/// # Examples
/// ## GenericDevice
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new(
///         "192.168.1.100",
///         "tapo-username@example.com",
///         "tapo-password",
///     )?
///     .generic_device()
///     .login()
///     .await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## L510
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new(
///         "192.168.1.100",
///         "tapo-username@example.com",
///         "tapo-password",
///     )?
///     .l510()
///     .login()
///     .await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## L530
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new(
///         "192.168.1.100",
///         "tapo-username@example.com",
///         "tapo-password",
///     )?
///     .l530()
///     .login()
///     .await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## L610
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new(
///         "192.168.1.100",
///         "tapo-username@example.com",
///         "tapo-password",
///     )?
///     .l610()
///     .login()
///     .await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## L630
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new(
///         "192.168.1.100",
///         "tapo-username@example.com",
///         "tapo-password",
///     )?
///     .l630()
///     .login()
///     .await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## L900
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new(
///         "192.168.1.100",
///         "tapo-username@example.com",
///         "tapo-password",
///     )?
///     .l900()
///     .login()
///     .await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## L920
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new(
///         "192.168.1.100",
///         "tapo-username@example.com",
///         "tapo-password",
///     )?
///     .l920()
///     .login()
///     .await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## L930
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new(
///         "192.168.1.100",
///         "tapo-username@example.com",
///         "tapo-password",
///     )?
///     .l930()
///     .login()
///     .await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## P100
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new(
///         "192.168.1.100",
///         "tapo-username@example.com",
///         "tapo-password",
///     )?
///     .p100()
///     .login()
///     .await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## P105
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new(
///         "192.168.1.100",
///         "tapo-username@example.com",
///         "tapo-password",
///     )?
///     .p105()
///     .login()
///     .await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## P110
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new(
///         "192.168.1.100",
///         "tapo-username@example.com",
///         "tapo-password",
///     )?
///     .p110()
///     .login()
///     .await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## P115
/// ```rust,no_run
/// use tapo::ApiClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::new(
///         "192.168.1.100",
///         "tapo-username@example.com",
///         "tapo-password",
///     )?
///     .p115()
///     .login()
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
    url: String,
    username: String,
    password: String,
    cookie_jar: CookieJar,
    tp_link_cipher: Option<TpLinkCipher>,
    token: Option<String>,
}

impl ApiClient {
    /// Returns a new instance of [`crate::ApiClient`].
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device
    /// * `tapo_username` - the Tapo username
    /// * `tapo_password` - the Tapo password
    ///
    /// # Example
    /// ```rust,no_run
    /// use tapo::ApiClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ApiClient::new(
    ///         "192.168.1.100",
    ///         "tapo-username@example.com",
    ///         "tapo-password",
    ///     )?;
    ///
    ///     let device = client.l530().login().await?;
    ///
    ///     device.on().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(
        ip_address: impl Into<String>,
        tapo_username: impl Into<String>,
        tapo_password: impl Into<String>,
    ) -> Result<ApiClient, Error> {
        let url = format!("http://{}/app", ip_address.into());
        debug!("Device url: {url}");

        let cookie_jar = CookieJar::new();
        let client = HttpClient::builder()
            .title_case_headers(true)
            .cookie_jar(cookie_jar.clone())
            .build()?;

        Ok(Self {
            client,
            url,
            username: tapo_username.into(),
            password: tapo_password.into(),
            cookie_jar,
            tp_link_cipher: None,
            token: None,
        })
    }

    /// Specializes the given [`crate::ApiClient`] into a [`crate::GenericDeviceHandler`].
    pub fn generic_device(self) -> GenericDeviceHandler<Unauthenticated> {
        GenericDeviceHandler::new(self)
    }

    /// Specializes the given [`crate::ApiClient`] into a [`crate::LightHandler`].
    pub fn l510(self) -> LightHandler<Unauthenticated> {
        LightHandler::new(self)
    }

    /// Specializes the given [`crate::ApiClient`] into a [`crate::ColorLightHandler`].
    pub fn l530(self) -> ColorLightHandler<Unauthenticated> {
        ColorLightHandler::new(self)
    }

    /// Specializes the given [`crate::ApiClient`] into a [`crate::LightHandler`].
    pub fn l610(self) -> LightHandler<Unauthenticated> {
        LightHandler::new(self)
    }

    /// Specializes the given [`crate::ApiClient`] into a [`crate::ColorLightHandler`].
    pub fn l630(self) -> ColorLightHandler<Unauthenticated> {
        ColorLightHandler::new(self)
    }

    /// Specializes the given [`crate::ApiClient`] into a [`crate::ColorLightHandler`].
    pub fn l900(self) -> ColorLightHandler<Unauthenticated> {
        ColorLightHandler::new(self)
    }

    /// Specializes the given [`crate::ApiClient`] into a [`crate::ColorLightStripHandler`].
    pub fn l920(self) -> ColorLightStripHandler<Unauthenticated> {
        ColorLightStripHandler::new(self)
    }

    /// Specializes the given [`crate::ApiClient`] into a [`crate::ColorLightStripHandler`].
    pub fn l930(self) -> ColorLightStripHandler<Unauthenticated> {
        ColorLightStripHandler::new(self)
    }

    /// Specializes the given [`crate::ApiClient`] into a [`crate::PlugHandler`].
    pub fn p100(self) -> PlugHandler<Unauthenticated> {
        PlugHandler::new(self)
    }

    /// Specializes the given [`crate::ApiClient`] into a [`crate::PlugHandler`].
    pub fn p105(self) -> PlugHandler<Unauthenticated> {
        PlugHandler::new(self)
    }

    /// Specializes the given [`crate::ApiClient`] into a [`crate::EnergyMonitoringPlugHandler`].
    pub fn p110(self) -> EnergyMonitoringPlugHandler<Unauthenticated> {
        EnergyMonitoringPlugHandler::new(self)
    }

    /// Specializes the given [`crate::ApiClient`] into a [`crate::EnergyMonitoringPlugHandler`].
    pub fn p115(self) -> EnergyMonitoringPlugHandler<Unauthenticated> {
        EnergyMonitoringPlugHandler::new(self)
    }

    pub(crate) async fn login(&mut self) -> Result<(), Error> {
        // we have to clear the cookie jar otherwise all subsequent login requests will fail
        self.cookie_jar.clear();

        self.handshake().await?;
        self.login_request().await?;

        Ok(())
    }

    pub(crate) async fn get_device_info<R>(&self) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt + DeviceInfoResultExt,
    {
        debug!("Get Device info...");
        let get_device_info_params = GetDeviceInfoParams::new();
        let get_device_info_request =
            TapoRequest::GetDeviceInfo(TapoParams::new(get_device_info_params));

        let result = self
            .execute_secure_passthrough_request::<R>(get_device_info_request, true)
            .await?
            .map(|result| result.decode())
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))??;

        Ok(result)
    }

    pub(crate) async fn get_device_info_json(&self) -> Result<serde_json::Value, Error> {
        debug!("Get Device info as json...");
        let get_device_info_params = GetDeviceInfoParams::new();
        let get_device_info_request =
            TapoRequest::GetDeviceInfo(TapoParams::new(get_device_info_params));

        let result = self
            .execute_secure_passthrough_request::<serde_json::Value>(get_device_info_request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        Ok(result)
    }

    pub(crate) async fn get_device_usage(&self) -> Result<DeviceUsageResult, Error> {
        debug!("Get Device usage...");
        let get_device_usage_params = GetDeviceUsageParams::new();
        let get_device_usage_request =
            TapoRequest::GetDeviceUsage(TapoParams::new(get_device_usage_params));

        let result = self
            .execute_secure_passthrough_request::<DeviceUsageResult>(get_device_usage_request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        Ok(result)
    }

    pub(crate) async fn set_lighting_effect(
        &self,
        lighting_effect: LightingEffect,
    ) -> Result<(), Error> {
        debug!("Lighting effect will change to: {lighting_effect:?}");

        let set_lighting_effect_request = TapoRequest::SetLightingEffect(Box::new(
            TapoParams::new(lighting_effect)
                .set_request_time_mils()?
                .set_terminal_uuid(TERMINAL_UUID),
        ));

        self.execute_secure_passthrough_request::<TapoResult>(set_lighting_effect_request, true)
            .await?;

        Ok(())
    }

    pub(crate) async fn get_energy_usage(&self) -> Result<EnergyUsageResult, Error> {
        debug!("Get Energy usage...");
        let get_energy_usage_params = GetEnergyUsageParams::new();
        let get_energy_usage_request =
            TapoRequest::GetEnergyUsage(TapoParams::new(get_energy_usage_params));

        let result = self
            .execute_secure_passthrough_request::<EnergyUsageResult>(get_energy_usage_request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        Ok(result)
    }

    pub(crate) async fn get_energy_data(
        &self,
        interval: EnergyDataInterval,
    ) -> Result<EnergyDataResult, Error> {
        debug!("Get Energy data...");
        let get_energy_data_params = GetEnergyDataParams::new(interval);
        let get_energy_data_request =
            TapoRequest::GetEnergyData(TapoParams::new(get_energy_data_params));

        let result = self
            .execute_secure_passthrough_request::<EnergyDataResult>(get_energy_data_request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        Ok(result)
    }

    async fn handshake(&mut self) -> Result<(), Error> {
        debug!("Performing handshake...");
        let key_pair = KeyPair::new()?;

        let handshake_params = HandshakeParams::new(key_pair.get_public_key()?);
        let handshake_request = TapoRequest::Handshake(TapoParams::new(handshake_params));
        debug!("Handshake request: {}", json!(handshake_request));

        let body = serde_json::to_vec(&handshake_request)?;

        let response: TapoResponse<HandshakeResult> = self
            .client
            .post_async(&self.url, body)
            .await?
            .json()
            .await?;

        debug!("Device responded with: {response:?}");

        validate_response(&response)?;

        let handshake_key = response
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .key;

        let tp_link_cipher = TpLinkCipher::new(&handshake_key, &key_pair)?;

        self.tp_link_cipher = Some(tp_link_cipher);

        Ok(())
    }

    async fn login_request(&mut self) -> Result<(), Error> {
        debug!("Will login with username '{}'...", self.username);

        let login_device_params =
            TapoParams::new(LoginDeviceParams::new(&self.username, &self.password))
                .set_request_time_mils()?;
        let login_device_request = TapoRequest::LoginDevice(login_device_params);

        let result = self
            .execute_secure_passthrough_request::<TokenResult>(login_device_request, false)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        self.token.replace(result.token);

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

        let tp_link_cipher = self
            .tp_link_cipher
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("tp_link_cipher shouldn't be None"))?;

        let request_encrypted = tp_link_cipher.encrypt(&request_string)?;

        let secure_passthrough_params = SecurePassthroughParams::new(&request_encrypted);
        let secure_passthrough_request =
            TapoRequest::SecurePassthrough(TapoParams::new(secure_passthrough_params));
        debug!("Secure passthrough request: {secure_passthrough_request:?}",);

        let url = if with_token {
            format!(
                "{}?token={}",
                &self.url,
                self.token
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("token shouldn't be None"))?
            )
        } else {
            self.url.clone()
        };

        let response: TapoResponse<TapoResult> = self
            .client
            .post_async(url, serde_json::to_vec(&secure_passthrough_request)?)
            .await?
            .json()
            .await?;

        debug!("Device responded with: {response:?}");

        validate_response(&response)?;

        let inner_response_encrypted = response
            .result
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .response;

        let inner_response_decrypted = tp_link_cipher.decrypt(&inner_response_encrypted)?;

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
