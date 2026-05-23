/// Demo: schedule rules on a P110 (the "Schedule" feature in the Tapo app).
///
/// Adds a representative sample of rules — one-shot, weekly, sunset,
/// and sunrise — reads one of them back, then deletes them.
/// Existing rules already on the device are left alone.
///
/// Build / run:
///   cargo run --example tapo_p110_schedule
///
/// Environment variables: TAPO_USERNAME, TAPO_PASSWORD, IP_ADDRESS.
use std::env;

use log::info;
use tapo::ApiClient;
use tapo::requests::{ScheduleRule, week_day};

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let ip_address = env::var("IP_ADDRESS")?;

    let device = ApiClient::new(tapo_username, tapo_password)
        .p110(ip_address)
        .await?;

    let preexisting: Vec<String> = device
        .get_schedule_rules()
        .await?
        .into_iter()
        .filter_map(|r| r.id)
        .collect();
    info!(
        "Pre-existing rules on the device: {} (left alone)",
        preexisting.len()
    );

    info!("Adding four demo rules...");
    let added = [
        // Turn on once, the next time the clock hits 06:30.
        device
            .add_schedule_rule(ScheduleRule::clock_once(6, 30, true)?)
            .await?,
        // Turn off weekly at 23:30 on Mondays and Wednesdays.
        device
            .add_schedule_rule(ScheduleRule::clock_weekly(
                23,
                30,
                week_day::MON | week_day::WED,
                false,
            )?)
            .await?,
        // Turn on every day, one hour after sunset.
        device
            .add_schedule_rule(ScheduleRule::sunset_weekly(60, week_day::EVERY_DAY, true)?)
            .await?,
        // Turn off on weekdays (Mon–Fri), 30 minutes before sunrise.
        device
            .add_schedule_rule(ScheduleRule::sunrise_weekly(
                -30,
                week_day::WEEKDAYS,
                false,
            )?)
            .await?,
    ];
    let added_ids: Vec<String> = added.iter().filter_map(|r| r.id.clone()).collect();
    info!("  added ids: {added_ids:?}");

    // Read one of them (the sunset rule, index 2) back and dump it.
    let sunset_id = added[2].id.clone().expect("device returned an id");
    let rules = device.get_schedule_rules().await?;
    let sunset = rules
        .iter()
        .find(|r| r.id.as_deref() == Some(sunset_id.as_str()))
        .expect("sunset rule we just added must come back");
    info!(
        "Read back sunset rule: id={:?} time_kind={:?} freq={:?} \
         offset_minutes={} week_day={:#09b} turn_on={}",
        sunset.id,
        sunset.time_kind,
        sunset.frequency,
        sunset.offset_minutes,
        sunset.week_day,
        sunset.turn_on,
    );

    info!("Cleaning up: removing the four demo rules.");
    for id in &added_ids {
        device.remove_schedule_rule(id.clone()).await?;
    }

    let remaining_ids: Vec<String> = device
        .get_schedule_rules()
        .await?
        .into_iter()
        .filter_map(|r| r.id)
        .collect();
    for id in &added_ids {
        assert!(
            !remaining_ids.contains(id),
            "demo rule {id} should be gone but is still on the device",
        );
    }
    info!(
        "Cleanup OK — {} pre-existing rule(s) left intact: {remaining_ids:?}",
        remaining_ids.len(),
    );

    info!("PASS");
    Ok(())
}
