use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct SmartCamDoParams {
    /// The method that the request goes by inside a `multipleRequest`.
    #[serde(skip)]
    method: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motor: Option<MotorAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<PresetAction>,
}

impl SmartCamDoParams {
    pub fn motor_move(x: i32, y: i32) -> Self {
        Self {
            method: "motorMove",
            motor: Some(MotorAction {
                move_action: MotorMoveParams {
                    x_coord: x.to_string(),
                    y_coord: y.to_string(),
                },
            }),
            preset: None,
        }
    }

    pub fn set_preset(name: &str) -> Self {
        Self {
            // The misspelling is the device's.
            method: "addMotorPostion",
            motor: None,
            preset: Some(PresetAction {
                set_preset: Some(SetPresetParams {
                    name: name.to_string(),
                }),
                goto_preset: None,
                remove_preset: None,
            }),
        }
    }

    pub fn goto_preset(id: &str) -> Self {
        Self {
            method: "motorMoveToPreset",
            motor: None,
            preset: Some(PresetAction {
                set_preset: None,
                goto_preset: Some(GotoPresetParams { id: id.to_string() }),
                remove_preset: None,
            }),
        }
    }

    pub fn remove_preset(id: &str) -> Self {
        Self {
            method: "deletePreset",
            motor: None,
            preset: Some(PresetAction {
                set_preset: None,
                goto_preset: None,
                remove_preset: Some(RemovePresetParams {
                    id: vec![id.to_string()],
                }),
            }),
        }
    }

    pub fn method(&self) -> &'static str {
        self.method
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct MotorAction {
    #[serde(rename = "move")]
    pub move_action: MotorMoveParams,
}

#[derive(Debug, Serialize)]
pub(crate) struct MotorMoveParams {
    pub x_coord: String,
    pub y_coord: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct PresetAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_preset: Option<SetPresetParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goto_preset: Option<GotoPresetParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_preset: Option<RemovePresetParams>,
}

#[derive(Debug, Serialize)]
pub(crate) struct SetPresetParams {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct GotoPresetParams {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct RemovePresetParams {
    pub id: Vec<String>,
}
