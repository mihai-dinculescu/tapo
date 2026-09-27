use serde::{Deserialize, Serialize};

/// A still snapshot captured from a camera.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object))]
pub struct Snapshot {
    /// The raw bytes of the snapshot.
    pub data: Vec<u8>,
    /// MIME content type of the snapshot, e.g. `"image/jpeg"`.
    pub content_type: String,
}

#[cfg(feature = "python")]
#[pyo3::prelude::pymethods]
impl Snapshot {
    /// The raw bytes of the snapshot.
    #[getter]
    fn data<'py>(&self, py: pyo3::Python<'py>) -> pyo3::Bound<'py, pyo3::types::PyBytes> {
        pyo3::types::PyBytes::new(py, &self.data)
    }

    /// MIME content type of the snapshot, e.g. `"image/jpeg"`.
    #[getter]
    fn content_type(&self) -> String {
        self.content_type.clone()
    }
}

#[cfg(feature = "python")]
crate::impl_to_dict!(Snapshot);
