//! Tapo API Client
//! Works with light bulbs (L530, L510, etc.), plugs (P110, P100, etc.), and others.
//!
//! # Example with L530
//! ```rust,no_run
//! use std::{env, thread, time::Duration};
//!
//! use log::{info, LevelFilter};
//! use tapo::{requests::Color, ApiClient, L530};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let log_level = env::var("RUST_LOG")
//!         .unwrap_or_else(|_| "info".to_string())
//!         .parse()
//!         .unwrap_or(LevelFilter::Info);
//!
//!     pretty_env_logger::formatted_timed_builder()
//!         .filter(Some("tapo"), log_level)
//!         .init();
//!
//!     let ip_address = env::var("IP_ADDRESS")?;
//!     let tapo_username = env::var("TAPO_USERNAME")?;
//!     let tapo_password = env::var("TAPO_PASSWORD")?;
//!
//!     let device = ApiClient::<L530>::new(ip_address, tapo_username, tapo_password, true).await?;
//!
//!     info!("Turning device on...");
//!     device.on().await?;
//!
//!     info!("Setting the brightness to 30%...");
//!     device.set_brightness(30).await?;
//!
//!     info!("Setting the color to `Chocolate`...");
//!     device.set_color(Color::Chocolate).await?;
//!
//!     info!("Waiting 2 seconds...");
//!     thread::sleep(Duration::from_secs(2));
//!
//!     info!("Setting the color to `Coral` using the `hue` and `saturation`...");
//!     device.set_hue_saturation(16, 68).await?;
//!
//!     info!("Waiting 2 seconds...");
//!     thread::sleep(Duration::from_secs(2));
//!
//!     info!("Setting the color to `Incandescent` using the `color temperature`...");
//!     device.set_color_temperature(2700).await?;
//!
//!     info!("Waiting 2 seconds...");
//!     thread::sleep(Duration::from_secs(2));
//!
//!     info!("Using the `set` API to change multiple properties in a single request...");
//!     device
//!         .set()
//!         .on()
//!         .brightness(50)?
//!         .color(Color::HotPink)?
//!         .send()
//!         .await?;
//!
//!     info!("Waiting 2 seconds...");
//!     thread::sleep(Duration::from_secs(2));
//!
//!     info!("Turning device off...");
//!     device.off().await?;
//!
//!     let device_info = device.get_device_info().await?;
//!     info!("Device info: {device_info:?}");
//!
//!     let device_usage = device.get_device_usage().await?;
//!     info!("Device usage: {device_usage:?}");
//!
//!     Ok(())
//! }
//! ```
//!
//! See [more examples](https://github.com/mihai-dinculescu/tapo/tree/main/examples).

mod api;
mod devices;
mod encryption;
mod tapo_date_format;

pub mod requests;
pub mod responses;

pub use api::*;
pub use devices::*;
