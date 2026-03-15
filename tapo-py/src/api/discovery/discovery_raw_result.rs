use pyo3::prelude::*;
use tapo::{DiscoveryError, DiscoveryRawResult};

#[pyclass(name = "MaybeDiscoveryRawResult")]
pub struct PyMaybeDiscoveryRawResult {
    result: Option<DiscoveryRawResult>,
    exception: Option<DiscoveryError>,
}

#[pymethods]
impl PyMaybeDiscoveryRawResult {
    pub fn get(mut slf: PyRefMut<'_, Self>) -> PyResult<DiscoveryRawResult> {
        if let Some(result) = slf.result.take() {
            Ok(result)
        } else if let Some(exception) = slf.exception.take() {
            Err(exception.into())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "No result or exception available. `get` can only be called once.",
            ))
        }
    }
}

pub fn convert_raw_result_to_maybe_py(
    result: Result<DiscoveryRawResult, DiscoveryError>,
) -> PyResult<PyMaybeDiscoveryRawResult> {
    match result {
        Ok(result) => Ok(PyMaybeDiscoveryRawResult {
            result: Some(result),
            exception: None,
        }),
        Err(e) => Ok(PyMaybeDiscoveryRawResult {
            result: None,
            exception: Some(e),
        }),
    }
}
