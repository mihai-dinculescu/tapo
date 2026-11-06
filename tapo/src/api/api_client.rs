use std::fmt;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, TimeZone as _, Utc};
use chrono_tz::{OffsetComponents as _, Tz};
use log::{debug, warn};
use reqwest::Client;
use serde::de::DeserializeOwned;
use tokio_stream::StreamExt as _;

use crate::error::{Error, TapoResponseError};
use crate::requests::{
    AddTimerParams, ControlChildParams, DeviceRebootParams, EmptyObjectParams, EmptyParams,
    EnergyDataInterval, GetChildDeviceListParams, GetEnergyDataParams, GetPowerDataParams,
    GetScheduleRulesParams, GetWirelessScanInfoParams, LightingEffect, MultipleRequestParams,
    PlayAlarmParams, PowerDataInterval, RemoveScheduleRulesParams, RemoveTimersParams,
    ScheduleRule, ScheduleRuleRaw, SegmentEffect, SetQsInfoParams, SmartCamControlChildParams,
    SmartCamGetChildDeviceListParams, SmartCamGetDeviceInfoParams,
    SmartCamSearchDetectionListParams, TapoParams, TapoRequest,
};
use crate::responses::{
    AddScheduleRuleResult, AddTimerResult, ControlChildResult, CurrentPowerResult,
    DecodableResultExt, DetectionListResultRaw, DetectionResult, EnergyDataResult,
    EnergyDataResultRaw, EnergyMeterDataResult, EnergyUsageResult, FirmwareUpdateStateResult,
    FirmwareUpdateStateResultRaw, LatestFirmwareResult, LatestFirmwareResultRaw, PowerDataResult,
    PowerDataResultRaw, PowerState, RecordingDownloadResult, ScheduleRuleListResultRaw,
    ScheduleRuleResult, SmartCamControlChildResult, TapoMultipleResponse, TapoResponseExt,
    TapoResult, Timer, TimerListResultRaw, TimezoneResult, TimezoneResultRaw,
    WifiConnStatusResultRaw, WifiJoinResult, WifiNetwork, WifiScanResultRaw, WifiSecurity,
    validate_response,
};
use crate::utils::unix_timestamp_seconds;

#[cfg(feature = "debug")]
use crate::requests::SmartCamGetAppComponentListParams;
#[cfg(feature = "debug")]
use crate::responses::{
    AppComponentListResultRaw, ChildDeviceComponentList, ChildDeviceComponentListResult, Component,
    ComponentListResult, SupportedAlarmTypeListResult,
};

use super::discovery::{DeviceDiscovery, DeviceDiscoveryRaw};
use super::protocol::media_stream;
use super::protocol::media_stream::download::DownloadRequest;
use super::protocol::{AuthProtocol, DeviceFamily, TapoProtocol};

#[cfg(feature = "bluetooth")]
use super::protocol::{BluetoothSetupDevice, discover_bluetooth_setup_devices};
use super::{
    CameraHandler, CameraHubHandler, CameraPtzHandler, ColorLightHandler, HubHandler, HubIrHandler,
    LightHandler, PlugEnergyMonitoringHandler, PlugHandler, PowerStripEnergyMonitoringHandler,
    PowerStripHandler, RgbLightStripHandler, RgbicLightStripHandler, SetupHandler,
};

const TERMINAL_UUID: &str = "00-00-00-00-00-00";
/// Used when the caller does not set one via [`ApiClient::with_timeout`].
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
/// How long a device in setup mode is given to answer the discovery query
/// that starts its web server. The wait ends as soon as it answers, and the
/// query is sent again every 3 seconds until then.
const SETUP_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(10);
/// Camera hubs (H200, H500) authenticate the local session and the media
/// stream with this username and the TP-Link cloud password, rather than the
/// cloud account username.
const CAMERA_HUB_USERNAME: &str = "admin";
/// A device in setup mode logs in to this account until it has joined a
/// Wi-Fi network, as the Tapo app does.
const SETUP_USERNAME: &str = "test@tp-link.net";
const SETUP_PASSWORD: &str = "test";

/// Backstop against a firmware whose reported rule count never lets the
/// listing loop finish. A P110 returns 5 rules per page and stores at most 32,
/// so a full device takes 7 pages.
const MAX_SCHEDULE_RULE_PAGES: u32 = 16;

/// Backstop against a firmware whose reported network count never lets the
/// Wi-Fi scan loop finish.
const MAX_WIFI_SCAN_PAGES: u32 = 16;

/// How often, and for how long, a device that was set up over Bluetooth is
/// asked whether it joined the Wi-Fi network, as the Tapo app does.
const WIFI_JOIN_POLL_INTERVAL: Duration = Duration::from_secs(2);
const WIFI_JOIN_POLL_TIMEOUT: Duration = Duration::from_secs(65);

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
#[derive(Clone)]
pub struct ApiClient {
    tapo_username: String,
    tapo_password: String,
    timeout: Option<Duration>,
    protocol: Option<TapoProtocol>,
}

/// Obscures the password and leaves out the protocol, which holds the
/// session keys and tokens.
impl fmt::Debug for ApiClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiClient")
            .field("tapo_username", &self.tapo_username)
            .field("tapo_password", &"OBSCURED")
            .field("timeout", &self.timeout)
            .finish_non_exhaustive()
    }
}

/// Tapo API Client constructor.
impl ApiClient {
    /// Returns a new instance of [`ApiClient`].
    /// [`ApiClient::clone`] an existing instance when multiple devices need to be controlled.
    /// Each clone opens its own connections once it is specialized into a device handler.
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
}

/// Device discovery.
impl ApiClient {
    /// Discovers one or more devices located at a specified unicast or broadcast IP address.
    ///
    #[cfg_attr(
        feature = "debug",
        doc = "To get each device's raw discovery JSON instead, use [`ApiClient::discover_devices_raw`] (requires the `debug` feature)."
    )]
    #[cfg_attr(
        not(feature = "debug"),
        doc = "To get each device's raw discovery JSON instead, use `ApiClient::discover_devices_raw` (requires the `debug` feature)."
    )]
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

