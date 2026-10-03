use serde::Serialize;

/// `module → section → data` envelope for `motorMove`:
/// `{"motor": {"move": {"x_coord": "10", "y_coord": "-5"}}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamMotorMoveParams {
    motor: MotorParams,
}

#[derive(Debug, Serialize)]
struct MotorParams {
    #[serde(rename = "move")]
    move_action: MoveParams,
}

#[derive(Debug, Serialize)]
struct MoveParams {
    x_coord: String,
    y_coord: String,
}

impl SmartCamMotorMoveParams {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            motor: MotorParams {
                move_action: MoveParams {
                    x_coord: x.to_string(),
                    y_coord: y.to_string(),
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
        let request =
            TapoRequest::SmartCamMotorMove(TapoParams::new(SmartCamMotorMoveParams::new(10, -5)));

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({
                "method": "motorMove",
                "params": { "motor": { "move": { "x_coord": "10", "y_coord": "-5" } } },
            })
        );
    }
}
