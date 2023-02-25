use std::fmt;
use std::marker::PhantomData;

use anyhow::Context;
use isahc::{config::Configurable, cookies::CookieJar, AsyncReadResponseExt, HttpClient};
use log::debug;
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::devices::{GenericDevice, TapoDeviceExt};
use crate::encryption::{decode_handshake_key, KeyPair, TpLinkCipher};
use crate::requests::{
    EnergyDataInterval, GenericSetDeviceInfoParams, GetDeviceInfoParams, GetDeviceUsageParams,
    GetEnergyDataParams, GetEnergyUsageParams, HandshakeParams, LoginDeviceParams,
    SecurePassthroughParams, TapoParams, TapoRequest,
};
use crate::responses::{
    validate_result, DeviceInfoResultExt, DeviceUsageResult, EnergyDataResult, EnergyUsageResult,
    HandshakeResult, TapoResponse, TapoResponseExt, TapoResult, TokenResult,
};

const TERMINAL_UUID: &str = "00-00-00-00-00-00";

/// Tapo API Client. See [examples](https://github.com/mihai-dinculescu/tapo/tree/main/examples).
///
/// # Examples
/// ## L530
/// ```rust,no_run
/// use tapo::{ApiClient, L530};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::<L530>::new(
///         "192.168.1.100".to_string(),
///         "tapo-username@example.com".to_string(),
///         "tapo-password".to_string(),
///         true,
///     ).await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## L510
/// ```rust,no_run
/// use tapo::{ApiClient, L510};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::<L510>::new(
///         "192.168.1.100".to_string(),
///         "tapo-username@example.com".to_string(),
///         "tapo-password".to_string(),
///         true,
///     ).await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## P110
/// ```rust,no_run
/// use tapo::{ApiClient, P110};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::<P110>::new(
///         "192.168.1.100".to_string(),
///         "tapo-username@example.com".to_string(),
///         "tapo-password".to_string(),
///         true,
///     ).await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## P100
/// ```rust,no_run
/// use tapo::{ApiClient, P100};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::<P100>::new(
///         "192.168.1.100".to_string(),
///         "tapo-username@example.com".to_string(),
///         "tapo-password".to_string(),
///         true,
///     ).await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
///
/// ## GenericDevice
/// ```rust,no_run
/// use tapo::{ApiClient, GenericDevice};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let device = ApiClient::<GenericDevice>::new(
///         "192.168.1.100".to_string(),
///         "tapo-username@example.com".to_string(),
///         "tapo-password".to_string(),
///         true,
///     ).await?;
///
///     device.on().await?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct ApiClient<D = GenericDevice>
where
    D: TapoDeviceExt,
{
    device_type: PhantomData<D>,
    client: HttpClient,
    url: String,
    username: String,
    password: String,
    cookie_jar: CookieJar,
    tp_link_cipher: Option<TpLinkCipher>,
    token: Option<String>,
}