/// Debug API.
#[cfg(feature = "debug")]
impl ApiClient {
    /// Discovers one or more devices located at a specified unicast or broadcast IP address,
    /// returning the IP address of the devices found and their discovery JSON response.
    ///
    /// # Arguments
    /// * `target` - The IP address at which the discovery will take place.
    ///   This address can be either a unicast (e.g. `192.168.1.10`) or a
    ///   broadcast address (e.g. `192.168.1.255`, `255.255.255.255`, etc.).
    /// * `timeout_s` - The maximum time to wait for a response from the device(s) in seconds.
    ///   Must be between `1` and `60`.
    pub async fn discover_devices_raw(
        target: impl Into<String>,
        timeout_s: u64,
    ) -> Result<DeviceDiscoveryRaw, Error> {
        if !(1..=60).contains(&timeout_s) {
            return Err(Error::Validation {
                field: "timeout_s".to_string(),
                message: "Must be between 1 and 60 seconds".to_string(),
            });
        }

        let target_ip = target.into().parse().map_err(|e| Error::Validation {
            field: "target".to_string(),
            message: format!("Invalid IP address: {e}"),
        })?;

        Ok(DeviceDiscoveryRaw::new(target_ip, Duration::from_secs(timeout_s)).await?)
    }
}

/// Device handler builders.
impl ApiClient {
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(LightHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(LightHandler::new(self, address))
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
    ///     .l430("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn l430(mut self, ip_address: impl Into<String>) -> Result<ColorLightHandler, Error> {
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(ColorLightHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(ColorLightHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(ColorLightHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(LightHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(ColorLightHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(RgbLightStripHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(RgbicLightStripHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(RgbicLightStripHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(PlugHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(PlugHandler::new(self, address))
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
    ///     .p125("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn p125(mut self, ip_address: impl Into<String>) -> Result<PlugHandler, Error> {
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(PlugHandler::new(self, address))
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
    ///     .tp10("192.168.1.100")
    ///     .await?;
    /// device.on().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tp10(mut self, ip_address: impl Into<String>) -> Result<PlugHandler, Error> {
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(PlugHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(PlugEnergyMonitoringHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(PlugEnergyMonitoringHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(PowerStripHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(PowerStripEnergyMonitoringHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(PowerStripHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(PowerStripEnergyMonitoringHandler::new(self, address))
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
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(HubHandler::new(self, address))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`HubIrHandler`].
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
    ///     .h110("192.168.1.100")
    ///     .await?;
    ///
    /// let child_device_list = device.get_child_device_list().await?;
    /// println!("Child device list: {child_device_list:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn h110(mut self, ip_address: impl Into<String>) -> Result<HubIrHandler, Error> {
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::Smart,
                AuthProtocol::Unknown,
            )
            .await?;

        Ok(HubIrHandler::new(self, address))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`CameraHubHandler`].
    ///
    /// The hub authenticates with the TP-Link cloud password; the configured
    /// username is ignored (the local `admin` account is used internally).
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
    ///     .h200("192.168.1.100")
    ///     .await?;
    ///
    /// let device_info = device.get_device_info().await?;
    /// println!("Device info: {device_info:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn h200(mut self, ip_address: impl Into<String>) -> Result<CameraHubHandler, Error> {
        self.prepare_for_camera_hub();
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::SmartCam,
                AuthProtocol::AesSsl,
            )
            .await?;

        Ok(CameraHubHandler::new(self, address))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`CameraHubHandler`].
    ///
    /// The hub authenticates with the TP-Link cloud password; the configured
    /// username is ignored (the local `admin` account is used internally).
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
    ///     .h500("192.168.1.100")
    ///     .await?;
    ///
    /// let device_info = device.get_device_info().await?;
    /// println!("Device info: {device_info:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn h500(mut self, ip_address: impl Into<String>) -> Result<CameraHubHandler, Error> {
        self.prepare_for_camera_hub();
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::SmartCam,
                AuthProtocol::AesSsl,
            )
            .await?;

        Ok(CameraHubHandler::new(self, address))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`CameraHandler`].
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
    ///     .c120("192.168.1.100")
    ///     .await?;
    ///
    /// let device_info = device.get_device_info().await?;
    /// println!("Device info: {device_info:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn c120(mut self, ip_address: impl Into<String>) -> Result<CameraHandler, Error> {
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::SmartCam,
                AuthProtocol::TpapThenAesSsl,
            )
            .await?;

        Ok(CameraHandler::new(self, address))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`CameraPtzHandler`].
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
    ///     .c210("192.168.1.100")
    ///     .await?;
    ///
    /// let device_info = device.get_device_info().await?;
    /// println!("Device info: {device_info:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn c210(mut self, ip_address: impl Into<String>) -> Result<CameraPtzHandler, Error> {
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::SmartCam,
                AuthProtocol::TpapThenAesSsl,
            )
            .await?;

        Ok(CameraPtzHandler::new(self, address))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`CameraPtzHandler`].
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
    ///     .c220("192.168.1.100")
    ///     .await?;
    ///
    /// let device_info = device.get_device_info().await?;
    /// println!("Device info: {device_info:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn c220(mut self, ip_address: impl Into<String>) -> Result<CameraPtzHandler, Error> {
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::SmartCam,
                AuthProtocol::TpapThenAesSsl,
            )
            .await?;

        Ok(CameraPtzHandler::new(self, address))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`CameraPtzHandler`].
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
    ///     .c225("192.168.1.100")
    ///     .await?;
    ///
    /// let device_info = device.get_device_info().await?;
    /// println!("Device info: {device_info:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn c225(mut self, ip_address: impl Into<String>) -> Result<CameraPtzHandler, Error> {
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::SmartCam,
                AuthProtocol::TpapThenAesSsl,
            )
            .await?;

        Ok(CameraPtzHandler::new(self, address))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`CameraPtzHandler`].
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
    ///     .c325wb("192.168.1.100")
    ///     .await?;
    ///
    /// let device_info = device.get_device_info().await?;
    /// println!("Device info: {device_info:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn c325wb(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<CameraPtzHandler, Error> {
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::SmartCam,
                AuthProtocol::TpapThenAesSsl,
            )
            .await?;

