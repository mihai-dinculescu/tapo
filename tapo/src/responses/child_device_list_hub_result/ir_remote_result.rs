use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{DecodableResultExt, decode_value};

/// Device info of the IR remotes paired with a Tapo H110 hub.
///
/// IR remotes are virtual child devices (`SMART.TAPOREMOTE`) that are created by
/// the Tapo app, either by picking a device from TP-Link's IR database or by
/// learning the keys from a physical remote. They have no firmware or hardware
/// of their own, which is why `fw_ver`, `hw_id` and `hw_ver` are always empty.
///
/// Specific properties: `key_list`, `key_sum`, `customize_key_sum`, `downloaded_key_sum`,
/// `remote_id`, `remote_type`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct IrRemoteResult {
    // Common properties to all Hub child devices.
    pub avatar: String,
    pub bind_count: u32,
    pub category: String,
    pub device_id: String,
    pub fw_ver: String,
    pub hw_id: String,
    pub hw_ver: String,
    pub mac: String,
    /// The kind of appliance the remote controls, as named by the Tapo app.
    /// Unlike the other hub child devices, this is not a Tapo model
    /// (e.g. "TV", "AV", "Light").
    pub model: String,
    pub nickname: String,
    pub parent_device_id: String,
    pub r#type: String,
    // Specific properties to this device.
    #[serde(rename = "lastOnboardingTimestamp")]
    pub last_onboarding_timestamp: u64,
    /// The keys stored on this remote.
    pub key_list: Vec<IrRemoteKey>,
    /// The total number of keys stored on this remote.
    pub key_sum: u32,
    /// The number of keys that were learned from a physical remote.
    pub customize_key_sum: u32,
    /// The number of keys that were downloaded from TP-Link's IR database.
    pub downloaded_key_sum: u32,
    /// The id of the appliance in TP-Link's IR database, or `0` for a generic one.
    pub remote_id: u32,
    pub remote_type: u8,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(IrRemoteResult);

impl DecodableResultExt for IrRemoteResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        self.key_list = self
            .key_list
            .into_iter()
            .map(|key| key.decode())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(self)
    }
}

/// A key stored on an IR remote paired with a Tapo H110 hub.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
#[allow(missing_docs)]
pub struct IrRemoteKey {
    /// The id of the key in TP-Link's IR database, or `-1` for a key that was
    /// learned from a physical remote.
    pub id: i64,
    /// The name of the key. This is the value that
    /// [`IrRemoteHandler::send_ir_cmd_by_id`](crate::IrRemoteHandler::send_ir_cmd_by_id) expects.
    pub name: String,
    /// The label shown in the Tapo app. It is meaningful for most downloaded keys
    /// (e.g. "POWER", "VOL+"), but it can also be an opaque string, in which case
    /// [`IrRemoteKey::name`] is the only usable identifier.
    pub display_name: String,
    /// The carrier frequency of the key, in kHz.
    pub pwm: u8,
    /// The icon shown in the Tapo app. Only set for keys that were learned from
    /// a physical remote.
    pub icon: Option<String>,
    /// The position of the key in the Tapo app. Only set for keys that were
    /// learned from a physical remote.
    pub order: Option<u32>,
    pub r#type: Option<String>,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(IrRemoteKey);

impl DecodableResultExt for IrRemoteKey {
    fn decode(mut self) -> Result<Self, Error> {
        // Unlike the other encoded fields, `display_name` is not always valid
        // base64 encoded UTF-8, so it's left as-is when it cannot be decoded
        // instead of failing the whole child device list.
        if let Ok(display_name) = decode_value(&self.display_name) {
            self.display_name = display_name;
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::responses::ChildDeviceHubResult;

    use super::*;

    const IR_REMOTE_JSON: &str = r#"{
        "avatar": "",
        "bind_count": 0,
        "category": "ir.remote",
        "copy_device_id": "",
        "customize_key_sum": 1,
        "device_id": "0000000000000000000000000000000000000000",
        "downloaded_key_sum": 2,
        "fw_ver": "",
        "hw_id": "",
        "hw_ver": "",
        "isThirdPartySub": true,
        "key_list": [
            { "display_name": "UE9XRVI=", "id": 1, "name": "POWER", "pwm": 26 },
            { "display_name": "Vk9MKw==", "id": 50, "name": "VOL+", "pwm": 26 },
            { "display_name": "Ug==", "icon": "One", "id": -1, "name": "m8B637k", "order": 1, "pwm": 24, "type": "" }
        ],
        "key_sum": 3,
        "lastOnboardingTimestamp": 1767226732,
        "mac": "000000000000",
        "model": "TV",
        "nickname": "TGl2aW5nIFJvb20gVFY=",
        "oemId": "",
        "on": 0,
        "parent_device_id": "0000000000000000000000000000000000000000",
        "remote_id": 8964,
        "remote_type": 1,
        "type": "SMART.TAPOREMOTE"
    }"#;

    #[test]
    fn test_ir_remote_parse() {
        let child: ChildDeviceHubResult = serde_json::from_str(IR_REMOTE_JSON).unwrap();

        let ChildDeviceHubResult::IrRemote(remote) = child else {
            panic!("expected an IR remote, got {child:?}");
        };

        // The `model` of an IR remote is the appliance kind, so the variant must
        // be picked based on the `type` instead.
        assert_eq!(remote.model, "TV");
        assert_eq!(remote.remote_id, 8964);
        assert_eq!(remote.key_sum, 3);
        assert_eq!(remote.key_list.len(), 3);

        let remote = remote.decode().unwrap();

        assert_eq!(remote.nickname, "Living Room TV");
        assert_eq!(remote.key_list[0].name, "POWER");
        assert_eq!(remote.key_list[0].display_name, "POWER");
        assert_eq!(remote.key_list[1].display_name, "VOL+");

        let custom_key = &remote.key_list[2];
        assert_eq!(custom_key.id, -1);
        assert_eq!(custom_key.name, "m8B637k");
        assert_eq!(custom_key.display_name, "R");
        assert_eq!(custom_key.icon.as_deref(), Some("One"));
        assert_eq!(custom_key.order, Some(1));
    }

    #[test]
    fn test_ir_remote_keeps_undecodable_display_name() {
        let key: IrRemoteKey = serde_json::from_str(
            r#"{ "display_name": "not base64!", "id": 1, "name": "POWER", "pwm": 26 }"#,
        )
        .unwrap();

        assert_eq!(key.decode().unwrap().display_name, "not base64!");
    }
}
