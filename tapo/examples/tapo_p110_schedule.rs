/// P110, P110M and P115 Schedule Example
use log::info;
use tapo::ApiClient;
use tapo::requests::{DaysOfWeek, ScheduleRule};
use tapo::responses::PowerState;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let [tapo_username, tapo_password, ip_address] =
        common::require_env_vars(["TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"])?;

    let device = ApiClient::new(tapo_username, tapo_password)
        .p110(ip_address)
        .await?;

    // Schedule rules fire on the device's own clock, so this example adds,
    // edits and removes rules instead of waiting for one of them to fire.
    // Rules that are already on the device are left alone; only the four
    // rules added below are removed again at the end.
    let rules = device.get_schedule_rules().await?;
    info!("The device starts with {} schedule rules.", rules.len());

    info!("Adding a rule that turns the device on once, at the next 06:30...");
    let rule = ScheduleRule::clock_once(6, 30, PowerState::On)?;
    let morning = device.add_schedule_rule(rule).await?;
    info!("Added rule: {morning:?}");

    info!("Adding a rule that turns the device off at 23:30 on Mondays and Wednesdays...");
    let rule =
        ScheduleRule::clock_weekly(23, 30, DaysOfWeek::MON | DaysOfWeek::WED, PowerState::Off)?;
    let late_night = device.add_schedule_rule(rule).await?;
    info!("Added rule: {late_night:?}");

    info!("Adding a rule that turns the device on every day, an hour after sunset...");
    let rule = ScheduleRule::sunset_weekly(60, DaysOfWeek::EVERY_DAY, PowerState::On)?;
    let after_sunset = device.add_schedule_rule(rule).await?;
    info!("Added rule: {after_sunset:?}");

    // A negative offset fires before the astronomical event instead of after it.
    info!("Adding a rule that turns the device off on weekdays, 30 minutes before sunrise...");
    let rule = ScheduleRule::sunrise_weekly(-30, DaysOfWeek::WEEKDAYS, PowerState::Off)?;
    let before_sunrise = device.add_schedule_rule(rule).await?;
    info!("Added rule: {before_sunrise:?}");

    let rules = device.get_schedule_rules().await?;
    info!(
        "The four rules should have been added: the device now holds {} rules.",
        rules.len()
    );
    for rule in &rules {
        info!("Rule: {rule:?}");
    }

    // A disabled rule stays on the device, but does not fire.
    info!("Disabling the 23:30 rule...");
    device
        .edit_schedule_rule(late_night.with_enabled(false))
        .await?;
    let rules = device.get_schedule_rules().await?;
    let late_night_after_edit = rules.iter().find(|rule| rule.id == late_night.id);
    info!("The 23:30 rule should now be disabled: {late_night_after_edit:?}");

    info!("Removing the four rules that were added...");
    for rule in [&morning, &late_night, &after_sunset, &before_sunrise] {
        if let Some(id) = &rule.id {
            device.remove_schedule_rule(id.clone()).await?;
        }
    }

    let rules = device.get_schedule_rules().await?;
    info!(
        "The added rules should be gone: the device holds {} rules again.",
        rules.len()
    );

    Ok(())
}
