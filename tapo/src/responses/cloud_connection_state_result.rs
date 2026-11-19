use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// The cloud connection state as the device sends it.
#[derive(Debug, Deserialize)]
pub(crate) struct CloudConnectionStateResultRaw {
    status: i32,
}

impl TapoResponseExt for CloudConnectionStateResultRaw {}

impl From<CloudConnectionStateResultRaw> for CloudConnectionStateResult {
    fn from(raw: CloudConnectionStateResultRaw) -> Self {
        match raw.status {
            0 => Self::Connected,
            1 => Self::Disconnected,
            2 => Self::Connecting,
            3 => Self::Disconnecting,
            code => Self::Unknown { code },
        }
    }
}

/// The state of the device's connection to the Tapo cloud.
///
/// Only [`Connected`](Self::Connected) means that the device reaches the Tapo
/// cloud. A device that has lost its internet connection keeps trying to
/// reconnect, so it reports [`Connecting`](Self::Connecting) for as long as the
/// connection is down.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloudConnectionStateResult {
    /// The device is connected to the Tapo cloud.
    Connected,
    /// The device is not connected to the Tapo cloud.
    Disconnected,
    /// The device is trying to connect to the Tapo cloud.
    Connecting,
    /// The device is disconnecting from the Tapo cloud.
    Disconnecting,
    /// The device reported a status that the library does not recognize.
    Unknown {
        /// Status code that the device reported.
        code: i32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(status: i32) -> CloudConnectionStateResult {
        let raw: CloudConnectionStateResultRaw =
            serde_json::from_value(serde_json::json!({ "status": status })).unwrap();
        raw.into()
    }

    #[test]
    fn reads_the_known_statuses() {
        assert_eq!(state(0), CloudConnectionStateResult::Connected);
        assert_eq!(state(1), CloudConnectionStateResult::Disconnected);
        assert_eq!(state(2), CloudConnectionStateResult::Connecting);
        assert_eq!(state(3), CloudConnectionStateResult::Disconnecting);
    }

    #[test]
    fn reads_an_unknown_status() {
        assert_eq!(
            state(-1001),
            CloudConnectionStateResult::Unknown { code: -1001 }
        );
    }
}
