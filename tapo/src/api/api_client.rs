use std::fmt;

use async_trait::async_trait;
use isahc::prelude::Configurable;
use isahc::HttpClient;
use log::debug;
use serde::de::DeserializeOwned;

use crate::api::protocol::{TapoProtocol, TapoProtocolExt};
use crate::api::{
    ColorLightHandler, ColorLightStripHandler, GenericDeviceHandler, HubHandler, LightHandler,
    PlugEnergyMonitoringHandler, PlugHandler,
};
use crate::error::{Error, TapoResponseError};
use crate::requests::{
    ControlChildParams, EmptyParams, EnergyDataInterval, GetEnergyDataParams, LightingEffect,
    MultipleRequestParams, TapoParams, TapoRequest,
};
use crate::responses::{
    validate_response, ControlChildResult, CurrentPowerResult, DecodableResultExt,
    EnergyDataResult, EnergyUsageResult, TapoMultipleResponse, TapoResponseExt, TapoResult,
};

const TERMINAL_UUID: &str = "00-00-00-00-00-00";

#[async_trait]
pub(crate) trait ApiClientExt: std::fmt::Debug + Send + Sync {
    async fn set_device_info(&self, device_info_params: serde_json::Value) -> Result<(), Error>;
}

/// Tapo API Client. See [examples](https://github.com/mihai-dinculescu/tapo/tree/main/tapo/examples).
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
#[derive(Debug, Clone)]
pub struct ApiClient {
    protocol: TapoProtocol,
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
            protocol: TapoProtocol::new(client, tapo_username.into(), tapo_password.into()),
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

    /// Specializes the given [`ApiClient`] into an authenticated [`PlugEnergyMonitoringHandler`].
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
    ) -> Result<PlugEnergyMonitoringHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(PlugEnergyMonitoringHandler::new(self))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`PlugEnergyMonitoringHandler`].
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
    ) -> Result<PlugEnergyMonitoringHandler, Error> {
        let url = build_url(&ip_address.into());
        self.login(url).await?;

        Ok(PlugEnergyMonitoringHandler::new(self))
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
    pub(crate) async fn login(&mut self, url: String) -> Result<(), Error> {
        self.protocol.login(url).await
    }

    pub(crate) async fn refresh_session(&mut self) -> Result<(), Error> {
        self.protocol.refresh_session().await
    }

    pub(crate) async fn get_device_info<R>(&self) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt + DecodableResultExt,
    {
        debug!("Get Device info...");
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.protocol
            .execute_request::<R>(request, true)
            .await?
            .map(|result| result.decode())
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
    }

    pub(crate) async fn get_device_usage<R>(&self) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        debug!("Get Device usage...");
        let request = TapoRequest::GetDeviceUsage(TapoParams::new(EmptyParams));

        self.protocol
            .execute_request::<R>(request, true)
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

        self.protocol
            .execute_request::<TapoResult>(request, true)
            .await?;

        Ok(())
    }

    pub(crate) async fn get_energy_usage(&self) -> Result<EnergyUsageResult, Error> {
        debug!("Get Energy usage...");
        let request = TapoRequest::GetEnergyUsage(TapoParams::new(EmptyParams));

        self.protocol
            .execute_request::<EnergyUsageResult>(request, true)
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

        self.protocol
            .execute_request::<EnergyDataResult>(request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn get_current_power(&self) -> Result<CurrentPowerResult, Error> {
        debug!("Get Current power...");
        let request = TapoRequest::GetCurrentPower(TapoParams::new(EmptyParams));

        self.protocol
            .execute_request::<CurrentPowerResult>(request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn get_child_device_list<R>(&self) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt + DecodableResultExt,
    {
        debug!("Get Child device list...");
        let request = TapoRequest::GetChildDeviceList(TapoParams::new(EmptyParams));

        self.protocol
            .execute_request::<R>(request, true)
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

        self.protocol
            .execute_request::<R>(request, true)
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
            .protocol
            .execute_request::<ControlChildResult<TapoMultipleResponse<R>>>(request, true)
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

        self.protocol
            .execute_request::<TapoResult>(set_device_info_request, true)
            .await?;

        Ok(())
    }
}

fn build_url(ip_address: &str) -> String {
    let url = format!("http://{}/app", ip_address);
    debug!("Device url: {url}");

    url
}
