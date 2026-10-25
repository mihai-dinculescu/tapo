use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use serde::Serialize;
use tapo::responses::DetectionResult;

use crate::responses::PyRecordingType;

/// A detection stored on a camera's SD card.
///
/// Wraps the Rust result so that `trigger` and `events` can be exposed as
/// [`PyRecordingType`]. `to_dict` serializes the wrapped value, so an
/// unknown type keeps its raw wire value there.
#[derive(Debug, Clone, Serialize)]
#[pyclass(name = "DetectionResult", from_py_object)]
#[serde(transparent)]
pub struct PyDetectionResult(DetectionResult);

impl From<DetectionResult> for PyDetectionResult {
    fn from(result: DetectionResult) -> Self {
        Self(result)
    }
}

#[pymethods]
impl PyDetectionResult {
    /// Start of the detection.
    #[getter]
    fn start_time(&self) -> DateTime<Utc> {
        self.0.start_time
    }

    /// End of the detection.
    #[getter]
    fn end_time(&self) -> DateTime<Utc> {
        self.0.end_time
    }

    /// The event the camera files the detection under.
    #[getter]
    fn trigger(&self) -> PyRecordingType {
        PyRecordingType::from(&self.0.trigger)
    }

    /// Every event detected in the span.
    #[getter]
    fn events(&self) -> Vec<PyRecordingType> {
        self.0.events.iter().map(PyRecordingType::from).collect()
    }
}

tapo::impl_to_dict!(PyDetectionResult);
