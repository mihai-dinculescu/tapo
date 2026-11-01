use serde::{Deserialize, Serialize};

use super::TapoResponseExt;

/// Whether the device is connected to the Tapo Cloud.
///
/// The cloud link is what the device uses for remote control, so this is the
/// connection state the API exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "i64", into = "i64")]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, eq, hash, frozen)
)]
pub enum CloudConnectionState {
    /// Connected to the Tapo Cloud.
    Connected,
    /// Disconnected from the Tapo Cloud.
    Disconnected,
    /// Connecting to the Tapo Cloud.
    Connecting,
    /// Disconnecting from the Tapo Cloud.
    Disconnecting,
    /// A state code this library does not know.
    Unknown,
}

impl From<i64> for CloudConnectionState {
    fn from(code: i64) -> Self {
        match code {
            0 => Self::Connected,
            1 => Self::Disconnected,
            2 => Self::Connecting,
            3 => Self::Disconnecting,
            _ => Self::Unknown,
        }
    }
}

impl From<CloudConnectionState> for i64 {
    fn from(state: CloudConnectionState) -> Self {
        match state {
            CloudConnectionState::Connected => 0,
            CloudConnectionState::Disconnected => 1,
            CloudConnectionState::Connecting => 2,
            CloudConnectionState::Disconnecting => 3,
            // No wire code exists for an unrecognised state; -1 is outside 0-3,
            // so it reads back as `Unknown`.
            CloudConnectionState::Unknown => -1,
        }
    }
}

/// The device's cloud connection state.
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ConnectCloudStateResult {
    pub status: CloudConnectionState,
}

impl TapoResponseExt for ConnectCloudStateResult {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requests::{EmptyParams, TapoParams, TapoRequest};

    #[test]
    fn cloud_state_request_uses_the_hub_method_name() {
        let request = TapoRequest::GetConnectCloudState(TapoParams::new(EmptyParams));

        let json = serde_json::to_value(request).unwrap();

        assert_eq!(json["method"], "get_connect_cloud_state");
        assert!(json["params"].is_null());
    }

    #[test]
    fn cloud_state_parses_the_status_object() {
        for (code, state) in [
            (0, CloudConnectionState::Connected),
            (1, CloudConnectionState::Disconnected),
            (2, CloudConnectionState::Connecting),
            (3, CloudConnectionState::Disconnecting),
        ] {
            let json = format!(r#"{{"status": {code}}}"#);

            let parsed: ConnectCloudStateResult = serde_json::from_str(&json).unwrap();

            assert_eq!(parsed.status, state);
        }
    }

    #[test]
    fn cloud_state_reports_unknown_codes() {
        for code in [4, 7, -1001] {
            let json = format!(r#"{{"status": {code}}}"#);

            let parsed: ConnectCloudStateResult = serde_json::from_str(&json).unwrap();

            assert_eq!(parsed.status, CloudConnectionState::Unknown);
        }
    }

    #[test]
    fn cloud_state_missing_status_is_an_error() {
        assert!(serde_json::from_str::<ConnectCloudStateResult>("{}").is_err());
    }

    #[test]
    fn cloud_state_survives_a_json_round_trip() {
        for (code, status) in [
            (0, CloudConnectionState::Connected),
            (1, CloudConnectionState::Disconnected),
            (2, CloudConnectionState::Connecting),
            (3, CloudConnectionState::Disconnecting),
            (-1, CloudConnectionState::Unknown),
        ] {
            let json = serde_json::to_value(ConnectCloudStateResult { status }).unwrap();

            assert_eq!(json["status"], code);
            assert_eq!(
                serde_json::from_value::<ConnectCloudStateResult>(json)
                    .unwrap()
                    .status,
                status
            );
        }
    }
}
