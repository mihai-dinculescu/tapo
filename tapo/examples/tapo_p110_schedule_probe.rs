//! TEMPORARY probe — not part of the Schedule API contribution.
//!
//! Measures the device-side limits the schedule implementation currently
//! guesses at, so they can be replaced with verified numbers. Delete this
//! file (and its `SUPPORTED_DEVICES.md` / changelog absence) before the PR
//! is finalised.
//!
//! Everything it learns is printed as a `FINDING:` line. It restores the
//! device to its starting set of rules on the way out, including when a
//! probe fails, but read the summary before trusting that — if it dies
//! mid-sweep, run `remove_all_schedule_rules` manually.
//!
//! Run with:
//!   TAPO_USERNAME=... TAPO_PASSWORD=... IP_ADDRESS=... \
//!     cargo run -p tapo --example tapo_p110_schedule_probe
use std::time::Duration;

use log::{info, warn};
use tapo::requests::{DaysOfWeek, ScheduleRule};
use tapo::responses::PowerState;
use tapo::{ApiClient, PlugEnergyMonitoringHandler};

mod common;

/// Hard stop for the rule-count sweep, so a device that never refuses
/// cannot loop forever.
const RULE_COUNT_CEILING: usize = 64;

/// Offsets to probe for the sunrise / sunset bound. `1440` (±24h) is what
/// the builders currently allow; the Tapo app itself offers far less.
const OFFSET_CANDIDATES: [i16; 8] = [60, 120, 300, 600, 720, 1080, 1440, 1441];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let [tapo_username, tapo_password, ip_address] =
        common::require_env_vars(["TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"])?;

    let device = ApiClient::new(tapo_username, tapo_password)
        .p110(ip_address)
        .await?;

    let pre_existing = device.get_schedule_rules().await?;
    info!(
        "The device holds {} schedule rules before probing.",
        pre_existing.len()
    );
    if !pre_existing.is_empty() {
        warn!(
            "This probe adds and removes many rules. {} rule(s) already exist and will be left \
             alone, but consider running against a plug with an empty schedule.",
            pre_existing.len()
        );
    }
    let baseline = pre_existing.len();

    probe_offset_bound(&device).await;
    probe_rule_count_limit(&device, baseline).await;
    probe_pagination(&device, baseline).await;
    probe_edit_of_missing_id(&device).await;
    probe_duplicate_add(&device).await;

    let remaining = device.get_schedule_rules().await?;
    if remaining.len() == baseline {
        info!("Cleanup complete: back to the original {baseline} rules.");
    } else {
        warn!(
            "Cleanup INCOMPLETE: {} rules remain, expected {baseline}. Remove the leftovers \
             before using the plug normally.",
            remaining.len()
        );
    }

    Ok(())
}

/// Adds a rule and immediately removes it, reporting whether the device
/// accepted it. Errors are returned rather than propagated so a rejection
/// is data, not a crash.
async fn try_rule(
    device: &PlugEnergyMonitoringHandler,
    rule: ScheduleRule,
) -> Result<ScheduleRule, String> {
    match device.add_schedule_rule(rule).await {
        Ok(added) => {
            if let Some(id) = added.id.clone()
                && let Err(e) = device.remove_schedule_rule(id).await
            {
                warn!("Failed to clean up a probe rule: {e}");
            }
            Ok(added)
        }
        Err(e) => Err(e.to_string()),
    }
}

/// How large a sunrise / sunset offset will the device actually store?
async fn probe_offset_bound(device: &PlugEnergyMonitoringHandler) {
    info!("--- Probing the sunrise / sunset offset bound ---");

    let mut largest_accepted: Option<i16> = None;
    let mut smallest_rejected: Option<i16> = None;

    for offset in OFFSET_CANDIDATES {
        // Skip what our own validation refuses to build; that bound is the
        // thing under test, so report it separately.
        let rule = match ScheduleRule::sunset_once(offset, PowerState::On) {
            Ok(rule) => rule,
            Err(e) => {
                info!("offset {offset:+} rejected locally by the builder: {e}");
                continue;
            }
        };

        match try_rule(device, rule).await {
            Ok(added) => {
                info!("offset {offset:+} accepted, stored as {:?}", added.time);
                largest_accepted = Some(offset);
            }
            Err(e) => {
                info!("offset {offset:+} REJECTED by the device: {e}");
                smallest_rejected = smallest_rejected.or(Some(offset));
            }
        }

        // Negative offsets travel a different code path on some firmwares.
        if let Ok(rule) = ScheduleRule::sunrise_once(-offset, PowerState::Off) {
            match try_rule(device, rule).await {
                Ok(_) => info!("offset {:+} accepted", -offset),
                Err(e) => info!("offset {:+} REJECTED by the device: {e}", -offset),
            }
        }
    }

    info!(
        "FINDING: largest accepted offset {:?}, smallest rejected offset {:?} \
         (builders currently allow ±1440)",
        largest_accepted, smallest_rejected
    );
}

