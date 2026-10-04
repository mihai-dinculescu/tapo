use serde::Serialize;

use crate::requests::SectionNames;

/// `module → section names` envelope for `getDeviceInfo`:
/// `{"device_info": {"name": ["basic_info"]}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamGetDeviceInfoParams {
    device_info: SectionNames,
}

impl SmartCamGetDeviceInfoParams {
    pub fn new() -> Self {
        Self {
            device_info: SectionNames::new(&["basic_info"]),
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
        // What a C220 takes inside a `multipleRequest`.
        let request =
            TapoRequest::SmartCamGetDeviceInfo(TapoParams::new(SmartCamGetDeviceInfoParams::new()));

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({
                "method": "getDeviceInfo",
                "params": { "device_info": { "name": ["basic_info"] } },
            })
        );
    }
}
