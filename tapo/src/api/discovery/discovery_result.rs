use anyhow::Context;

use crate::responses::{
    DecodableResultExt, DeviceInfoBasicResult, DeviceInfoColorLightResult, DeviceInfoHubResult,
    DeviceInfoLightResult, DeviceInfoPlugEnergyMonitoringResult, DeviceInfoPlugResult,
    DeviceInfoPowerStripResult, DeviceInfoRgbLightStripResult, DeviceInfoRgbicLightStripResult,
};
use crate::{
    ApiClient, ColorLightHandler, Error, HubHandler, LightHandler, PlugEnergyMonitoringHandler,
    PlugHandler, PowerStripEnergyMonitoringHandler, PowerStripHandler, RgbLightStripHandler,
    RgbicLightStripHandler,
};

use crate::api::protocol::AuthProtocol;

use super::DeviceType;
use super::discovery_raw_result::DiscoveryRawResult;

#[derive(Debug)]
/// Result of the device discovery process.
pub enum DiscoveryResult {
    /// A Tapo device without a specific handler implementation.
    ///
    /// If you believe that this device is already supported through one of the existing handlers, or would like to explore adding support for a currently
    /// unsupported model, please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues)
    /// to start the discussion.
    Other {
        /// Device info of a Tapo device without a specific handler implementation.
        ///
        /// If you believe that this device is already supported through one of the existing handlers, or would like to explore adding support for a currently
        /// unsupported model, please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues)
        /// to start the discussion.
        device_info: Box<DeviceInfoBasicResult>,
        /// The IP address of the device.
        ip: String,
    },
    /// Tapo L510, L520 and L610 devices.
    Light {
        /// Device info of Tapo L510, L520 and L610.
        device_info: Box<DeviceInfoLightResult>,
        /// Handler for the [L510](https://www.tapo.com/en/search/?q=L510),
        /// [L520](https://www.tapo.com/en/search/?q=L520) and
        /// [L610](https://www.tapo.com/en/search/?q=L610) devices.
        handler: LightHandler,
    },
    /// Tapo L530, L535 and L630 devices.
    ColorLight {
        /// Device info of Tapo L530, L535 and L630.
        device_info: Box<DeviceInfoColorLightResult>,
        /// Handler for the [L530](https://www.tapo.com/en/search/?q=L530),
        /// [L535](https://www.tapo.com/en/search/?q=L535) and
        /// [L630](https://www.tapo.com/en/search/?q=L630) devices.
        handler: ColorLightHandler,
    },
    /// Tapo L900 devices.
    RgbLightStrip {
        /// Device info of Tapo L900.
        device_info: Box<DeviceInfoRgbLightStripResult>,
        /// Handler for the [L900](https://www.tapo.com/en/search/?q=L900) devices.
        handler: RgbLightStripHandler,
    },
    /// Tapo L920 and L930 devices.
    RgbicLightStrip {
        /// Device info of Tapo L920 and L930.
        device_info: Box<DeviceInfoRgbicLightStripResult>,
        /// Handler for the [L920](https://www.tapo.com/en/search/?q=L920) and
        /// [L930](https://www.tapo.com/en/search/?q=L930) devices.
        handler: RgbicLightStripHandler,
    },
    /// Tapo P100 and P105 devices.
    Plug {
        /// Device info of Tapo P100 and P105.
        device_info: Box<DeviceInfoPlugResult>,
        /// Handler for the [P100](https://www.tapo.com/en/search/?q=P100) and
        /// [P105](https://www.tapo.com/en/search/?q=P105) devices.
        handler: PlugHandler,
    },
    /// Tapo P110, P110M and P115 devices.
    PlugEnergyMonitoring {
        /// Device info of Tapo P110, P110M and P115.
        device_info: Box<DeviceInfoPlugEnergyMonitoringResult>,
        /// Handler for the [P110](https://www.tapo.com/en/search/?q=P110),
        /// [P110M](https://www.tapo.com/en/search/?q=P110M) and
        /// [P115](https://www.tapo.com/en/search/?q=P115) devices.
        handler: PlugEnergyMonitoringHandler,
    },
    /// Tapo P300 and P306 devices.
    PowerStrip {
        /// Device info of Tapo P300 and P306.
        device_info: Box<DeviceInfoPowerStripResult>,
        /// Handler for the [P300](https://www.tapo.com/en/search/?q=P300) and
        /// [P306](https://www.tp-link.com/us/search/?q=P306) devices.
        handler: PowerStripHandler,
    },
    /// Tapo P304M and P316M devices.
    PowerStripEnergyMonitoring {
        /// Device info of Tapo P304M and P316M.
        device_info: Box<DeviceInfoPowerStripResult>,
        /// Handler for the [P304M](https://www.tp-link.com/uk/search/?q=P304M) and
        /// [P316M](https://www.tp-link.com/us/search/?q=P316M) devices.
        handler: PowerStripEnergyMonitoringHandler,
    },
    /// Tapo H100 devices.
    Hub {
        /// Device info of Tapo H100.
        device_info: Box<DeviceInfoHubResult>,
        /// Handler for the [H100](https://www.tapo.com/en/search/?q=H100) devices.
        handler: HubHandler,
    },
}

