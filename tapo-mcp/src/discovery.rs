use std::sync::{Arc, RwLock};
use std::time::Duration;

use tokio::time::MissedTickBehavior;
use tracing::Instrument as _;

use crate::config::AppConfig;
use crate::errors::{TapoMcpError, error_message};
use crate::models::DevicesList;
use crate::requests::get_devices;

#[derive(Default)]
enum CacheState {
    /// No discovery run has finished yet.
    #[default]
    Pending,
    /// The last successful run.
    Ready(Arc<DevicesList>),
    /// Every run so far has failed; holds the last error message.
    Failed(String),
}

/// The device list from the most recent successful background discovery.
#[derive(Default)]
pub struct DeviceCache {
    // A std lock is fine: it is never held across an `.await`.
    state: RwLock<CacheState>,
}

impl DeviceCache {
    /// Returns the latest successful discovery, or [`TapoMcpError::DiscoveryUnavailable`]
    /// if there is none yet. Never waits for a discovery run.
    pub(crate) fn get(&self) -> Result<Arc<DevicesList>, TapoMcpError> {
        match &*self.state.read().unwrap_or_else(|e| e.into_inner()) {
            CacheState::Pending => Err(TapoMcpError::DiscoveryUnavailable {
                reason: "the first device discovery since the server started is still running; try again shortly".to_string(),
            }),
            CacheState::Ready(devices) => Ok(Arc::clone(devices)),
            CacheState::Failed(message) => Err(TapoMcpError::DiscoveryUnavailable {
                reason: message.clone(),
            }),
        }
    }

    /// Applies the outcome of one discovery run. A failure never replaces a previous
    /// success, since a stale list beats no list; it is attached to that list as
    /// `refresh_error` instead, until the next success replaces the list.
    fn record(&self, outcome: Result<DevicesList, String>) {
        let mut state = self.state.write().unwrap_or_else(|e| e.into_inner());
        match outcome {
            Ok(devices) => *state = CacheState::Ready(Arc::new(devices)),
            Err(message) => match &mut *state {
                CacheState::Ready(devices) => Arc::make_mut(devices).refresh_error = Some(message),
                _ => *state = CacheState::Failed(message),
            },
        }
    }
}

/// Spawns a task that runs discovery at startup and then every `discovery_interval`
/// seconds, storing each result in `cache`.
pub fn spawn_discovery(config: Arc<AppConfig>, cache: Arc<DeviceCache>) {
    let period = Duration::from_secs(config.discovery_interval);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(period);
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
        loop {
            interval.tick().await;
            let span = tracing::info_span!(
                "refresh_devices",
                devices = tracing::field::Empty,
                errors = tracing::field::Empty,
            );
            async {
                let outcome = run_discovery(Arc::clone(&config)).await;
                match &outcome {
                    Ok(list) => {
                        let span = tracing::Span::current();
                        span.record("devices", list.devices.len());
                        span.record("errors", list.errors.len());
                    }
                    Err(err) => tracing::warn!(%err, "Device discovery failed"),
                }
                cache.record(outcome.map_err(|err| {
                    format!(
                        "last discovery failed: {err}; it is retried every {} seconds",
                        period.as_secs()
                    )
                }));
            }
            .instrument(span)
            .await;
        }
    });
}

/// Runs one discovery in its own task, so a panic fails only this run and not the
/// refresh loop.
async fn run_discovery(config: Arc<AppConfig>) -> Result<DevicesList, String> {
    let run = async move { get_devices(&config).await }.in_current_span();
    match tokio::spawn(run).await {
        Ok(Ok(devices)) => Ok(devices),
        Ok(Err(err)) => Err(error_message(&err)),
        Err(err) => Err(format!("discovery task failed: {err}")),
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    fn devices_list() -> DevicesList {
        DevicesList {
            discovered_at: Utc::now(),
            devices: vec![],
            unsupported: vec![],
            errors: vec![],
            refresh_error: None,
        }
    }

    #[test]
    fn pending_is_unavailable() {
        let cache = DeviceCache::default();

        assert!(matches!(
            cache.get(),
            Err(TapoMcpError::DiscoveryUnavailable { .. })
        ));
    }

    #[test]
    fn failed_is_unavailable() {
        let cache = DeviceCache::default();
        cache.record(Err("boom".to_string()));

        assert!(matches!(
            cache.get(),
            Err(TapoMcpError::DiscoveryUnavailable { reason }) if reason == "boom"
        ));
    }

    #[test]
    fn ready_returns_list() {
        let cache = DeviceCache::default();
        let list = devices_list();
        let discovered_at = list.discovered_at;
        cache.record(Ok(list));

        assert_eq!(cache.get().unwrap().discovered_at, discovered_at);
    }

    #[test]
    fn failure_after_ready_keeps_list() {
        let cache = DeviceCache::default();
        let list = devices_list();
        let discovered_at = list.discovered_at;
        cache.record(Ok(list));
        cache.record(Err("boom".to_string()));

        let devices = cache.get().unwrap();
        assert_eq!(devices.discovered_at, discovered_at);
        assert_eq!(devices.refresh_error.as_deref(), Some("boom"));
    }

    #[test]
    fn success_after_ready_failure_clears_refresh_error() {
        let cache = DeviceCache::default();
        cache.record(Ok(devices_list()));
        cache.record(Err("boom".to_string()));
        cache.record(Ok(devices_list()));

        assert!(cache.get().unwrap().refresh_error.is_none());
    }

    #[test]
    fn success_after_failure_returns_list() {
        let cache = DeviceCache::default();
        cache.record(Err("boom".to_string()));
        cache.record(Ok(devices_list()));

        assert!(cache.get().is_ok());
    }
}