/// How many schedule rules will the device hold before it refuses?
async fn probe_rule_count_limit(device: &PlugEnergyMonitoringHandler, baseline: usize) {
    info!("--- Probing the maximum rule count ---");

    let mut added_ids = Vec::new();
    let mut refusal = None;

    for index in 0..RULE_COUNT_CEILING {
        // Distinct minute-of-day per rule, so nothing is rejected merely for
        // colliding with an existing rule.
        let hour = (index / 60) as u8;
        let minute = (index % 60) as u8;
        let rule = match ScheduleRule::clock_weekly(hour, minute, DaysOfWeek::MON, PowerState::On) {
            Ok(rule) => rule,
            Err(e) => {
                warn!("Ran out of distinct times to probe with at index {index}: {e}");
                break;
            }
        };

        match device.add_schedule_rule(rule).await {
            Ok(added) => {
                if let Some(id) = added.id {
                    added_ids.push(id);
                }
            }
            Err(e) => {
                refusal = Some((baseline + added_ids.len(), e.to_string()));
                break;
            }
        }
    }

    match &refusal {
        Some((count, e)) => info!(
            "FINDING: the device refused rule number {} (so it holds {count}): {e}",
            count + 1
        ),
        None => info!(
            "FINDING: no refusal after {} added rules (ceiling {RULE_COUNT_CEILING} reached); \
             the real limit is higher than this probe went",
            added_ids.len()
        ),
    }

    // Also worth knowing: does a full device report its count honestly?
    match device.get_schedule_rules().await {
        Ok(rules) => info!(
            "FINDING: get_schedule_rules read back {} rules while the device was full",
            rules.len()
        ),
        Err(e) => warn!("get_schedule_rules failed while the device was full: {e}"),
    }

    info!("Removing the {} probe rules...", added_ids.len());
    for id in added_ids {
        if let Err(e) = device.remove_schedule_rule(id).await {
            warn!("Failed to remove a probe rule: {e}");
        }
    }
}

/// Does `get_schedule_rules` actually paginate, and is the `sum` field
/// trustworthy? This is what the `MAX_PAGES` guard in the client is
/// guessing about.
async fn probe_pagination(device: &PlugEnergyMonitoringHandler, baseline: usize) {
    info!("--- Probing list pagination ---");

    // Enough rules to exceed any plausible page size without filling the
    // device, so a single-page response tells us the page size is large.
    const PROBE_RULES: usize = 12;

    let mut added_ids = Vec::new();
    for index in 0..PROBE_RULES {
        let rule = ScheduleRule::clock_weekly(1, index as u8, DaysOfWeek::TUE, PowerState::On)
            .expect("valid probe rule");
        match device.add_schedule_rule(rule).await {
            Ok(added) => {
                if let Some(id) = added.id {
                    added_ids.push(id);
                }
            }
            Err(e) => {
                warn!("Could not add pagination probe rule {index}: {e}");
                break;
            }
        }
    }

    match device.get_schedule_rules().await {
        Ok(rules) => {
            let expected = baseline + added_ids.len();
            info!(
                "FINDING: with {expected} rules on the device, get_schedule_rules returned {}{}",
                rules.len(),
                if rules.len() == expected {
                    " (complete — paging logic held)"
                } else {
                    " (MISMATCH — paging is wrong or the device truncated)"
                }
            );
        }
        Err(e) => warn!("get_schedule_rules failed during the pagination probe: {e}"),
    }

    info!(
        "NOTE: the per-page size and the `sum` field are not visible through the public API. \
         Re-run with RUST_LOG=trace to read them off the raw responses."
    );

    for id in added_ids {
        if let Err(e) = device.remove_schedule_rule(id).await {
            warn!("Failed to remove a pagination probe rule: {e}");
        }
    }
}

/// What does the device do with an edit for an id that does not exist?
/// The client only checks that an id is *present*.
async fn probe_edit_of_missing_id(device: &PlugEnergyMonitoringHandler) {
    info!("--- Probing an edit against a non-existent id ---");

    let rule = ScheduleRule::clock_once(3, 15, PowerState::On)
        .expect("valid probe rule")
        .with_id("does-not-exist");

    match device.edit_schedule_rule(rule).await {
        Ok(()) => {
            info!("FINDING: editing an unknown id SUCCEEDED silently — the client cannot detect it")
        }
        Err(e) => info!("FINDING: editing an unknown id returned an error: {e}"),
    }
}

/// Does the device deduplicate two identical rules, or store both?
async fn probe_duplicate_add(device: &PlugEnergyMonitoringHandler) {
    info!("--- Probing a duplicate rule ---");

    let build = || {
        ScheduleRule::clock_weekly(4, 45, DaysOfWeek::WED, PowerState::Off)
            .expect("valid probe rule")
    };

    let first = match device.add_schedule_rule(build()).await {
        Ok(added) => added,
        Err(e) => {
            warn!("Could not add the first duplicate probe rule: {e}");
            return;
        }
    };

    match device.add_schedule_rule(build()).await {
        Ok(second) => {
            info!(
                "FINDING: a duplicate rule was accepted — ids {:?} and {:?}",
                first.id, second.id
            );
            if let Some(id) = second.id
                && let Err(e) = device.remove_schedule_rule(id).await
            {
                warn!("Failed to remove the duplicate probe rule: {e}");
            }
        }
        Err(e) => info!("FINDING: a duplicate rule was REJECTED: {e}"),
    }

    if let Some(id) = first.id
        && let Err(e) = device.remove_schedule_rule(id).await
    {
        warn!("Failed to remove the first duplicate probe rule: {e}");
    }

    // Give the device a moment before the caller reads the final rule list.
    tokio::time::sleep(Duration::from_millis(500)).await;
}