        Ok(CameraPtzHandler::new(self, address))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`CameraPtzHandler`].
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
    ///     .c520ws("192.168.1.100")
    ///     .await?;
    ///
    /// let device_info = device.get_device_info().await?;
    /// println!("Device info: {device_info:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn c520ws(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<CameraPtzHandler, Error> {
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::SmartCam,
                AuthProtocol::TpapThenAesSsl,
            )
            .await?;

        Ok(CameraPtzHandler::new(self, address))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`CameraPtzHandler`].
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
    ///     .tc40("192.168.1.100")
    ///     .await?;
    ///
    /// let device_info = device.get_device_info().await?;
    /// println!("Device info: {device_info:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tc40(mut self, ip_address: impl Into<String>) -> Result<CameraPtzHandler, Error> {
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::SmartCam,
                AuthProtocol::TpapThenAesSsl,
            )
            .await?;

        Ok(CameraPtzHandler::new(self, address))
    }

    /// Specializes the given [`ApiClient`] into an authenticated [`CameraPtzHandler`].
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
    ///     .tc70("192.168.1.100")
    ///     .await?;
    ///
    /// let device_info = device.get_device_info().await?;
    /// println!("Device info: {device_info:?}");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tc70(mut self, ip_address: impl Into<String>) -> Result<CameraPtzHandler, Error> {
        let address = self
            .login(
                &ip_address.into(),
                DeviceFamily::SmartCam,
                AuthProtocol::TpapThenAesSsl,
            )
            .await?;

        Ok(CameraPtzHandler::new(self, address))
    }
}

/// Device setup.
impl ApiClient {
    /// Specializes the given [`ApiClient`] into a [`SetupHandler`] for a new
    /// or factory-reset device that broadcasts a setup network. See
    /// [`SetupHandler`] for how to reach the device.
    ///
    /// The device takes the client's credentials as its login once it joins
    /// a Wi-Fi network. They need not be those of a TP-Link account.
    ///
    /// # Arguments
    ///
    /// * `ip_address` - the IP address of the device on its setup network,
    ///   usually `192.168.0.1`
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # use tapo::responses::WifiSecurity;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let setup = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .setup_over_wifi("192.168.0.1")
    ///     .await?;
    ///
    /// let networks = setup.scan_wifi().await?;
    /// println!("Networks: {networks:?}");
    ///
    /// setup
    ///     .join_wifi(
    ///         "wifi-name",
    ///         "wifi-password",
    ///         WifiSecurity::Wpa2Psk,
    ///         chrono_tz::Europe::London,
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn setup_over_wifi(
        mut self,
        ip_address: impl Into<String>,
    ) -> Result<SetupHandler, Error> {
        let address = self.login_for_setup(&ip_address.into()).await?;

        Ok(SetupHandler::new(self, Some(address)))
    }

    /// Discovers the new or factory-reset devices nearby that can be set up
    /// over Bluetooth, such as the
    /// [P110M](https://www.tapo.com/en/search/?q=P110M). A device that is
    /// already set up is left out. Pass one to
    /// [`ApiClient::setup_over_bluetooth`].
    ///
    /// # Arguments
    ///
    /// * `timeout_s` - How long to scan for, in seconds. Must be between `1`
    ///   and `60`.
    ///
    /// # Errors
    ///
    /// Returns an error if `timeout_s` is out of range, if the host has no
    /// Bluetooth adapter, or if the scan fails.
    #[cfg(feature = "bluetooth")]
    pub async fn discover_bluetooth_setup_devices(
        timeout_s: u64,
    ) -> Result<Vec<BluetoothSetupDevice>, Error> {
        if !(1..=60).contains(&timeout_s) {
            return Err(Error::Validation {
                field: "timeout_s".to_string(),
                message: "Must be between 1 and 60 seconds".to_string(),
            });
        }

        discover_bluetooth_setup_devices(Duration::from_secs(timeout_s)).await
    }

    /// Specializes the given [`ApiClient`] into a [`SetupHandler`] for a new
    /// or factory-reset device that is set up over Bluetooth, as found by
    /// [`ApiClient::discover_bluetooth_setup_devices`].
    ///
    /// The device takes the client's credentials as its login once it joins
    /// a Wi-Fi network. They need not be those of a TP-Link account.
    ///
    /// Starting another setup of the same device ends this one, even if that
    /// setup fails. This handler then returns an error.
    ///
    /// # Arguments
    ///
    /// * `device` - the device, as returned by [`ApiClient::discover_bluetooth_setup_devices`]
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use tapo::ApiClient;
    /// # use tapo::responses::WifiSecurity;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let devices = ApiClient::discover_bluetooth_setup_devices(10).await?;
    /// let device = devices.first().ok_or("No device to set up")?;
    ///
    /// let setup = ApiClient::new("tapo-username@example.com", "tapo-password")
    ///     .setup_over_bluetooth(device)
    ///     .await?;
    ///
    /// let networks = setup.scan_wifi().await?;
    /// println!("Networks: {networks:?}");
    ///
    /// setup
    ///     .join_wifi(
    ///         "wifi-name",
    ///         "wifi-password",
    ///         WifiSecurity::Wpa2Psk,
    ///         chrono_tz::Europe::London,
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "bluetooth")]
    pub async fn setup_over_bluetooth(
        mut self,
        device: &BluetoothSetupDevice,
    ) -> Result<SetupHandler, Error> {
        self.init_protocol(DeviceFamily::Smart, true)?;
        let timeout = self.timeout();

        let protocol = self
            .protocol
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("The protocol should have been initialized"))?;
        protocol.login_over_bluetooth(device, timeout).await?;

        Ok(SetupHandler::new(self, None))
    }
}

/// Tapo API Client private methods.
impl ApiClient {
    /// Prepares the client for a camera hub (H200, H500). Must be called
    /// before the login.
    ///
    /// Switches to the local `admin` account that camera hubs require. The
    /// override persists, so session refreshes reuse it.
    pub(crate) fn prepare_for_camera_hub(&mut self) {
        self.tapo_username = CAMERA_HUB_USERNAME.to_string();
    }

    pub(crate) async fn login(
        &mut self,
        ip_address: &str,
        device_family: DeviceFamily,
        auth_protocol: AuthProtocol,
    ) -> Result<(String, u16), Error> {
        self.login_on_port(ip_address, None, device_family, auth_protocol)
            .await
    }

    /// Logs in to the device at `ip_address` and returns the IP address and
    /// the port that it was reached at.
    ///
    /// `ip_address` is either the address that a builder was given, which
    /// may already carry a port (`"{ip}:{port}"`), or a bare IP address from
    /// discovery. `port` is the port that discovery saw the device announce
    /// its web server on. It is appended to `ip_address` as is, so it must
    /// only be passed with a bare IP address.
    pub(crate) async fn login_on_port(
        &mut self,
        ip_address: &str,
        port: Option<u16>,
        device_family: DeviceFamily,
        auth_protocol: AuthProtocol,
    ) -> Result<(String, u16), Error> {
        self.init_protocol(device_family, false)?;

        let (protocol, tapo_username, tapo_password) = self.protocol_with_credentials()?;

        protocol
            .login(
                ip_address,
                port,
                tapo_username,
                tapo_password,
                device_family,
                auth_protocol,
            )
            .await
    }

