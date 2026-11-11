use serde::Serialize;
use serde_json::Value;

/// The device info fields that the `DeviceInfo` capability leaves out: the
/// home's location and network, the vendor and regional variant and the state
/// the device restores at power-on, which repeats the names of the live state.
const OMITTED_DEVICE_INFO_FIELDS: [&str; 8] = [
    "default_states",
    "has_set_location_info",
    "latitude",
    "longitude",
    "mac",
    "oem_id",
    "specs",
    "ssid",
];

/// The JSON that the `DeviceInfo` capability returns for a device's info.
pub fn device_info_json(info: &impl Serialize) -> Result<Value, serde_json::Error> {
    let mut value = to_value(info)?;

    if let Value::Object(fields) = &mut value {
        for name in OMITTED_DEVICE_INFO_FIELDS {
            fields.remove(name);
        }
    }

    Ok(value)
}

/// Converts a library result to JSON, keeping each `f32` as the library has
/// it. [`serde_json::to_value`] widens an `f32` to `f64`, which turns 9.3 into
/// 9.300000190734863; JSON text carries it as 9.3.
pub fn to_value(result: &impl Serialize) -> Result<Value, serde_json::Error> {
    serde_json::from_str(&serde_json::to_string(result)?)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[derive(Serialize)]
    struct Reading {
        current_temp: f32,
        records: Vec<Record>,
    }

    #[derive(Serialize)]
    struct Record {
        temperature: f32,
        humidity: u8,
    }

    #[test]
    fn to_value_keeps_floats_as_the_library_has_them() {
        let reading = Reading {
            current_temp: 9.3,
            records: vec![
                Record {
                    temperature: -0.2,
                    humidity: 64,
                },
                Record {
                    temperature: 20.0,
                    humidity: 65,
                },
            ],
        };

        assert_eq!(
            to_value(&reading).unwrap().to_string(),
            r#"{"current_temp":9.3,"records":[{"humidity":64,"temperature":-0.2},{"humidity":65,"temperature":20.0}]}"#
        );
    }

    #[test]
    fn device_info_json_leaves_out_the_omitted_fields() {
        let info = json!({
            "brightness": 50,
            "default_states": { "state": { "brightness": 80 }, "type": "last_states" },
            "device_id": "0000000000000000000000000000000000000000",
            "device_on": true,
            "fw_id": "00000000000000000000000000000000",
            "fw_ver": "1.4.5 Build 260923 Rel.174333",
            "has_set_location_info": true,
            "hw_id": "00000000000000000000000000000000",
            "latitude": 515074,
            "longitude": -1278,
            "mac": "AA-BB-CC-DD-EE-FF",
            "oem_id": "00000000000000000000000000000000",
            "region": "Europe/London",
            "specs": "EU",
            "ssid": "Home",
        });

        assert_eq!(
            device_info_json(&info).unwrap(),
            json!({
                "brightness": 50,
                "device_id": "0000000000000000000000000000000000000000",
                "device_on": true,
                "fw_id": "00000000000000000000000000000000",
                "fw_ver": "1.4.5 Build 260923 Rel.174333",
                "hw_id": "00000000000000000000000000000000",
                "region": "Europe/London",
            })
        );
    }
}
