use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use log::debug;
use reqwest::Client;
use serde::de::DeserializeOwned;
use tokio::sync::RwLock;

use crate::error::{Error, TapoResponseError};
use crate::requests::{
    ControlChildParams, DeviceRebootParams, EmptyParams, EnergyDataInterval,
    GetChildDeviceListParams, GetEnergyDataParams, GetPowerDataParams, LightingEffect,
    MultipleRequestParams, PlayAlarmParams, PowerDataInterval, TapoParams, TapoRequest,
};
use crate::responses::{
    ControlChildResult, CurrentPowerResult, DecodableResultExt, EnergyDataResult,
    EnergyDataResultRaw, EnergyUsageResult, PowerDataResult, PowerDataResultRaw,
    SupportedAlarmTypeListResult, TapoMultipleResponse, TapoResponseExt, TapoResult,
    validate_response,
};

use super::discovery::DeviceDiscovery;
use super::protocol::{TapoProtocol, TapoProtocolExt};
use super::{
    ColorLightHandler, GenericDeviceHandler, HubHandler, LightHandler, PlugEnergyMonitoringHandler,
    PlugHandler, PowerStripEnergyMonitoringHandler, PowerStripHandler, RgbLightStripHandler,
    RgbicLightStripHandler,
};

const TERMINAL_UUID: &str = "00-00-00-00-00-00";

/// Implemented by all ApiClient implementations.
#[async_trait]
pub trait ApiClientExt: std::fmt::Debug + Send + Sync {
    /// Sets device info by sending the given parameters.
    async fn set_device_info(&self, device_info_params: serde_json::Value) -> Result<(), Error>;
    /// Reboots the device.
    async fn device_reboot(&self, delay_s: u16) -> Result<(), Error>;
    /// Hardware resets the device.
    async fn device_reset(&self) -> Result<(), Error>;
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
///     let device = ApiClient::new("tapo-username@example.com", "tapo-password")
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
    tapo_username: String,
    tapo_password: String,
    timeout: Option<Duration>,
    protocol: Option<TapoProtocol>,
}

/// Tapo API Client constructor.
impl ApiClient {
    /// Returns a new instance of [`ApiClient`].
    /// It is cheaper to [`ApiClient::clone`] an existing instance than to create a new one when multiple devices need to be controller.
    /// This is because [`ApiClient::clone`] reuses the underlying [`reqwest::Client`].
    ///
    /// # Arguments
    ///
    /// * `tapo_username` - the Tapo username
    /// * `tapo_password` - the Tapo password
    ///
    /// Note: The default connection timeout is 30 seconds.
    /// Use [`ApiClient::with_timeout`] to change it.
    pub fn new(tapo_username: impl Into<String>, tapo_password: impl Into<String>) -> ApiClient {
        Self {
            tapo_username: tapo_username.into(),
            tapo_password: tapo_password.into(),
            timeout: None,
            protocol: None,
        }
    }

    /// Changes the connection timeout from the default value to the given value.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The new connection timeout value.
    pub fn with_timeout(mut self, timeout: Duration) -> ApiClient {
        self.timeout = Some(timeout);
        self
    }

