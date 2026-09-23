use serde::{Deserialize, Serialize};

/// Bitmask of the AI detection types a camera paired to a camera hub
/// runs itself: `1` person, `2` pet, `4` vehicle, `8` face.
/// Bits above `8` are reserved for detection types not yet known.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object))]
#[serde(transparent)]
pub struct AiCameraSupport(u64);

impl AiCameraSupport {
    /// Whether the camera runs person detection.
    pub fn person(&self) -> bool {
        self.0 & 1 != 0
    }

    /// Whether the camera runs pet detection.
    pub fn pet(&self) -> bool {
        self.0 & 2 != 0
    }

    /// Whether the camera runs vehicle detection.
    pub fn vehicle(&self) -> bool {
        self.0 & 4 != 0
    }

    /// Whether the camera runs face detection.
    pub fn face(&self) -> bool {
        self.0 & 8 != 0
    }

    /// The raw bitmask value.
    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[cfg(feature = "python")]
#[pyo3::prelude::pymethods]
impl AiCameraSupport {
    /// Whether the camera runs person detection.
    #[getter(person)]
    fn get_person(&self) -> bool {
        self.person()
    }

    /// Whether the camera runs pet detection.
    #[getter(pet)]
    fn get_pet(&self) -> bool {
        self.pet()
    }

    /// Whether the camera runs vehicle detection.
    #[getter(vehicle)]
    fn get_vehicle(&self) -> bool {
        self.vehicle()
    }

    /// Whether the camera runs face detection.
    #[getter(face)]
    fn get_face(&self) -> bool {
        self.face()
    }

    /// The raw bitmask value.
    #[getter(raw)]
    fn get_raw(&self) -> u64 {
        self.raw()
    }
}
