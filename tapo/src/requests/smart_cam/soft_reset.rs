use serde::Serialize;

/// `module → section → data` envelope for `softReset`:
/// `{"system": {"soft_reset": {"type": "factory"}}}`.
///
/// Cameras accept the plain `reset` request without resetting.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamSoftResetParams {
    system: SoftResetSystemParams,
}

#[derive(Debug, Serialize)]
struct SoftResetSystemParams {
    soft_reset: SoftResetParams,
}

#[derive(Debug, Serialize)]
struct SoftResetParams {
    #[serde(rename = "type")]
    reset_type: &'static str,
}

impl SmartCamSoftResetParams {
    pub fn factory() -> Self {
        Self {
            system: SoftResetSystemParams {
                soft_reset: SoftResetParams {
                    reset_type: "factory",
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
        let request =
            TapoRequest::SmartCamSoftReset(TapoParams::new(SmartCamSoftResetParams::factory()));

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({
                "method": "softReset",
                "params": { "system": { "soft_reset": { "type": "factory" } } },
            })
        );
    }
}