    /// Discovers one or more devices located at a specified unicast or broadcast IP address.
    ///
    /// # Arguments
    /// * `target` - The IP address at which the discovery will take place.
    ///   This address can be either a unicast (e.g. `192.168.1.10`) or a
    ///   broadcast address (e.g. `192.168.1.255`, `255.255.255.255`, etc.).
    /// * `timeout_s` - The maximum time to wait for a response from the device(s) in seconds.
    ///   Must be between `1` and `60`.
    pub async fn discover_devices(
        self,
        target: impl Into<String>,
        timeout_s: u64,
    ) -> Result<DeviceDiscovery, Error> {
        if !(1..=60).contains(&timeout_s) {
            return Err(Error::Validation {
                field: "timeout_s".to_string(),
                message: "Must be between 1 and 60 seconds".to_string(),
            });
        }

        Ok(DeviceDiscovery::new(self, target, Duration::from_secs(timeout_s)).await?)
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
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
        self.login(ip_address).await?;

        Ok(GenericDeviceHandler::new(Arc::new(RwLock::new(self))))
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .l510("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l510(mut self, ip_address: impl Into<String>) -> Result<LightHandler, Error> {
        self.login(ip_address).await?;

        Ok(LightHandler::new(Arc::new(RwLock::new(self))))
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .l520("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l520(mut self, ip_address: impl Into<String>) -> Result<LightHandler, Error> {
        self.login(ip_address).await?;

        Ok(LightHandler::new(Arc::new(RwLock::new(self))))
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .l530("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l530(mut self, ip_address: impl Into<String>) -> Result<ColorLightHandler, Error> {
        self.login(ip_address).await?;

        Ok(ColorLightHandler::new(Arc::new(RwLock::new(self))))
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .l535("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l535(mut self, ip_address: impl Into<String>) -> Result<ColorLightHandler, Error> {
        self.login(ip_address).await?;

        Ok(ColorLightHandler::new(Arc::new(RwLock::new(self))))
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .l610("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l610(mut self, ip_address: impl Into<String>) -> Result<LightHandler, Error> {
        self.login(ip_address).await?;

        Ok(LightHandler::new(Arc::new(RwLock::new(self))))
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .l630("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l630(mut self, ip_address: impl Into<String>) -> Result<ColorLightHandler, Error> {
        self.login(ip_address).await?;

        Ok(ColorLightHandler::new(Arc::new(RwLock::new(self))))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`RgbLightStripHandler`].
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .l900("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l900(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<RgbLightStripHandler, Error> {
        self.login(ip_address).await?;

        Ok(RgbLightStripHandler::new(Arc::new(RwLock::new(self))))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`RgbicLightStripHandler`].
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .l920("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l920(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<RgbicLightStripHandler, Error> {
        self.login(ip_address).await?;

        Ok(RgbicLightStripHandler::new(Arc::new(RwLock::new(self))))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`RgbicLightStripHandler`].
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .l930("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l930(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<RgbicLightStripHandler, Error> {
        self.login(ip_address).await?;

        Ok(RgbicLightStripHandler::new(Arc::new(RwLock::new(self))))
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .p100("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn p100(mut self, ip_address: impl Into<String>) -> Result<PlugHandler, Error> {
        self.login(ip_address).await?;

        Ok(PlugHandler::new(Arc::new(RwLock::new(self))))
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .p105("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn p105(mut self, ip_address: impl Into<String>) -> Result<PlugHandler, Error> {
        self.login(ip_address).await?;

        Ok(PlugHandler::new(Arc::new(RwLock::new(self))))
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
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
        self.login(ip_address).await?;

        Ok(PlugEnergyMonitoringHandler::new(Arc::new(RwLock::new(
            self,
        ))))
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
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
        self.login(ip_address).await?;

        Ok(PlugEnergyMonitoringHandler::new(Arc::new(RwLock::new(
            self,
        ))))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`PowerStripHandler`].
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .p300("192.168.1.100")
    ///     .await?;
    /// let child_device_list = device.get_child_device_list().await?;
    /// println!("Child device list: {child_device_list:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn p300(mut self, ip_address: impl Into<String>) -> Result<PowerStripHandler, Error> {
        self.login(ip_address).await?;

        Ok(PowerStripHandler::new(Arc::new(RwLock::new(self))))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`PowerStripEnergyMonitoringHandler`].
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .p304("192.168.1.100")
    ///     .await?;
    /// let child_device_list = device.get_child_device_list().await?;
    /// println!("Child device list: {child_device_list:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn p304(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<PowerStripEnergyMonitoringHandler, Error> {
        self.login(ip_address).await?;

        Ok(PowerStripEnergyMonitoringHandler::new(Arc::new(
            RwLock::new(self),
        )))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`PowerStripHandler`].
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .p306("192.168.1.100")
    ///     .await?;
    /// let child_device_list = device.get_child_device_list().await?;
    /// println!("Child device list: {child_device_list:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn p306(mut self, ip_address: impl Into<String>) -> Result<PowerStripHandler, Error> {
        self.login(ip_address).await?;

        Ok(PowerStripHandler::new(Arc::new(RwLock::new(self))))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`PowerStripEnergyMonitoringHandler`].
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .p316("192.168.1.100")
    ///     .await?;
    /// let child_device_list = device.get_child_device_list().await?;
    /// println!("Child device list: {child_device_list:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn p316(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<PowerStripEnergyMonitoringHandler, Error> {
        self.login(ip_address).await?;

        Ok(PowerStripEnergyMonitoringHandler::new(Arc::new(
            RwLock::new(self),
        )))
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
    /// let device = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .h100("192.168.1.100")
    ///     .await?;
    ///
    /// let child_device_list = device.get_child_device_list().await?;
    /// println!("Child device list: {child_device_list:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn h100(mut self, ip_address: impl Into<String>) -> Result<HubHandler, Error> {
        self.login(ip_address).await?;

        Ok(HubHandler::new(Arc::new(RwLock::new(self))))
    }
}

/// Tapo API Client private methods.
impl ApiClient {
    pub(crate) async fn login(&mut self, ip_address: impl Into<String>) -> Result<(), Error> {
        let url = format!("http://{}/app", ip_address.into());
        debug!("Device url: {url}");

        let tapo_username = self.tapo_username.clone();
        let tapo_password = self.tapo_password.clone();

        self.get_protocol_mut()?
            .login(url, tapo_username, tapo_password)
            .await
    }

    pub(crate) async fn refresh_session(&mut self) -> Result<(), Error> {
        let tapo_username = self.tapo_username.clone();
        let tapo_password = self.tapo_password.clone();

        self.get_protocol_mut()?
            .refresh_session(tapo_username, tapo_password)
            .await
    }

    pub(crate) async fn get_supported_alarm_type_list(
        &self,
    ) -> Result<SupportedAlarmTypeListResult, Error> {
        let request = TapoRequest::GetSupportedAlarmTypeList(TapoParams::new(EmptyParams));

        self.get_protocol()?
            .execute_request(request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn play_alarm(&self, params: PlayAlarmParams) -> Result<(), Error> {
        let request = TapoRequest::PlayAlarm(TapoParams::new(params));

        self.get_protocol()?
            .execute_request::<serde_json::Value>(request, true)
            .await?;

        Ok(())
    }

    pub(crate) async fn stop_alarm(&self) -> Result<(), Error> {
        let request = TapoRequest::StopAlarm(TapoParams::new(EmptyParams));

        self.get_protocol()?
            .execute_request::<serde_json::Value>(request, true)
            .await?;

        Ok(())
    }

    pub(crate) async fn get_device_info<R>(&self) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt + DecodableResultExt,
    {
        debug!("Get Device info...");
        let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

        self.get_protocol()?
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

        self.get_protocol()?
            .execute_request(request, true)
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

        self.get_protocol()?
            .execute_request::<TapoResult>(request, true)
            .await?;

        Ok(())
    }

    pub(crate) async fn get_energy_usage(&self) -> Result<EnergyUsageResult, Error> {
        debug!("Get Energy usage...");
        let request = TapoRequest::GetEnergyUsage(TapoParams::new(EmptyParams));

        self.get_protocol()?
            .execute_request(request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn get_current_power(&self) -> Result<CurrentPowerResult, Error> {
        debug!("Get Current power...");
        let request = TapoRequest::GetCurrentPower(TapoParams::new(EmptyParams));

        self.get_protocol()?
            .execute_request(request, true)
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

        self.get_protocol()?
            .execute_request::<EnergyDataResultRaw>(request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
            .map(|result| result.try_into())?
    }

    pub(crate) async fn get_power_data(
        &self,
        interval: PowerDataInterval,
    ) -> Result<PowerDataResult, Error> {
        debug!("Get Power data...");
        let params = GetPowerDataParams::new(interval);
        let request = TapoRequest::GetPowerData(TapoParams::new(params));

        self.get_protocol()?
            .execute_request::<PowerDataResultRaw>(request, true)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
            .map(|result| result.try_into())?
    }

    pub(crate) async fn get_child_device_list<R>(&self, start_index: u64) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt + DecodableResultExt,
    {
        debug!("Get Child device list starting with index {start_index}...");
        let request = TapoRequest::GetChildDeviceList(TapoParams::new(
            GetChildDeviceListParams::new(start_index),
        ));

        self.get_protocol()?
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

        self.get_protocol()?
            .execute_request::<R>(request, true)
            .await?
            .map(|result| result.decode())
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
    }

    pub(crate) async fn control_child<R>(
        &self,
        device_id: String,
        child_request: TapoRequest,
    ) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        debug!("Control child...");
        let params = MultipleRequestParams::new(vec![child_request]);
        let request = TapoRequest::MultipleRequest(Box::new(TapoParams::new(params)));

        let params = ControlChildParams::new(device_id, request);
        let request = TapoRequest::ControlChild(Box::new(TapoParams::new(params)));

        let responses = self
            .get_protocol()?
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

        Ok(response.result)
    }

    fn get_protocol_mut(&mut self) -> Result<&mut TapoProtocol, Error> {
        if self.protocol.is_none() {
            let timeout = self.timeout.unwrap_or_else(|| Duration::from_secs(30));

            let client = Client::builder()
                .http1_title_case_headers()
                .timeout(timeout)
                .build()?;
            let protocol = TapoProtocol::new(client);
            self.protocol.replace(protocol);
        }

        self.protocol.as_mut().ok_or_else(|| {
            Error::Other(anyhow::anyhow!(
                "The protocol should have been initialized already"
            ))
        })
    }

    fn get_protocol(&self) -> Result<&TapoProtocol, Error> {
        self.protocol.as_ref().ok_or_else(|| {
            Error::Other(anyhow::anyhow!(
                "The protocol should have been initialized already"
            ))
        })
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

        self.get_protocol()?
            .execute_request::<TapoResult>(set_device_info_request, true)
            .await?;

        Ok(())
    }

    async fn device_reboot(&self, delay: u16) -> Result<(), Error> {
        debug!("Device reboot...");
        let request = TapoRequest::DeviceReboot(TapoParams::new(DeviceRebootParams::new(delay)));

        self.get_protocol()?
            .execute_request::<serde_json::Value>(request, true)
            .await?;

        Ok(())
    }

    async fn device_reset(&self) -> Result<(), Error> {
        debug!("Device reset...");
        let request = TapoRequest::DeviceReset(TapoParams::new(EmptyParams));

        self.get_protocol()?
            .execute_request::<serde_json::Value>(request, true)
            .await?;

        Ok(())
    }
}
