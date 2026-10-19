use serde::Serialize;

use crate::requests::EmptyObjectParams;

/// `module → section → data` envelope for `rebootDevice`:
/// `{"system": {"reboot": {}}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamRebootDeviceParams {
    system: RebootSystemParams,
}

#[derive(Debug, Serialize)]
struct RebootSystemParams {
    reboot: EmptyObjectParams,
}

impl SmartCamRebootDeviceParams {
    pub fn new() -> Self {
        Self {
            system: RebootSystemParams {
                reboot: EmptyObjectParams {},
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::requests::{TapoParams, TapoRequest};

    use super::*;

    #[test]
    fn request_names_its_method() {
        let request =
            TapoRequest::SmartCamRebootDevice(TapoParams::new(SmartCamRebootDeviceParams::new()));

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({
                "method": "rebootDevice",
                "params": { "system": { "reboot": {} } },
            })
        );
    }
}
