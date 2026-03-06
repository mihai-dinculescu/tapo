#[cfg(feature = "debug")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct ComponentListResult {
    pub component_list: Vec<Component>,
}

#[cfg(feature = "debug")]
impl crate::responses::TapoResponseExt for ComponentListResult {}

/// A component (feature/capability) reported by a Tapo device.
#[cfg(feature = "debug")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, get_all))]
pub struct Component {
    /// The component identifier (e.g. `"energy_monitoring"`, `"countdown"`).
    pub id: String,
    /// The version code of the component.
    pub ver_code: u8,
}

#[cfg(all(feature = "debug", feature = "python"))]
#[pyo3::pymethods]
impl Component {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}