    /// Logs in to a device in setup mode at `ip_address` with the login
    /// that such a device takes, and returns the IP address and the port
    /// that it was reached at. The client's own credentials are left for
    /// the device to take as its login once it joins a Wi-Fi network.
    pub(crate) async fn login_for_setup(
        &mut self,
        ip_address: &str,
    ) -> Result<(String, u16), Error> {
        // A device in setup mode starts its web server only once it gets a
        // discovery query, and refuses connections until then. A failed
        // query does not stop the login, because a device whose web server
        // is already running can still be set up.
        let ip = ip_address.parse::<IpAddr>().ok().or_else(|| {
            ip_address
                .parse::<SocketAddr>()
                .ok()
                .map(|address| address.ip())
        });

        if let Some(ip) = ip {
            match DeviceDiscoveryRaw::new(ip, SETUP_DISCOVERY_TIMEOUT).await {
                Ok(mut discovery) => match discovery.next().await {
                    Some(Ok(_)) => debug!("The device answered the discovery query"),
                    Some(Err(error)) => debug!("The discovery query failed: {error}"),
                    None => debug!("The device did not answer the discovery query"),
                },
                Err(error) => debug!("The discovery query could not be sent: {error}"),
            }
        }

        self.init_protocol(DeviceFamily::Smart, true)?;

        let protocol = self
            .protocol
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("The protocol should have been initialized"))?;

