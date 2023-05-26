use serde::Deserialize;

use super::TapoResponseExt;

/// Trigger logs result.
#[derive(Debug, Deserialize)]
pub struct TriggerLogsResult<T> {
    /// The `id` of the most recent log item that is returned.
    pub start_id: u64,
    /// The total number of log items that the hub holds for this device.
    pub sum: u64,
    /// Log items in reverse chronological order (newest first).
    pub logs: Vec<T>,
}

impl<T> TapoResponseExt for TriggerLogsResult<T> {}
