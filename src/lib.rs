//! Tapo API Client
//! Works with light bulbs (L530, L510, etc.), plugs (P110, P100, etc.), and others.
//!
//! # Example with L530
//! ```rust,ignore
//! let device = ApiClient::<L530>::new(ip_address, tapo_username, tapo_password, true).await?;
//!
//! info!("Turning device on...");
//! device.on().await?;
//!
//! info!("Setting the brightness to 30%...");
//! device.set_brightness(30).await?;
//!
//! info!("Setting the color to `Chocolate`...");
//! device.set_color(Color::Chocolate).await?;
//!
//! info!("Waiting 2 seconds...");
//! thread::sleep(Duration::from_secs(2));
//!
//! info!("Setting the color to `Coral` using the `hue` and `saturation`...");
//! device.set_hue_saturation(16, 68).await?;
//!
//! info!("Waiting 2 seconds...");
//! thread::sleep(Duration::from_secs(2));
//!
//! info!("Setting the color to `Incandescent` using the `color temperature`...");
//! device.set_color_temperature(2700).await?;
//!
//! info!("Waiting 2 seconds...");
//! thread::sleep(Duration::from_secs(2));
//!
//! let device_info = device.get_device_info().await?;
//! info!("Device info: {device_info:?}");
//!
//! let device_usage = device.get_device_usage().await?;
//! info!("Device usage: {device_usage:?}");
//!
//! info!("Turning device off...");
//! device.off().await?;
//! ```
//!
//! See [more examples](https://github.com/mihai-dinculescu/tapo/tree/main/examples).

mod api;
mod devices;
mod encryption;
mod requests;
mod responses;
mod tapo_date_format;

pub use api::*;
pub use devices::{GenericDevice, TapoDeviceExt, L510, L530, P100, P110};
pub use requests::Color;
pub use responses::GenericDeviceInfoResult;
pub use responses::{
    DefaultState, DeviceInfoResultExt, DeviceUsageResult, EnergyUsageResult, L510DeviceInfoResult,
    L510State, L510StateWrapper, L530DeviceInfoResult, L530State, L530StateWrapper,
    PlugDeviceInfoResult, PlugState, PlugStateWrapper, TapoResponseExt, UsageByPeriodResult,
};
