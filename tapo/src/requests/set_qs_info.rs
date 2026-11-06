use serde::Serialize;

use crate::responses::{WifiSecurity, encode_value};

/// The details that a device in setup mode joins a Wi-Fi network with. The
/// device takes `account` as its local login from then on, and does not
/// take the request at all without it.
#[derive(Serialize)]
pub(crate) struct SetQsInfoParams {
    account: QsAccount,
    wireless: QsWireless,
    time: QsTime,
}

#[derive(Serialize)]
struct QsAccount {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct QsWireless {
    ssid: String,
    password: String,
    key_type: WifiSecurity,
}

/// The device's clock, which reads the year 2000 while it is in setup mode.
#[derive(Serialize)]
struct QsTime {
    timestamp: i64,
    /// The standard offset of `region` in minutes, without daylight saving.
    time_diff: i64,
    region: String,
}

impl SetQsInfoParams {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        username: &str,
        password: &str,
        ssid: &str,
        wifi_password: &str,
        security: WifiSecurity,
        timestamp: i64,
        time_diff: i64,
        region: &str,
    ) -> Self {
        Self {
            account: QsAccount {
                username: encode_value(username),
                password: encode_value(password),
            },
            wireless: QsWireless {
                ssid: encode_value(ssid),
                password: encode_value(wifi_password),
                key_type: security,
            },
            time: QsTime {
                timestamp,
                time_diff,
                region: region.to_string(),
            },
        }
    }
}

/// Leaves the passwords out.
impl std::fmt::Debug for SetQsInfoParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SetQsInfoParams").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::requests::{TapoParams, TapoRequest};

    use super::*;

    fn params() -> SetQsInfoParams {
        SetQsInfoParams::new(
            "me@example.com",
            "local-password",
            "Simulated Wi-Fi",
            "wifi-password",
            WifiSecurity::Wpa2Psk,
            1793652834,
            0,
            "Europe/London",
        )
    }

    #[test]
    fn the_details_are_sent_base64_encoded() {
        assert_eq!(
            serde_json::to_value(params()).unwrap(),
            json!({
                "account": {"username": "bWVAZXhhbXBsZS5jb20=", "password": "bG9jYWwtcGFzc3dvcmQ="},
                "wireless": {"ssid": "U2ltdWxhdGVkIFdpLUZp", "password": "d2lmaS1wYXNzd29yZA==", "key_type": "wpa2_psk"},
                "time": {"timestamp": 1793652834, "time_diff": 0, "region": "Europe/London"},
            })
        );
    }

    #[test]
    fn the_passwords_stay_out_of_the_logs() {
        let request = TapoRequest::SetQsInfo(Box::new(TapoParams::new(params())));
        let request_string = serde_json::to_string(&request).unwrap();

        let logged = request.for_log(&request_string);

        assert!(!logged.contains("bG9jYWwtcGFzc3dvcmQ="));
        assert!(!logged.contains("d2lmaS1wYXNzd29yZA=="));
    }
}
