#![warn(missing_docs)]

//! Tapo API Client.
//!
//! Tested with light bulbs (L510, L530, L610, L630), light strips (L900, L920, L930),
//! plugs (P100, P105, P110, P115), hubs (H100), switches (S200B) and sensors (T100, T110, T310, T315).
//!
//! # Example with L530
//! ```rust,no_run
//! use std::{env, thread, time::Duration};
//!
//! use tapo::{requests::Color, ApiClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let tapo_username = env::var("TAPO_USERNAME")?;
//!     let tapo_password = env::var("TAPO_PASSWORD")?;
//!     let ip_address = env::var("IP_ADDRESS")?;
//!
//!     let device = ApiClient::new(tapo_username, tapo_password)?
//!         .l530(ip_address)
//!         .await?;
//!
//!     println!("Turning device on...");
//!     device.on().await?;
//!
//!     println!("Setting the brightness to 30%...");
//!     device.set_brightness(30).await?;
//!
//!     println!("Setting the color to `Chocolate`...");
//!     device.set_color(Color::Chocolate).await?;
//!
//!     println!("Waiting 2 seconds...");
//!     thread::sleep(Duration::from_secs(2));
//!
//!     println!("Setting the color to `Deep Sky Blue` using the `hue` and `saturation`...");
//!     device.set_hue_saturation(195, 100).await?;
//!
//!     println!("Waiting 2 seconds...");
//!     thread::sleep(Duration::from_secs(2));
//!
//!     println!("Setting the color to `Incandescent` using the `color temperature`...");
//!     device.set_color_temperature(2700).await?;
//!
//!     println!("Waiting 2 seconds...");
//!     thread::sleep(Duration::from_secs(2));
//!
//!     println!("Using the `set` API to change multiple properties in a single request...");
//!     device
//!         .set()
//!         .brightness(50)
//!         .color(Color::HotPink)
//!         .send()
//!         .await?;
//!
//!     println!("Waiting 2 seconds...");
//!     thread::sleep(Duration::from_secs(2));
//!
//!     println!("Turning device off...");
//!     device.off().await?;
//!
//!     let device_info = device.get_device_info().await?;
//!     println!("Device info: {device_info:?}");
//!
//!     let device_usage = device.get_device_usage().await?;
//!     println!("Device usage: {device_usage:?}");
//!
//!     Ok(())
//! }
//! ```
//!
//! See [more examples](https://github.com/mihai-dinculescu/tapo/tree/main/tapo/examples).

mod api;
mod error;
mod tapo_date_format;

#[cfg(feature = "python")]
pub mod python;

pub mod requests;
pub mod responses;

pub use api::*;
pub use error::*;
