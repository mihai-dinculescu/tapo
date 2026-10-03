use serde::Serialize;

/// `module → section → data` envelope for `addMotorPostion`:
/// `{"preset": {"set_preset": {"name": "Door"}}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamAddMotorPositionParams {
    preset: PresetParams,
}

#[derive(Debug, Serialize)]
struct PresetParams {
    set_preset: SetPresetParams,
}

#[derive(Debug, Serialize)]
struct SetPresetParams {
    name: String,
}

impl SmartCamAddMotorPositionParams {
    pub fn new(name: &str) -> Self {
        Self {
            preset: PresetParams {
                set_preset: SetPresetParams {
                    name: name.to_string(),
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
        let request = TapoRequest::SmartCamAddMotorPosition(TapoParams::new(
            SmartCamAddMotorPositionParams::new("Door"),
        ));

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({
                "method": "addMotorPostion",
                "params": { "preset": { "set_preset": { "name": "Door" } } },
            })
        );
    }
}
