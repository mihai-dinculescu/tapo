/// L930 Example
use std::{env, thread, time::Duration};

use log::{info, LevelFilter};
use tapo::{
    requests::{Color, LightingEffect, LightingEffectPreset, LightingEffectType},
    ApiClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_level = env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .parse()
        .unwrap_or(LevelFilter::Info);

    pretty_env_logger::formatted_timed_builder()
        .filter(Some("tapo"), log_level)
        .init();

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let ip_address = env::var("IP_ADDRESS")?;

    let device = ApiClient::new(tapo_username, tapo_password)?
        .l930(ip_address)
        .await?;

    info!("Turning device on...");
    device.on().await?;

    info!("Setting the brightness to 30%...");
    device.set_brightness(30).await?;

    info!("Setting the color to `Chocolate`...");
    device.set_color(Color::Chocolate).await?;

    info!("Waiting 2 seconds...");
    thread::sleep(Duration::from_secs(2));

    info!("Setting the color to `Deep Sky Blue` using the `hue` and `saturation`...");
    device.set_hue_saturation(195, 100).await?;

    info!("Waiting 2 seconds...");
    thread::sleep(Duration::from_secs(2));

    info!("Setting the color to `Incandescent` using the `color temperature`...");
    device.set_color_temperature(2700).await?;

    info!("Waiting 2 seconds...");
    thread::sleep(Duration::from_secs(2));

    info!("Using the `set` API to set multiple properties in a single request...");
    device
        .set()
        .brightness(50)
        .color(Color::HotPink)
        .send()
        .await?;

    info!("Waiting 2 seconds...");
    thread::sleep(Duration::from_secs(2));

    info!("Setting a preset Lighting effect...");
    device
        .set_lighting_effect(LightingEffectPreset::BubblingCauldron)
        .await?;

    info!("Waiting 10 seconds...");
    thread::sleep(Duration::from_secs(10));

    info!("Setting a custom static Lighting effect...");
    let custom_effect = LightingEffect::new_with_random_id(
        "My Custom Static Effect",
        LightingEffectType::Static,
        true,
        true,
        100,
        vec![[359, 85, 100]],
    )
    .with_expansion_strategy(1)
    .with_segments(vec![0, 1, 2])
    .with_sequence(vec![[359, 85, 100], [0, 0, 100], [236, 72, 100]]);

    device.set_lighting_effect(custom_effect).await?;

    info!("Waiting 10 seconds...");
    thread::sleep(Duration::from_secs(10));

    info!("Setting a custom sequence Lighting effect...");
    let custom_effect = LightingEffect::new_with_random_id(
        "My Custom Sequence Effect",
        LightingEffectType::Sequence,
        true,
        true,
        100,
        vec![[359, 85, 100]],
    )
    .with_expansion_strategy(1)
    .with_segments(vec![0, 1, 2])
    .with_sequence(vec![[359, 85, 100], [0, 0, 100], [236, 72, 100]])
    .with_direction(1)
    .with_duration(50);

    device.set_lighting_effect(custom_effect).await?;

    info!("Waiting 10 seconds...");
    thread::sleep(Duration::from_secs(10));

    info!("Turning device off...");
    device.off().await?;

    let device_info = device.get_device_info().await?;
    info!("Device info: {device_info:?}");

    let device_usage = device.get_device_usage().await?;
    info!("Device usage: {device_usage:?}");

    Ok(())
}
