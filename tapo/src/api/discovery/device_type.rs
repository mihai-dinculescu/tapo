use std::fmt;

use serde::{Deserialize, Serialize};

/// Categorizes a Tapo device by its capabilities.
///
/// This enum maps model strings (e.g. "L530", "P110") to a high-level device category,
/// centralizing the model-string-to-category logic in one place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(from_py_object, get_all, eq, eq_int)
)]
pub enum DeviceType {
    /// Tapo L510, L520, L610 — dimmable lights.
    Light,
    /// Tapo L530, L535, L630 — color lights.
    ColorLight,
    /// Tapo L900 — RGB light strip.
    RgbLightStrip,
    /// Tapo L920, L930 — RGBIC light strip.
    RgbicLightStrip,
    /// Tapo P100, P105 — smart plugs.
    Plug,
    /// Tapo P110, P110M, P115 — smart plugs with energy monitoring.
    PlugEnergyMonitoring,
    /// Tapo P300, P306 — power strips.
    PowerStrip,
    /// Tapo P304M, P316M — power strips with energy monitoring.
    PowerStripEnergyMonitoring,
    /// Tapo H100 — smart hub.
    Hub,
    /// Tapo C210, C220, C225, C325WB, C520WS, TC40, TC70 — smart cameras with PTZ.
    CameraPtz,
    /// A Tapo device without a specific handler implementation.
    Other,
}

impl DeviceType {
    /// Determines the device type from a model string.
    ///
    /// Unknown models return [`DeviceType::Other`].
    pub fn from_model(model: &str) -> Self {
        match model {
            "L510" | "L520" | "L610" => DeviceType::Light,
            "L530" | "L530 Series" | "L535" | "L535B" | "L630" => DeviceType::ColorLight,
            "L900" => DeviceType::RgbLightStrip,
            "L920" | "L930" => DeviceType::RgbicLightStrip,
            "P100" | "P105" => DeviceType::Plug,
            "P110" | "P110M" | "P115" => DeviceType::PlugEnergyMonitoring,
            "P300" | "P306" => DeviceType::PowerStrip,
            "P304M" | "P316M" => DeviceType::PowerStripEnergyMonitoring,
            "H100" => DeviceType::Hub,
            "C210" | "C220" | "C225" | "C325WB" | "C520WS" | "TC40" | "TC70" => {
                DeviceType::CameraPtz
            }
            _ => DeviceType::Other,
        }
    }
}

impl DeviceType {
    /// Returns a human-readable name for this device type as a static string.
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceType::Light => "Light",
            DeviceType::ColorLight => "Color Light",
            DeviceType::RgbLightStrip => "RGB Light Strip",
            DeviceType::RgbicLightStrip => "RGBIC Light Strip",
            DeviceType::Plug => "Plug",
            DeviceType::PlugEnergyMonitoring => "Plug with Energy Monitoring",
            DeviceType::PowerStrip => "Power Strip",
            DeviceType::PowerStripEnergyMonitoring => "Power Strip with Energy Monitoring",
            DeviceType::Hub => "Hub",
            DeviceType::CameraPtz => "Smart Camera with PTZ",
            DeviceType::Other => "Other",
        }
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_model_lights() {
        assert_eq!(DeviceType::from_model("L510"), DeviceType::Light);
        assert_eq!(DeviceType::from_model("L520"), DeviceType::Light);
        assert_eq!(DeviceType::from_model("L610"), DeviceType::Light);
    }

    #[test]
    fn from_model_color_lights() {
        assert_eq!(DeviceType::from_model("L530"), DeviceType::ColorLight);
        assert_eq!(
            DeviceType::from_model("L530 Series"),
            DeviceType::ColorLight
        );
        assert_eq!(DeviceType::from_model("L535"), DeviceType::ColorLight);
        assert_eq!(DeviceType::from_model("L535B"), DeviceType::ColorLight);
        assert_eq!(DeviceType::from_model("L630"), DeviceType::ColorLight);
    }

    #[test]
    fn from_model_light_strips() {
        assert_eq!(DeviceType::from_model("L900"), DeviceType::RgbLightStrip);
        assert_eq!(DeviceType::from_model("L920"), DeviceType::RgbicLightStrip);
        assert_eq!(DeviceType::from_model("L930"), DeviceType::RgbicLightStrip);
    }

    #[test]
    fn from_model_plugs() {
        assert_eq!(DeviceType::from_model("P100"), DeviceType::Plug);
        assert_eq!(DeviceType::from_model("P105"), DeviceType::Plug);
        assert_eq!(
            DeviceType::from_model("P110"),
            DeviceType::PlugEnergyMonitoring
        );
        assert_eq!(
            DeviceType::from_model("P110M"),
            DeviceType::PlugEnergyMonitoring
        );
        assert_eq!(
            DeviceType::from_model("P115"),
            DeviceType::PlugEnergyMonitoring
        );
    }

    #[test]
    fn from_model_power_strips() {
        assert_eq!(DeviceType::from_model("P300"), DeviceType::PowerStrip);
        assert_eq!(DeviceType::from_model("P306"), DeviceType::PowerStrip);
        assert_eq!(
            DeviceType::from_model("P304M"),
            DeviceType::PowerStripEnergyMonitoring
        );
        assert_eq!(
            DeviceType::from_model("P316M"),
            DeviceType::PowerStripEnergyMonitoring
        );
    }

    #[test]
    fn from_model_hub() {
        assert_eq!(DeviceType::from_model("H100"), DeviceType::Hub);
    }

    #[test]
    fn from_model_smart_cam_ptz() {
        assert_eq!(DeviceType::from_model("C210"), DeviceType::CameraPtz);
        assert_eq!(DeviceType::from_model("C220"), DeviceType::CameraPtz);
        assert_eq!(DeviceType::from_model("C225"), DeviceType::CameraPtz);
        assert_eq!(DeviceType::from_model("C325WB"), DeviceType::CameraPtz);
        assert_eq!(DeviceType::from_model("C520WS"), DeviceType::CameraPtz);
        assert_eq!(DeviceType::from_model("TC40"), DeviceType::CameraPtz);
        assert_eq!(DeviceType::from_model("TC70"), DeviceType::CameraPtz);
    }

    #[test]
    fn from_model_unknown_returns_other() {
        assert_eq!(DeviceType::from_model("UNKNOWN"), DeviceType::Other);
        assert_eq!(DeviceType::from_model(""), DeviceType::Other);
        assert_eq!(DeviceType::from_model("X999"), DeviceType::Other);
    }

    #[test]
    fn display() {
        assert_eq!(DeviceType::Other.to_string(), "Other");
        assert_eq!(DeviceType::Light.to_string(), "Light");
        assert_eq!(DeviceType::PowerStrip.to_string(), "Power Strip");
        assert_eq!(
            DeviceType::PowerStripEnergyMonitoring.to_string(),
            "Power Strip with Energy Monitoring"
        );
    }
}
