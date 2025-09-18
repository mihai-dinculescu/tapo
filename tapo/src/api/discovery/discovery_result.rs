use std::net::IpAddr;

use anyhow::Context;

use crate::responses::{
    DecodableResultExt, DeviceInfoColorLightResult, DeviceInfoGenericResult, DeviceInfoHubResult,
    DeviceInfoLightResult, DeviceInfoPlugEnergyMonitoringResult, DeviceInfoPlugResult,
    DeviceInfoPowerStripResult, DeviceInfoRgbLightStripResult, DeviceInfoRgbicLightStripResult,
};
use crate::{
    ApiClient, ColorLightHandler, Error, GenericDeviceHandler, HubHandler, LightHandler,
    PlugEnergyMonitoringHandler, PlugHandler, PowerStripEnergyMonitoringHandler, PowerStripHandler,
    RgbLightStripHandler, RgbicLightStripHandler,
};

#[derive(Debug)]
/// Result of the device discovery process.
pub enum DiscoveryResult {
    /// A Generic Tapo device.
    ///
    /// If you believe this device is already supported, or would like to explore adding support for a currently
    /// unsupported model, please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues)
    /// to start the discussion.
    GenericDevice {
        /// Device info of a Generic Tapo device.
        ///
        /// If you believe this device is already supported, or would like to explore adding support for a currently
        /// unsupported model, please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues)
        /// to start the discussion.
        device_info: Box<DeviceInfoGenericResult>,
        /// Handler for generic devices. It provides the functionality common to all Tapo [devices](https://www.tapo.com/en/).
        ///
        /// If you believe this device is already supported, or would like to explore adding support for a currently
        /// unsupported model, please [open an issue on GitHub](https://github.com/mihai-dinculescu/tapo/issues)
        /// to start the discussion.
        handler: GenericDeviceHandler,
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
    ($discovery_result_type:ident, $device_info_type:ident, $device_info:expr, $handler:expr) => {{
        DiscoveryResult::$discovery_result_type {
            device_info: Box::new(
                serde_json::from_value::<$device_info_type>($device_info)?.decode()?,
            ),
            handler: $handler.into(),
        }
    }};
}

impl DiscoveryResult {
    pub(crate) async fn new(client: ApiClient, ip_addr: IpAddr) -> Result<Self, Error> {
        let handler = client.generic_device(ip_addr.to_string()).await?;

        let device_info = handler.get_device_info_json().await?;

        let model = device_info
            .as_object()
            .context("Expected device_info result to be an object")?
            .get_key_value("model")
            .context("Expected device_info to contain the model field")?
            .1
            .as_str()
            .context("Expected device_info model field to have a string value")?;

        let result = match model {
            "L510" | "L520" | "L610" => {
                map_device_model!(Light, DeviceInfoLightResult, device_info, handler)
            }
            "L530" | "L530 Series" | "L535" | "L535B" | "L630" => {
                map_device_model!(ColorLight, DeviceInfoColorLightResult, device_info, handler)
            }
            "L900" => {
                map_device_model!(
                    RgbLightStrip,
                    DeviceInfoRgbLightStripResult,
                    device_info,
                    handler
                )
            }
            "L920" | "L930" => {
                map_device_model!(
                    RgbicLightStrip,
                    DeviceInfoRgbicLightStripResult,
                    device_info,
                    handler
                )
            }
            "P100" | "P105" => {
                map_device_model!(Plug, DeviceInfoPlugResult, device_info, handler)
            }
            "P110" | "P110M" | "P115" => {
                map_device_model!(
                    PlugEnergyMonitoring,
                    DeviceInfoPlugEnergyMonitoringResult,
                    device_info,
                    handler
                )
            }
            "P300" | "P306" => {
                map_device_model!(PowerStrip, DeviceInfoPowerStripResult, device_info, handler)
            }
            "P304M" | "P316M" => {
                map_device_model!(
                    PowerStripEnergyMonitoring,
                    DeviceInfoPowerStripResult,
                    device_info,
                    handler
                )
            }
            "H100" => {
                map_device_model!(Hub, DeviceInfoHubResult, device_info, handler)
            }
            _ => {
                map_device_model!(GenericDevice, DeviceInfoGenericResult, device_info, handler)
            }
        };

        Ok(result)
    }
}