/// The functionality of [`crate::ApiClient<D>`] that's common to all devices.
impl<D> ApiClient<D>
where
    D: TapoDeviceExt,
{
    /// Returns a new instance of [`crate::ApiClient<D>`]. If `attempt_login` is `true`, a login will be attempted.
    ///
    /// # Arguments
    ///
    /// * `ip_address` - *String*; the IP address of the device
    /// * `tapo_username` - *String*; the Tapo username
    /// * `tapo_password` - *String*; the Tapo password
    /// * `attempt_login` - *bool*; whether to attempt to login
    ///
    /// # Examples
    /// ```rust,no_run
    /// use tapo::{ApiClient, L530};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let device = ApiClient::<L530>::new(
    ///         "192.168.1.100".to_string(),
    ///         "tapo-username@example.com".to_string(),
    ///         "tapo-password".to_string(),
    ///         true,
    ///     ).await?;
    ///
    ///     device.on().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(
        ip_address: String,
        tapo_username: String,
        tapo_password: String,
        attempt_login: bool,
    ) -> anyhow::Result<ApiClient<D>> {
        let url = format!("http://{ip_address}/app");
        debug!("Device url: {url}");

        let cookie_jar = CookieJar::new();
        let client = HttpClient::builder()
            .title_case_headers(true)
            .cookie_jar(cookie_jar.clone())
            .build()?;

        let mut instance = Self {
            device_type: PhantomData,
            client,
            url,
            username: tapo_username,
            password: tapo_password,
            cookie_jar,
            tp_link_cipher: None,
            token: None,
        };

        if attempt_login {
            instance.login().await?;
        }

        Ok(instance)
    }

    /// Attempts to login. This function can be called multiple times for the same [`crate::ApiClient<D>`].
    pub async fn login(&mut self) -> anyhow::Result<()> {
        // we have to clear the cookie jar otherwise all subsequent login requests will fail
        self.cookie_jar.clear();

        self.handshake().await?;
        self.login_request().await?;

        Ok(())
    }

    /// Turns *on* the device.
    pub async fn on(&self) -> anyhow::Result<()> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(true)?)?;
        self.set_device_info_internal(json).await
    }

    /// Turns *off* the device.
    pub async fn off(&self) -> anyhow::Result<()> {
        let json = serde_json::to_value(GenericSetDeviceInfoParams::device_on(false)?)?;
        self.set_device_info_internal(json).await
    }

    /// Gets *device info* as [`serde_json::Value`].
    pub async fn get_device_info_json(&self) -> anyhow::Result<serde_json::Value> {
        debug!("Get Device info as json...");
        let get_device_info_params = GetDeviceInfoParams::new();
        let get_device_info_request =
            TapoRequest::GetDeviceInfo(TapoParams::new(get_device_info_params));

        let result = self
            .execute_secure_passthrough_request::<serde_json::Value>(get_device_info_request, true)
            .await?
            .context("failed to obtain a response for get device info")?;

        Ok(result)
    }

    /// Gets *device usage*. It contains the time in use, the power consumption, and the energy savings of the device.
    pub async fn get_device_usage(&self) -> anyhow::Result<DeviceUsageResult> {
        debug!("Get Device usage...");
        let get_device_usage_params = GetDeviceUsageParams::new();
        let get_device_usage_request =
            TapoRequest::GetDeviceUsage(TapoParams::new(get_device_usage_params));

        let result = self
            .execute_secure_passthrough_request::<DeviceUsageResult>(get_device_usage_request, true)
            .await?
            .context("failed to obtain a response for get device usage")?;

        Ok(result)
    }

    pub(crate) async fn set_device_info_internal(
        &self,
        device_info_params: serde_json::Value,
    ) -> anyhow::Result<()> {
        debug!("Device info will change to: {device_info_params:?}");

        let set_device_info_request = TapoRequest::SetDeviceInfo(
            TapoParams::new(device_info_params)
                .set_request_time_mils()?
                .set_terminal_uuid(TERMINAL_UUID),
        );

        self.execute_secure_passthrough_request::<TapoResult>(set_device_info_request, true)
            .await?;

        Ok(())
    }

    pub(crate) async fn get_device_info_internal<R>(&self) -> anyhow::Result<R>
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
            .context("failed to obtain a response for get device info")??;

        Ok(result)
    }

    pub(crate) async fn get_energy_usage_internal(&self) -> anyhow::Result<EnergyUsageResult> {
        debug!("Get Energy usage...");
        let get_energy_usage_params = GetEnergyUsageParams::new();
        let get_energy_usage_request =
            TapoRequest::GetEnergyUsage(TapoParams::new(get_energy_usage_params));

        let result = self
            .execute_secure_passthrough_request::<EnergyUsageResult>(get_energy_usage_request, true)
            .await?
            .context("failed to obtain a response for get energy usage")?;

        Ok(result)
    }

    pub(crate) async fn get_energy_data_internal(
        &self,
        interval: EnergyDataInterval,
    ) -> anyhow::Result<EnergyDataResult> {
        debug!("Get Energy data...");
        let get_energy_data_params = GetEnergyDataParams::new(interval);
        let get_energy_data_request =
            TapoRequest::GetEnergyData(TapoParams::new(get_energy_data_params));

        let result = self
            .execute_secure_passthrough_request::<EnergyDataResult>(get_energy_data_request, true)
            .await?
            .context("failed to obtain a response for get energy data")?;

        Ok(result)
    }

    async fn handshake(&mut self) -> anyhow::Result<()> {
        debug!("Performing handshake...");
        let key_pair = KeyPair::new()?;

        let handshake_params = HandshakeParams::new(&key_pair.public_key);
        let handshake_request = TapoRequest::Handshake(TapoParams::new(handshake_params));
        debug!("Handshake request: {}", json!(handshake_request));

        let response: TapoResponse<HandshakeResult> = self
            .client
            .post_async(&self.url, serde_json::to_vec(&handshake_request)?)
            .await?
            .json()
            .await?;

        debug!("Device responded with: {response:?}");

        validate_result(&response)?;

        let handshake_key = response
            .result
            .context("failed to find the result component of the handshake response")?
            .key;
        let tp_link_cipher = decode_handshake_key(&handshake_key, &key_pair)?;

        self.tp_link_cipher = Some(tp_link_cipher);

        Ok(())
    }

    async fn login_request(&mut self) -> anyhow::Result<()> {
        debug!("Will login with username '{}'...", self.username);

        let login_device_params =
            TapoParams::new(LoginDeviceParams::new(&self.username, &self.password)?)
                .set_request_time_mils()?;
        let login_device_request = TapoRequest::LoginDevice(login_device_params);

        let result = self
            .execute_secure_passthrough_request::<TokenResult>(login_device_request, false)
            .await?
            .context("failed to find the result component of the login inner response")?;

        let token = result.token;

        self.token.replace(token);

        Ok(())
    }

    async fn execute_secure_passthrough_request<R>(
        &self,
        request: TapoRequest,
        with_token: bool,
    ) -> anyhow::Result<Option<R>>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        let request_string = serde_json::to_string(&request)?;
        debug!("Request to passthrough: {request_string}");

        let request_encrypted = self
            .tp_link_cipher
            .as_ref()
            .context("failed to find tp_link_cipher")?
            .encrypt(&request_string)?;

        let secure_passthrough_params = SecurePassthroughParams::new(&request_encrypted);
        let secure_passthrough_request =
            TapoRequest::SecurePassthrough(TapoParams::new(secure_passthrough_params));
        debug!("Secure passthrough request: {secure_passthrough_request:?}",);

        let url = if with_token {
            format!(
                "{}?token={}",
                &self.url,
                self.token.as_ref().context("failed to find token")?
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

        validate_result(&response)?;

        let inner_response_encrypted = response
            .result
            .context("failed to find the result component of the execute command response")?
            .response;

        let inner_response_decrypted = self
            .tp_link_cipher
            .as_ref()
            .context("failed to find tp_link_cipher")?
            .decrypt(&inner_response_encrypted)?;

        debug!("Device inner response decrypted: {inner_response_decrypted}");

        let inner_response: TapoResponse<R> = serde_json::from_str(&inner_response_decrypted)?;

        debug!("Device inner response: {inner_response:?}");

        validate_result(&inner_response)?;

        let result = inner_response.result;

        Ok(result)
    }
}