        protocol
            .login_for_setup(ip_address, SETUP_USERNAME, SETUP_PASSWORD)
            .await
    }

    /// Builds the protocol, whose HTTP client opens a connection per request
    /// for a camera or camera hub, and when `connection_per_request` is set.
    fn init_protocol(
        &mut self,
        device_family: DeviceFamily,
        connection_per_request: bool,
    ) -> Result<(), Error> {
        if self.protocol.is_none() {
            let mut builder = Client::builder()
                .http1_title_case_headers()
                .timeout(self.timeout())
                .danger_accept_invalid_certs(true);

            // A camera or camera hub gets a connection per request. A
            // camera hub advertises `keep-alive` but closes the connection
            // after a login request, and a request sent on that connection
            // before the close is noticed fails. A camera refuses a TPAP
            // login request on a connection that has carried a request of a
            // session. Every request opens its own connection, not only the
            // ones around a login, because a session refresh can log in
            // again at any point.
            //
            // A device in setup mode gets one too. It leaves its setup
            // network by dropping the connection of the request that puts it
            // on a Wi-Fi network, and a request sent on a connection that it
            // closed while idle, as when the user takes a while to pick a
            // network, fails the same way without having reached it.
            if device_family == DeviceFamily::SmartCam || connection_per_request {
                builder = builder.pool_max_idle_per_host(0);
            }

            // `join_wifi` takes a timeout as sent, so a connection that is
            // never made must fail as a connect error first. Half of the
            // timeout always runs out before the whole.
            if connection_per_request {
                builder = builder.connect_timeout(self.timeout() / 2);
            }

            self.protocol = Some(TapoProtocol::new(builder.build()?));
        }

        Ok(())
    }

    pub(crate) async fn refresh_session(&mut self) -> Result<(), Error> {
        let (protocol, tapo_username, tapo_password) = self.protocol_with_credentials()?;

        protocol.refresh_session(tapo_username, tapo_password).await
    }

    pub(crate) async fn download_recording<W: tokio::io::AsyncWrite + Unpin + Send>(
        &self,
        ip_address: &str,
        player_id: &str,
        child_device_id: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        writer: &mut W,
    ) -> Result<RecordingDownloadResult, Error> {
        debug!("Download recording...");

        if end_time <= start_time {
            return Err(Error::Validation {
                field: "end_time".to_string(),
                message: "Must be after start_time".to_string(),
            });
        }

        // The hub ends the clip itself; the time limit (twice the clip's
        // length on top of the configured timeout) is a backstop for a hub
        // that streams slowly or never reports the end.
        let clip_length = (end_time - start_time)
            .to_std()
            .map_err(anyhow::Error::from)?;
        let time_limit = self.timeout() + clip_length * 2;

        let start_time = unix_timestamp_seconds("start_time", start_time)?;
        let end_time = unix_timestamp_seconds("end_time", end_time)?;

        let connection = media_stream::authenticate(
            ip_address,
            &self.tapo_username,
            &self.tapo_password,
            &child_device_id,
            player_id,
            self.timeout(),
        )
        .await?;

        let request = DownloadRequest {
            device_id: child_device_id,
            player_id: player_id.to_string(),
            start_time,
            end_time,
        };

        media_stream::download::download(connection, request, time_limit, writer).await
    }

    /// The detections on a camera's SD card within the given time range
    /// (`searchDetectionList`), all pages of them.
    ///
    /// # Errors
    ///
    /// Returns an error if `end_time` is before `start_time` or more than 24
    /// hours after it, or if either is before 1970.
    pub(crate) async fn get_detections(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<DetectionResult>, Error> {
        // The Tapo app fetches pages of 100 (`start_index: 0`, `end_index: 99`)
        // and stops paging past a start index of 12288.
        const PAGE_SIZE: u64 = 100;
        const MAX_START_INDEX: u64 = 12288;

        debug!("Get detections...");

        if end_time < start_time {
            return Err(Error::Validation {
                field: "end_time".to_string(),
                message: "Must not be before start_time".to_string(),
            });
        }
        // The camera answers a search window of 25 hours or more with
        // `-71105`; the app only ever searches one day at a time.
        if end_time - start_time > chrono::Duration::hours(24) {
            return Err(Error::Validation {
                field: "end_time".to_string(),
                message: "Must be at most 24 hours after start_time".to_string(),
            });
        }

        let start_time = unix_timestamp_seconds("start_time", start_time)?;
        let end_time = unix_timestamp_seconds("end_time", end_time)?;

        let mut results = Vec::new();
        let mut start_index = 0;

        while start_index <= MAX_START_INDEX {
            let request = TapoRequest::SmartCamSearchDetectionList(TapoParams::new(
                SmartCamSearchDetectionListParams::new(
                    start_time,
                    end_time,
                    start_index,
                    start_index + PAGE_SIZE - 1,
                ),
            ));

            let page = self
                .execute_smart_cam_multiple_request::<DetectionListResultRaw>(request)
                .await?;

            let to_be_continued = page.to_be_continued();
            results.extend(page.detections());

            if !to_be_continued {
                break;
            }
            start_index += PAGE_SIZE;
        }

        Ok(results)
    }

    #[cfg(feature = "debug")]
    pub(crate) async fn get_supported_alarm_type_list(
        &self,
    ) -> Result<SupportedAlarmTypeListResult, Error> {
        let request = TapoRequest::GetSupportedAlarmTypeList(TapoParams::new(EmptyParams));

        self.protocol()?
            .execute_request(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn play_alarm(&self, params: PlayAlarmParams) -> Result<(), Error> {
        let request = TapoRequest::PlayAlarm(TapoParams::new(params));

        self.protocol()?
            .execute_request::<serde_json::Value>(request)
            .await?;

        Ok(())
    }

    pub(crate) async fn stop_alarm(&self) -> Result<(), Error> {
        let request = TapoRequest::StopAlarm(TapoParams::new(EmptyParams));

        self.protocol()?
            .execute_request::<serde_json::Value>(request)
            .await?;

        Ok(())
    }

    #[cfg(feature = "debug")]
    pub(crate) async fn get_component_list(&self) -> Result<Vec<Component>, Error> {
        debug!("Get Component list...");

        match self.protocol()?.device_family() {
            DeviceFamily::SmartCam => {
                let request = TapoRequest::SmartCamGetAppComponentList(TapoParams::new(
                    SmartCamGetAppComponentListParams::new(),
                ));

                self.execute_smart_cam_multiple_request::<AppComponentListResultRaw>(request)
                    .await
                    .map(|result| result.components())
            }
            DeviceFamily::Smart => {
                let request = TapoRequest::ComponentNegotiation(TapoParams::new(EmptyParams));

                let result: ComponentListResult = self
                    .protocol()?
                    .execute_request(request)
                    .await?
                    .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

                Ok(result.component_list)
            }
        }
    }

    pub(crate) async fn get_device_info<R>(&self) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt + DecodableResultExt,
    {
        debug!("Get Device info...");

        match self.protocol()?.device_family() {
            DeviceFamily::SmartCam => {
                let request = TapoRequest::SmartCamGetDeviceInfo(TapoParams::new(
                    SmartCamGetDeviceInfoParams::new(),
                ));

                self.execute_smart_cam_section_request(request).await
            }
            DeviceFamily::Smart => {
                let request = TapoRequest::GetDeviceInfo(TapoParams::new(EmptyParams));

                self.protocol()?
                    .execute_request::<R>(request)
                    .await?
                    .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
                    .decode()
            }
        }
    }

    pub(crate) async fn get_current_power(&self) -> Result<CurrentPowerResult, Error> {
        debug!("Get Current power...");
        let request = TapoRequest::GetCurrentPower(TapoParams::new(EmptyParams));

        self.protocol()?
            .execute_request(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn get_energy_meter_data(&self) -> Result<EnergyMeterDataResult, Error> {
        debug!("Get Energy meter data...");
        let request = TapoRequest::GetEmeterData(TapoParams::new(EmptyParams));

        self.protocol()?
            .execute_request(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn get_latest_firmware(&self) -> Result<LatestFirmwareResult, Error> {
        debug!("Get Latest firmware...");
        let request = TapoRequest::GetLatestFw(TapoParams::new(EmptyParams));

        let raw = self
            .protocol()?
            .execute_request::<LatestFirmwareResultRaw>(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        Ok(raw.try_into()?)
    }

    pub(crate) async fn start_firmware_update(&self) -> Result<(), Error> {
        debug!("Start Firmware update...");
        let request = TapoRequest::FwDownload(TapoParams::new(EmptyParams));

        self.protocol()?
            .execute_request::<serde_json::Value>(request)
            .await?;

        Ok(())
    }

    pub(crate) async fn get_firmware_update_state(
        &self,
    ) -> Result<FirmwareUpdateStateResult, Error> {
        debug!("Get Firmware update state...");
        let request = TapoRequest::GetFwDownloadState(TapoParams::new(EmptyParams));

        let raw = self
            .protocol()?
            .execute_request::<FirmwareUpdateStateResultRaw>(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        Ok(raw.try_into()?)
    }

    pub(crate) async fn get_device_usage<R>(&self) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        debug!("Get Device usage...");
        let request = TapoRequest::GetDeviceUsage(TapoParams::new(EmptyParams));

        self.protocol()?
            .execute_request(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn get_energy_data(
        &self,
        interval: EnergyDataInterval,
        timezone: Option<Tz>,
    ) -> Result<EnergyDataResult, Error> {
        let timezone = match timezone {
            Some(timezone) => timezone,
            None => self.get_timezone().await?.timezone,
        };

        debug!("Get Energy data...");
        let params = GetEnergyDataParams::new(interval, &timezone)?;
        let request = TapoRequest::GetEnergyData(TapoParams::new(params));

        self.protocol()?
            .execute_request::<EnergyDataResultRaw>(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .in_timezone(&timezone)
    }

    pub(crate) async fn get_energy_usage(&self) -> Result<EnergyUsageResult, Error> {
        debug!("Get Energy usage...");
        let request = TapoRequest::GetEnergyUsage(TapoParams::new(EmptyParams));

        self.protocol()?
            .execute_request(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn get_power_data(
        &self,
        interval: PowerDataInterval,
    ) -> Result<PowerDataResult, Error> {
        debug!("Get Power data...");
        let params = GetPowerDataParams::new(interval);
        let request = TapoRequest::GetPowerData(TapoParams::new(params));

        self.protocol()?
            .execute_request::<PowerDataResultRaw>(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
            .map(|result| result.try_into())?
    }

    pub(crate) async fn get_timezone(&self) -> Result<TimezoneResult, Error> {
        debug!("Get Timezone...");
        let request = TapoRequest::GetDeviceTime(TapoParams::new(EmptyParams));

        self.protocol()?
            .execute_request::<TimezoneResultRaw>(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
            .map(Into::into)
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

        self.protocol()?
            .execute_request::<TapoResult>(request)
            .await?;

        Ok(())
    }

    pub(crate) async fn set_segment_effect(
        &self,
        segment_effect: SegmentEffect,
    ) -> Result<(), Error> {
        debug!("Segment effect will change to: {segment_effect:?}");

        let request = TapoRequest::ApplySegmentEffectRule(Box::new(
            TapoParams::new(segment_effect)
                .set_request_time_mils()?
                .set_terminal_uuid(TERMINAL_UUID),
        ));

        self.protocol()?
            .execute_request::<TapoResult>(request)
            .await?;

        Ok(())
    }

    pub(crate) async fn get_child_device_list<R>(&self, start_index: u64) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt + DecodableResultExt,
    {
        debug!("Get Child device list starting with index {start_index}...");

        match self.protocol()?.device_family() {
            DeviceFamily::SmartCam => {
                let request = TapoRequest::SmartCamGetChildDeviceList(TapoParams::new(
                    SmartCamGetChildDeviceListParams::new(start_index),
                ));

                self.execute_smart_cam_multiple_request::<R>(request)
                    .await?
                    .decode()
            }
            DeviceFamily::Smart => {
                let request = TapoRequest::GetChildDeviceList(TapoParams::new(
                    GetChildDeviceListParams::new(start_index),
                ));

                self.protocol()?
                    .execute_request::<R>(request)
                    .await?
                    .map(|result| result.decode())
                    .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            }
        }
    }

    #[cfg(feature = "debug")]
    pub(crate) async fn get_child_device_component_list(
        &self,
    ) -> Result<Vec<ChildDeviceComponentList>, Error> {
        debug!("Get Child device component list...");

        let result: ChildDeviceComponentListResult = match self.protocol()?.device_family() {
            DeviceFamily::SmartCam => {
                let request = TapoRequest::SmartCamGetChildDeviceComponentList(TapoParams::new(
                    SmartCamGetChildDeviceListParams::new(0),
                ));

                self.execute_smart_cam_multiple_request(request).await?
            }
            DeviceFamily::Smart => {
                let request =
                    TapoRequest::GetChildDeviceComponentList(TapoParams::new(EmptyParams));

                self.protocol()?
                    .execute_request(request)
                    .await?
                    .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            }
        };

        Ok(result.child_component_list)
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

        match self.protocol()?.device_family() {
            DeviceFamily::SmartCam => {
                // SmartCam hubs take the child request directly in a
                // controlChild envelope (no inner multipleRequest), and nest
                // the child's response under a snake_case `response_data`
                // field (seen from the H500). A refusal replaces that field
                // with an `err_code` (seen from the H200).
                let params = SmartCamControlChildParams::new(device_id, child_request);
                let request = TapoRequest::SmartCamControlChild(Box::new(TapoParams::new(params)));

                let result = self
                    .execute_smart_cam_multiple_request::<SmartCamControlChildResult<R>>(request)
                    .await?;

                validate_response(result.err_code)?;

                let response = result
                    .response_data
                    .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

                validate_response(response.error_code)?;

                Ok(response.result)
            }
            DeviceFamily::Smart => {
                let params = MultipleRequestParams::new(vec![child_request]);
                let request = TapoRequest::MultipleRequest(Box::new(TapoParams::new(params)));

                let params = ControlChildParams::new(device_id, request);
                let request = TapoRequest::ControlChild(Box::new(TapoParams::new(params)));

                let responses = self
                    .protocol()?
                    .execute_request::<ControlChildResult<TapoMultipleResponse<R>>>(request)
                    .await?
                    .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
                    .response_data
                    .result
                    .responses;

                let response = responses
                    .into_iter()
                    .next()
                    .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

                validate_response(response.error_code)?;

                Ok(response.result)
            }
        }
    }

    /// Executes a SmartCam request that reads a section of the device's
    /// configuration, returning what the section holds.
    pub(crate) async fn execute_smart_cam_section_request<R>(
        &self,
        request: TapoRequest,
    ) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        let result = self
            .execute_smart_cam_multiple_request::<serde_json::Value>(request)
            .await?;

        let leaf = extract_section_leaf(result)
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        Ok(serde_json::from_value(leaf)?)
    }

    /// Executes a SmartCam request whose result is of no interest.
    pub(crate) async fn execute_smart_cam_command(
        &self,
        request: TapoRequest,
    ) -> Result<(), Error> {
        self.execute_smart_cam_request_opt::<serde_json::Value>(request)
            .await?;

        Ok(())
    }

    /// Executes a SmartCam request, returning its result.
    ///
    /// # Errors
    ///
    /// Returns an error if the response has no result.
    pub(crate) async fn execute_smart_cam_multiple_request<R>(
        &self,
        request: TapoRequest,
    ) -> Result<R, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        self.execute_smart_cam_request_opt(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    /// A camera or camera hub takes a request only inside a
    /// `multipleRequest`, so every request is sent as the only request of
    /// one. The inner response is read as JSON first, because a refused
    /// request comes with an empty `result`, which has to be reported
    /// as the device's error code rather than as a failure to decode it.
    async fn execute_smart_cam_request_opt<R>(
        &self,
        request: TapoRequest,
    ) -> Result<Option<R>, Error>
    where
        R: fmt::Debug + DeserializeOwned + TapoResponseExt,
    {
        let request = TapoRequest::MultipleRequest(Box::new(TapoParams::new(
            MultipleRequestParams::new(vec![request]),
        )));

        let responses = self
            .protocol()?
            .execute_request::<TapoMultipleResponse<serde_json::Value>>(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?
            .result
            .responses;

        let response = responses
            .into_iter()
            .next()
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        validate_response(response.error_code)?;

        response
            .result
            .map(serde_json::from_value::<R>)
            .transpose()
            .map_err(Into::into)
    }

    pub(crate) async fn set_timer(
        &self,
        delay: Duration,
        desired_state: PowerState,
    ) -> Result<Timer, Error> {
        // 24h is the maximum the Tapo app allows, and matches what the
        // device accepts in practice.
        const MAX_DELAY_S: u32 = 24 * 60 * 60;

        let delay_s: u32 = delay.as_secs().try_into().unwrap_or(u32::MAX);
        if delay_s == 0 || delay_s > MAX_DELAY_S {
            return Err(Error::Validation {
                field: "delay".into(),
                message: format!("Must be between 1 second and 24 hours ({MAX_DELAY_S} seconds)"),
            });
        }

        // The P110 supports at most one armed timer at a time, and
        // rejects a second `add_countdown_rule` while one is armed.
        // Wipe first to make `set_timer` a true replace.
        self.clear_timer().await?;

        let params = AddTimerParams::new(delay_s, desired_state);
        let request = TapoRequest::AddCountdownRule(TapoParams::new(params));

        let result = self
            .protocol()?
            .execute_request::<AddTimerResult>(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        Ok(Timer {
            id: result.id,
            delay_s,
            remaining_s: delay_s,
            desired_state,
        })
    }

    pub(crate) async fn get_timer(&self) -> Result<Option<Timer>, Error> {
        let request = TapoRequest::GetCountdownRules(TapoParams::new(EmptyObjectParams {}));
        let list = self
            .protocol()?
            .execute_request::<TimerListResultRaw>(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;
        list.rule_list
            .into_iter()
            .next()
            .map(|raw| raw.into_timer())
            .transpose()
    }

    pub(crate) async fn clear_timer(&self) -> Result<(), Error> {
        let request =
            TapoRequest::RemoveCountdownRules(TapoParams::new(RemoveTimersParams::remove_all()));
        self.protocol()?
            .execute_request::<serde_json::Value>(request)
            .await?;
        Ok(())
    }

    pub(crate) async fn add_schedule_rule(
        &self,
        rule: ScheduleRule,
    ) -> Result<ScheduleRuleResult, Error> {
        // The device assigns the id, so never send one it might honour.
        let mut params = ScheduleRuleRaw::from(&rule);
        params.id = None;

        let request = TapoRequest::AddScheduleRule(TapoParams::new(params));
        let added = self
            .protocol()?
            .execute_request::<AddScheduleRuleResult>(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

        Ok(rule.into_result(added.id))
    }

    pub(crate) async fn edit_schedule_rule(&self, rule: ScheduleRule) -> Result<(), Error> {
        if rule.id().is_none() {
            return Err(Error::Validation {
                field: "rule.id".into(),
                message: "edit_schedule_rule requires rule.id to be set".into(),
            });
        }

        let request = TapoRequest::EditScheduleRule(TapoParams::new(ScheduleRuleRaw::from(&rule)));
        self.protocol()?
            .execute_request::<serde_json::Value>(request)
            .await?;

        Ok(())
    }

    pub(crate) async fn get_schedule_rules(&self) -> Result<Vec<ScheduleRuleResult>, Error> {
        let mut all = Vec::new();
        // Counted separately from `all`, which holds only the rules that
        // parsed: a skipped rule must still advance us towards `sum`, or the
        // loop would never reach it.
        let mut fetched = 0u32;
        let mut start_index = 0u32;

        for _ in 0..MAX_SCHEDULE_RULE_PAGES {
            let page = self.get_schedule_rules_page(start_index).await?;
            let returned = page.rule_list.len() as u32;

            if returned == 0 {
                break;
            }

            for raw in page.rule_list {
                // Parse per rule, so one the library cannot represent (an
                // unknown `s_type` from a newer app, say) degrades the
                // listing instead of failing it outright.
                match ScheduleRuleResult::try_from(raw) {
                    Ok(rule) => all.push(rule),
                    Err(e) => warn!("Skipping a schedule rule that could not be parsed: {e}"),
                }
            }

            fetched += returned;
            if fetched >= page.sum {
                break;
            }

            start_index += returned;
        }

        Ok(all)
    }

    pub(crate) async fn get_max_schedule_rules(&self) -> Result<u32, Error> {
        Ok(self
            .get_schedule_rules_page(0)
            .await?
            .schedule_rule_max_count)
    }

    async fn get_schedule_rules_page(
        &self,
        start_index: u32,
    ) -> Result<ScheduleRuleListResultRaw, Error> {
        let request =
            TapoRequest::GetScheduleRules(TapoParams::new(GetScheduleRulesParams { start_index }));

        self.protocol()?
            .execute_request::<ScheduleRuleListResultRaw>(request)
            .await?
            .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))
    }

    pub(crate) async fn remove_schedule_rule(&self, id: String) -> Result<(), Error> {
        let request = TapoRequest::RemoveScheduleRules(TapoParams::new(
            RemoveScheduleRulesParams::specific(vec![id]),
        ));
        self.protocol()?
            .execute_request::<serde_json::Value>(request)
            .await?;

        Ok(())
    }

    pub(crate) async fn remove_all_schedule_rules(&self) -> Result<(), Error> {
        let request = TapoRequest::RemoveScheduleRules(TapoParams::new(
            RemoveScheduleRulesParams::remove_all(),
        ));
        self.protocol()?
            .execute_request::<serde_json::Value>(request)
            .await?;

        Ok(())
    }

    /// The Wi-Fi networks that a device in setup mode sees, all pages of them.
    pub(crate) async fn scan_wifi(&self) -> Result<Vec<WifiNetwork>, Error> {
        debug!("Scan Wi-Fi...");
        let mut networks = Vec::new();

        for _ in 0..MAX_WIFI_SCAN_PAGES {
            let request = TapoRequest::GetWirelessScanInfo(TapoParams::new(
                GetWirelessScanInfoParams::new(networks.len()),
            ));

            let page: WifiScanResultRaw = self
                .protocol()?
                .execute_request(request)
                .await?
                .ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

            let received = page.ap_list.len();
            for network in page.ap_list {
                networks.push(WifiNetwork::try_from(network)?);
            }

            // The Tapo app asks for the next page, starting after the
            // networks received so far, until it has `sum` of them.
            if received == 0 || networks.len() >= page.sum {
                return Ok(networks);
            }
        }

        warn!(
            "Wi-Fi scan stopped after {MAX_WIFI_SCAN_PAGES} pages; the device may have reported more networks"
        );

        Ok(networks)
    }

    /// Sends a device in setup mode the Wi-Fi network to join, with the
    /// client's credentials as its login.
    pub(crate) async fn join_wifi(
        &self,
        ssid: &str,
        password: &str,
        security: WifiSecurity,
        timezone: Tz,
    ) -> Result<WifiJoinResult, Error> {
        debug!("Join Wi-Fi...");

        if ssid.is_empty() {
            return Err(Error::Validation {
                field: "ssid".to_string(),
                message: "Must not be empty".to_string(),
            });
        }
        // `scan_wifi` puts U+FFFD in place of bytes of a network name that
        // are not UTF-8, so such a name is not the network's, and the device
        // would leave its setup network for a network that does not exist.
        if ssid.contains(char::REPLACEMENT_CHARACTER) {
            return Err(Error::Validation {
                field: "ssid".to_string(),
                message:
                    "Must not hold U+FFFD, which scan_wifi puts in place of a name that is not UTF-8"
                        .to_string(),
            });
        }
        if security == WifiSecurity::Unknown {
            return Err(Error::Validation {
                field: "security".to_string(),
                message: "Must be a security type that the library knows".to_string(),
            });
        }
        // The device leaves its setup network without saying whether it
        // could join, so a password left out by mistake would take a factory
        // reset to undo.
        if password.is_empty() && security != WifiSecurity::None {
            return Err(Error::Validation {
                field: "password".to_string(),
                message: "Must not be empty for a network with security".to_string(),
            });
        }

        // The device's clock reads the year 2000 in setup mode, so it is
        // set from the host's. The Tapo app sends the standard offset,
        // without daylight saving. chrono-tz takes the winter time of a
        // zone whose summer time is its standard time, such as Europe/Dublin,
        // as the daylight saving one, so the offset of such a zone is off by
        // the difference between the two.
        let now = Utc::now();
        let standard_offset = timezone
            .offset_from_utc_datetime(&now.naive_utc())
            .base_utc_offset()
            .num_minutes();

        let request = TapoRequest::SetQsInfo(Box::new(TapoParams::new(SetQsInfoParams::new(
            &self.tapo_username,
            &self.tapo_password,
            ssid,
            password,
            security,
            now.timestamp(),
            standard_offset,
            timezone.name(),
        ))));

        let protocol = self.protocol()?;

        // A device that is set up over Bluetooth answers, and says whether
        // it joined when asked.
        if protocol.is_bluetooth() {
            protocol
                .execute_request::<serde_json::Value>(request)
                .await?;

            return self.wait_for_wifi_join(ssid).await;
        }

        // A device that joins over its setup network leaves it without
        // answering, so the request ends in a dropped connection or a
        // timeout. Only a device that could not be reached at all did not
        // get it.
        match protocol.execute_request::<serde_json::Value>(request).await {
            Ok(_) => Ok(WifiJoinResult::Sent),
            Err(Error::Http(error)) if !error.is_connect() => {
                debug!("The device left its setup network without answering: {error}");
                Ok(WifiJoinResult::Sent)
            }
            Err(error) => Err(error),
        }
    }

    /// Asks a device that was set up over Bluetooth whether it joined the
    /// Wi-Fi network `ssid`, until it says so or the wait runs out.
    async fn wait_for_wifi_join(&self, ssid: &str) -> Result<WifiJoinResult, Error> {
        let deadline = tokio::time::Instant::now() + WIFI_JOIN_POLL_TIMEOUT;

        let mut last_status: Option<WifiConnStatusResultRaw> = None;

        loop {
            let request = TapoRequest::GetWifiConnStatus(TapoParams::new(EmptyParams));
            // A poll takes no longer than the time left, so that the wait
            // ends at the deadline.
            let Ok(response) =
                tokio::time::timeout_at(deadline, self.protocol()?.execute_request(request)).await
            else {
                break;
            };
            let status: WifiConnStatusResultRaw =
                response?.ok_or_else(|| Error::Tapo(TapoResponseError::EmptyResult))?;

            if status.status == "connected" {
                return Ok(WifiJoinResult::Connected);
            }
            last_status = Some(status);

            if tokio::time::Instant::now() + WIFI_JOIN_POLL_INTERVAL > deadline {
                break;
            }
            tokio::time::sleep(WIFI_JOIN_POLL_INTERVAL).await;
        }

        let (status, reason) = last_status.map_or_else(
            || ("unknown".to_string(), "none".to_string()),
            |status| {
                let reason = status
                    .last_fail_reason
                    .map_or_else(|| "none".to_string(), |reason| reason.to_string());
                (status.status, reason)
            },
        );

        Err(Error::Tapo(TapoResponseError::ResponseError {
            description: format!(
                "The device did not join {ssid} within {} seconds (status: {status}, last fail reason: {reason}). Check the password and factory reset the device to try again.",
                WIFI_JOIN_POLL_TIMEOUT.as_secs(),
            ),
        }))
    }

    pub(crate) fn timeout(&self) -> Duration {
        self.timeout.unwrap_or(DEFAULT_TIMEOUT)
    }

    /// The protocol, along with the username and password it logs in with.
    fn protocol_with_credentials(&mut self) -> Result<(&mut TapoProtocol, &str, &str), Error> {
        let protocol = self.protocol.as_mut().ok_or_else(|| {
            Error::Other(anyhow::anyhow!(
                "The protocol should have been initialized already"
            ))
        })?;

        Ok((protocol, &self.tapo_username, &self.tapo_password))
    }

    fn protocol(&self) -> Result<&TapoProtocol, Error> {
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

        self.protocol()?
            .execute_request::<TapoResult>(set_device_info_request)
            .await?;

        Ok(())
    }

    async fn device_reboot(&self, delay: u16) -> Result<(), Error> {
        debug!("Device reboot...");
        let request = TapoRequest::DeviceReboot(TapoParams::new(DeviceRebootParams::new(delay)));

        self.protocol()?
            .execute_request::<serde_json::Value>(request)
            .await?;

        Ok(())
    }

    async fn device_reset(&self) -> Result<(), Error> {
        debug!("Device reset...");
        let request = TapoRequest::DeviceReset(TapoParams::new(EmptyParams));

        self.protocol()?
            .execute_request::<serde_json::Value>(request)
            .await?;

        Ok(())
    }
}

fn extract_section_leaf(result: serde_json::Value) -> Option<serde_json::Value> {
    // SmartCam get responses place data under a single section key
    // (e.g. "device_info": {"basic_info": {...}}). Extract the leaf object.
    let serde_json::Value::Object(result) = result else {
        return None;
    };

    let serde_json::Value::Object(section) = result.into_values().next()? else {
        return None;
    };

    let leaf = section.into_values().next()?;
    leaf.is_object().then_some(leaf)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn section_leaf_of_a_get_result_is_extracted() {
        // What a C220 answers `getDeviceInfo` with.
        let result = json!({ "device_info": { "basic_info": { "device_model": "C220" } } });

        assert_eq!(
            extract_section_leaf(result),
            Some(json!({ "device_model": "C220" }))
        );
    }

    #[test]
    fn section_leaf_of_an_empty_result_is_none() {
        for result in [json!({}), json!({ "device_info": {} }), json!(null)] {
            assert_eq!(extract_section_leaf(result), None);
        }
    }

    #[test]
    fn test_debug_obscures_the_password() {
        let client = ApiClient::new("user@example.com", "hunter2");

        assert_eq!(
            format!("{client:?}"),
            r#"ApiClient { tapo_username: "user@example.com", tapo_password: "OBSCURED", timeout: None, .. }"#
        );
    }
}
