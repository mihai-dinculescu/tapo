use std::fmt;

use serde::{Deserialize, Serialize};

/// Categorizes a hub child device by what it does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HubChildType {
    /// Tapo KE100 - thermostatic radiator valve (TRV).
    ThermostaticRadiatorValve,
    /// Tapo S200B, S200D - button switch.
    ButtonSwitch,
    /// Tapo S210 - light switch.
    LightSwitch,
    /// Tapo T100 - motion sensor.
    MotionSensor,
    /// Tapo T110 - contact sensor.
    ContactSensor,
    /// Tapo T300 - water sensor.
    WaterSensor,
    /// Tapo T310, T315 - temperature and humidity sensor.
    TemperatureHumiditySensor,
    /// IR remote paired with an H110 hub.
    IrRemote,
    /// A hub child device without a specific result type.
    Other,
}

impl HubChildType {
    /// Returns a human-readable name for this hub child type as a static string.
    pub fn as_str(&self) -> &'static str {
        match self {
            HubChildType::ThermostaticRadiatorValve => "Thermostatic Radiator Valve",
            HubChildType::ButtonSwitch => "Button Switch",
            HubChildType::LightSwitch => "Light Switch",
            HubChildType::MotionSensor => "Motion Sensor",
            HubChildType::ContactSensor => "Contact Sensor",
            HubChildType::WaterSensor => "Water Sensor",
            HubChildType::TemperatureHumiditySensor => "Temperature and Humidity Sensor",
            HubChildType::IrRemote => "IR Remote",
            HubChildType::Other => "Other",
        }
    }
}

impl fmt::Display for HubChildType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
