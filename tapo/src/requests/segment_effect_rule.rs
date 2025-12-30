use serde::{Deserialize, Serialize};
use serde_with::{BoolFromInt, serde_as};

/// Parameters for the `apply_segment_effect_rule` request.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentEffectRuleParams {
    /// Brightness between 0 and 100.
    pub brightness: u8,
    /// Whether the effect is custom (serialized as 1/0).
    #[serde_as(as = "BoolFromInt")]
    #[serde(rename = "custom")]
    pub is_custom: bool,
    /// Device type string expected by the device, e.g. `strip`.
    #[serde(rename = "deviceType")]
    pub device_type: String,
    /// Colors displayed in the app.
    pub display_colors: Vec<[u16; 4]>,
    /// Whether the effect is enabled (serialized as 1/0).
    #[serde_as(as = "BoolFromInt")]
    #[serde(rename = "enable")]
    pub enabled: bool,
    /// Effect identifier. Any non-empty string is accepted by the device.
    pub id: String,
    /// Effect name shown in the app.
    pub name: String,
    /// Segment list, typically the total segment count for the strip (required).
    pub segments: Vec<u8>,
    /// Effect state list (required).
    pub states: Vec<[u16; 4]>,
    /// Effect type (e.g. `circulating`, `breathe`).
    #[serde(rename = "type")]
    pub r#type: String,
}

impl SegmentEffectRuleParams {
    /// Creates a new segment effect rule with the required fields.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        effect_type: impl Into<String>,
        brightness: u8,
        is_custom: bool,
        enabled: bool,
        display_colors: Vec<[u16; 4]>,
        segments: Vec<u8>,
        states: Vec<[u16; 4]>,
    ) -> Self {
        Self {
            brightness,
            is_custom,
            device_type: "strip".to_string(),
            display_colors,
            enabled,
            id: id.into(),
            name: name.into(),
            segments,
            states,
            r#type: effect_type.into(),
        }
    }

    /// Overrides the device type.
    pub fn with_device_type(mut self, device_type: impl Into<String>) -> Self {
        self.device_type = device_type.into();
        self
    }

    /// Sets the segments.
    pub fn with_segments(mut self, segments: Vec<u8>) -> Self {
        self.segments = segments;
        self
    }

    /// Sets the states.
    pub fn with_states(mut self, states: Vec<[u16; 4]>) -> Self {
        self.states = states;
        self
    }
}
