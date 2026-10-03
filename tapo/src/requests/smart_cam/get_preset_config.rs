use serde::Serialize;

use crate::requests::SectionNames;

/// `module → section names` envelope for `getPresetConfig`:
/// `{"preset": {"name": ["preset"]}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamGetPresetConfigParams {
    preset: SectionNames,
}

impl SmartCamGetPresetConfigParams {
    pub fn new() -> Self {
        Self {
            preset: SectionNames::new(&["preset"]),
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
        let request = TapoRequest::SmartCamGetPresetConfig(TapoParams::new(
            SmartCamGetPresetConfigParams::new(),
        ));

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({
                "method": "getPresetConfig",
                "params": { "preset": { "name": ["preset"] } },
            })
        );
    }
}