macro_rules! map_device_model {
    ($discovery_result_type:ident, $device_info_type:ident, $handler_type:ident, $device_info:expr, $client:expr) => {{
        DiscoveryResult::$discovery_result_type {
            device_info: Box::new(
                serde_json::from_value::<$device_info_type>($device_info)?.decode()?,
            ),
            handler: $handler_type::new($client.clone()),
        }
    }};
}

impl DiscoveryResult {
    pub(crate) async fn new(
        mut client: ApiClient,
        raw_result: DiscoveryRawResult,
    ) -> Result<Self, Error> {
        let auth_protocol = AuthProtocol::from(&raw_result);
        client
            .login(raw_result.ip.to_string(), auth_protocol)
            .await?;
        let device_info: serde_json::Value = client.get_device_info().await?;
        let client = std::sync::Arc::new(tokio::sync::RwLock::new(client));

        let model = device_info
            .as_object()
            .context("Expected device_info result to be an object")?
            .get_key_value("model")
            .context("Expected device_info to contain the model field")?
            .1
            .as_str()
            .context("Expected device_info model field to have a string value")?;

        let device_type = DeviceType::from_model(model);

        let result = match device_type {
            DeviceType::Light => {
                map_device_model!(
                    Light,
                    DeviceInfoLightResult,
                    LightHandler,
                    device_info,
                    client
                )
            }
            DeviceType::ColorLight => {
                map_device_model!(
                    ColorLight,
                    DeviceInfoColorLightResult,
                    ColorLightHandler,
                    device_info,
                    client
                )
            }
            DeviceType::RgbLightStrip => {
                map_device_model!(
                    RgbLightStrip,
                    DeviceInfoRgbLightStripResult,
                    RgbLightStripHandler,
                    device_info,
                    client
                )
            }
            DeviceType::RgbicLightStrip => {
                map_device_model!(
                    RgbicLightStrip,
                    DeviceInfoRgbicLightStripResult,
                    RgbicLightStripHandler,
                    device_info,
                    client
                )
            }
            DeviceType::Plug => {
                map_device_model!(Plug, DeviceInfoPlugResult, PlugHandler, device_info, client)
            }
            DeviceType::PlugEnergyMonitoring => {
                map_device_model!(
                    PlugEnergyMonitoring,
                    DeviceInfoPlugEnergyMonitoringResult,
                    PlugEnergyMonitoringHandler,
                    device_info,
                    client
                )
            }
            DeviceType::PowerStrip => {
                map_device_model!(
                    PowerStrip,
                    DeviceInfoPowerStripResult,
                    PowerStripHandler,
                    device_info,
                    client
                )
            }
            DeviceType::PowerStripEnergyMonitoring => {
                map_device_model!(
                    PowerStripEnergyMonitoring,
                    DeviceInfoPowerStripResult,
                    PowerStripEnergyMonitoringHandler,
                    device_info,
                    client
                )
            }
            DeviceType::Hub => {
                map_device_model!(Hub, DeviceInfoHubResult, HubHandler, device_info, client)
            }
            DeviceType::Other => DiscoveryResult::Other {
                device_info: Box::new(
                    serde_json::from_value::<DeviceInfoBasicResult>(device_info)?.decode()?,
                ),
                ip: raw_result.ip.to_string(),
            },
        };

        Ok(result)
    }

    /// Returns the [`DeviceType`] category of this discovery result.
    pub fn device_type(&self) -> DeviceType {
        match self {
            DiscoveryResult::Light { .. } => DeviceType::Light,
            DiscoveryResult::ColorLight { .. } => DeviceType::ColorLight,
            DiscoveryResult::RgbLightStrip { .. } => DeviceType::RgbLightStrip,
            DiscoveryResult::RgbicLightStrip { .. } => DeviceType::RgbicLightStrip,
            DiscoveryResult::Plug { .. } => DeviceType::Plug,
            DiscoveryResult::PlugEnergyMonitoring { .. } => DeviceType::PlugEnergyMonitoring,
            DiscoveryResult::PowerStrip { .. } => DeviceType::PowerStrip,
            DiscoveryResult::PowerStripEnergyMonitoring { .. } => {
                DeviceType::PowerStripEnergyMonitoring
            }
            DiscoveryResult::Hub { .. } => DeviceType::Hub,
            DiscoveryResult::Other { .. } => DeviceType::Other,
        }
    }

