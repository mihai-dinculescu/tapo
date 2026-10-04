use serde::Serialize;

/// `module → section → data` envelope for `deletePreset`:
/// `{"preset": {"remove_preset": {"id": ["1"]}}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamDeletePresetParams {
    preset: PresetParams,
}

#[derive(Debug, Serialize)]
struct PresetParams {
    remove_preset: RemovePresetParams,
}

#[derive(Debug, Serialize)]
struct RemovePresetParams {
    id: Vec<String>,
}

impl SmartCamDeletePresetParams {
    pub fn new(id: &str) -> Self {
        Self {
            preset: PresetParams {
                remove_preset: RemovePresetParams {
                    id: vec![id.to_string()],
                },
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
        // What a C220 takes inside a `multipleRequest`.
        let request = TapoRequest::SmartCamDeletePreset(TapoParams::new(
            SmartCamDeletePresetParams::new("1"),
        ));

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({
                "method": "deletePreset",
                "params": { "preset": { "remove_preset": { "id": ["1"] } } },
            })
        );
    }
}
