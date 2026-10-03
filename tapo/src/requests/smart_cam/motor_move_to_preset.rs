use serde::Serialize;

/// `module → section → data` envelope for `motorMoveToPreset`:
/// `{"preset": {"goto_preset": {"id": "1"}}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamMotorMoveToPresetParams {
    preset: PresetParams,
}

#[derive(Debug, Serialize)]
struct PresetParams {
    goto_preset: GotoPresetParams,
}

#[derive(Debug, Serialize)]
struct GotoPresetParams {
    id: String,
}

impl SmartCamMotorMoveToPresetParams {
    pub fn new(id: &str) -> Self {
        Self {
            preset: PresetParams {
                goto_preset: GotoPresetParams { id: id.to_string() },
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
        let request = TapoRequest::SmartCamMotorMoveToPreset(TapoParams::new(
            SmartCamMotorMoveToPresetParams::new("1"),
        ));

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({
                "method": "motorMoveToPreset",
                "params": { "preset": { "goto_preset": { "id": "1" } } },
            })
        );
    }
}
