use serde::{Deserialize, Serialize};

use crate::responses::TapoResponseExt;

/// A single PTZ preset position.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct Preset {
    /// Preset identifier.
    pub id: String,
    /// User-assigned name.
    pub name: String,
    /// Pan position (normalized, typically -1.0 to 1.0).
    pub pan: f64,
    /// Tilt position (normalized, typically -1.0 to 1.0).
    pub tilt: f64,
    /// Whether this preset is read-only.
    pub read_only: bool,
}

#[cfg(feature = "python")]
crate::impl_to_dict!(Preset);

/// Raw preset config as returned by the camera.
/// The camera returns parallel arrays; this struct matches that shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PresetRaw {
    #[serde(default)]
    pub id: Vec<String>,
    #[serde(default)]
    pub name: Vec<String>,
    #[serde(default)]
    pub position_pan: Vec<String>,
    #[serde(default)]
    pub position_tilt: Vec<String>,
    #[serde(default)]
    pub read_only: Vec<String>,
}

impl TapoResponseExt for PresetRaw {}

impl PresetRaw {
    /// Zips the parallel arrays into a list of [`Preset`] structs.
    ///
    /// Entries where `position_pan` or `position_tilt` fail to parse as `f64` are skipped.
    pub fn into_presets(self) -> Vec<Preset> {
        self.id
            .into_iter()
            .zip(self.name)
            .zip(self.position_pan)
            .zip(self.position_tilt)
            .zip(self.read_only)
            .filter_map(|((((id, name), pan), tilt), read_only)| {
                Some(Preset {
                    id,
                    name,
                    pan: pan.parse().ok()?,
                    tilt: tilt.parse().ok()?,
                    read_only: read_only != "0",
                })
            })
            .collect()
    }
}