    /// Returns the model string (e.g. "L530", "P110").
    pub fn model(&self) -> &str {
        match self {
            DiscoveryResult::Light { device_info, .. } => &device_info.model,
            DiscoveryResult::ColorLight { device_info, .. } => &device_info.model,
            DiscoveryResult::RgbLightStrip { device_info, .. } => &device_info.model,
            DiscoveryResult::RgbicLightStrip { device_info, .. } => &device_info.model,
            DiscoveryResult::Plug { device_info, .. } => &device_info.model,
            DiscoveryResult::PlugEnergyMonitoring { device_info, .. } => &device_info.model,
            DiscoveryResult::PowerStrip { device_info, .. } => &device_info.model,
            DiscoveryResult::PowerStripEnergyMonitoring { device_info, .. } => &device_info.model,
            DiscoveryResult::Hub { device_info, .. } => &device_info.model,
            DiscoveryResult::Other { device_info, .. } => &device_info.model,
        }
    }

    /// Returns the IP address of the device.
    pub fn ip(&self) -> &str {
        match self {
            DiscoveryResult::Light { device_info, .. } => &device_info.ip,
            DiscoveryResult::ColorLight { device_info, .. } => &device_info.ip,
            DiscoveryResult::RgbLightStrip { device_info, .. } => &device_info.ip,
            DiscoveryResult::RgbicLightStrip { device_info, .. } => &device_info.ip,
            DiscoveryResult::Plug { device_info, .. } => &device_info.ip,
            DiscoveryResult::PlugEnergyMonitoring { device_info, .. } => &device_info.ip,
            DiscoveryResult::PowerStrip { device_info, .. } => &device_info.ip,
            DiscoveryResult::PowerStripEnergyMonitoring { device_info, .. } => &device_info.ip,
            DiscoveryResult::Hub { device_info, .. } => &device_info.ip,
            DiscoveryResult::Other { ip, .. } => ip,
        }
    }

    /// Returns the device ID.
    pub fn device_id(&self) -> &str {
        match self {
            DiscoveryResult::Light { device_info, .. } => &device_info.device_id,
            DiscoveryResult::ColorLight { device_info, .. } => &device_info.device_id,
            DiscoveryResult::RgbLightStrip { device_info, .. } => &device_info.device_id,
            DiscoveryResult::RgbicLightStrip { device_info, .. } => &device_info.device_id,
            DiscoveryResult::Plug { device_info, .. } => &device_info.device_id,
            DiscoveryResult::PlugEnergyMonitoring { device_info, .. } => &device_info.device_id,
            DiscoveryResult::PowerStrip { device_info, .. } => &device_info.device_id,
            DiscoveryResult::PowerStripEnergyMonitoring { device_info, .. } => {
                &device_info.device_id
            }
            DiscoveryResult::Hub { device_info, .. } => &device_info.device_id,
            DiscoveryResult::Other { device_info, .. } => &device_info.device_id,
        }
    }

    /// Returns the device nickname.
    ///
    /// PowerStrip variants lack a nickname field, so a descriptive literal is returned instead.
    pub fn nickname(&self) -> &str {
        match self {
            DiscoveryResult::Light { device_info, .. } => &device_info.nickname,
            DiscoveryResult::ColorLight { device_info, .. } => &device_info.nickname,
            DiscoveryResult::RgbLightStrip { device_info, .. } => &device_info.nickname,
            DiscoveryResult::RgbicLightStrip { device_info, .. } => &device_info.nickname,
            DiscoveryResult::Plug { device_info, .. } => &device_info.nickname,
            DiscoveryResult::PlugEnergyMonitoring { device_info, .. } => &device_info.nickname,
            DiscoveryResult::PowerStrip { .. } => DeviceType::PowerStrip.as_str(),
            DiscoveryResult::PowerStripEnergyMonitoring { .. } => {
                DeviceType::PowerStripEnergyMonitoring.as_str()
            }
            DiscoveryResult::Hub { device_info, .. } => &device_info.nickname,
            DiscoveryResult::Other { device_info, .. } => device_info
                .nickname
                .as_deref()
                .unwrap_or(DeviceType::Other.as_str()),
        }
    }
}
