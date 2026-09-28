use serde::Serialize;

use crate::error::Error;

#[derive(Debug, Serialize)]
pub(crate) struct LightSetDeviceInfoParams {
    brightness: u8,
}

impl LightSetDeviceInfoParams {
    pub(crate) fn brightness(value: u8) -> Result<Self, Error> {
        if !(1..=100).contains(&value) {
            return Err(Error::Validation {
                field: "brightness".to_string(),
                message: "Must be between 1 and 100".to_string(),
            });
        }

        Ok(Self { brightness: value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brightness() {
        for value in [0, 101] {
            assert!(matches!(
                LightSetDeviceInfoParams::brightness(value),
                Err(Error::Validation { field, message }) if field == "brightness" && message == "Must be between 1 and 100"
            ));
        }

        let params = LightSetDeviceInfoParams::brightness(50).unwrap();
        assert_eq!(
            serde_json::to_value(params).unwrap(),
            serde_json::json!({ "brightness": 50 })
        );
    }
}
