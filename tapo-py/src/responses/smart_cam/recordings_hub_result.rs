use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use serde::Serialize;
use tapo::responses::{RecordingHubResult, RecordingType};

/// Recording stored on a camera hub for a camera paired to it.
///
/// Wraps the Rust result so that `video_type` can be exposed as
/// [`PyRecordingType`]. `to_dict` serializes the wrapped value, so an
/// unknown `video_type` keeps its raw wire value there.
#[derive(Debug, Clone, Serialize)]
#[pyclass(name = "RecordingHubResult", from_py_object)]
#[serde(transparent)]
pub struct PyRecordingHubResult(RecordingHubResult);

impl From<RecordingHubResult> for PyRecordingHubResult {
    fn from(result: RecordingHubResult) -> Self {
        Self(result)
    }
}

#[pymethods]
impl PyRecordingHubResult {
    /// Start of the recording.
    #[getter]
    fn start_time(&self) -> DateTime<Utc> {
        self.0.start_time
    }

    /// End of the recording.
    #[getter]
    fn end_time(&self) -> DateTime<Utc> {
        self.0.end_time
    }

    /// The type of event that produced the recording.
    #[getter]
    fn video_type(&self) -> PyRecordingType {
        PyRecordingType::from(&self.0.video_type)
    }
}

tapo::impl_to_dict!(PyRecordingHubResult);

/// The type of event that produced a recording. PyO3 cannot expose a Rust
/// enum that mixes unit variants with a data-carrying one, so this plain enum
/// mirrors [`RecordingType`], with every unknown wire value folded into
/// `Other`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[pyclass(name = "RecordingType", from_py_object, eq, eq_int)]
#[allow(missing_docs)]
pub enum PyRecordingType {
    Timing,
    Motion,
    Tamper,
    LineCrossing,
    AreaIntrusion,
    Person,
    BabyCry,
    Vehicle,
    Pet,
    RingAlarm,
    Bark,
    Meow,
    GlassBreaking,
    Smoke,
    PackageDelivered,
    PackagePickedUp,
    MissedDoorbellRing,
    AnsweredDoorbellRing,
    AntiTheft,
    Face,
    UnfamiliarFace,
    UnfamiliarPerson,
    BabyLeave,
    BabyCaregiver,
    BabyAsleep,
    BabyAwake,
    FaceCover,
    SafeFenceOut,
    BabyMotion,
    PanoramicVideo,
    Animal,
    Other,
}

impl From<&RecordingType> for PyRecordingType {
    fn from(value: &RecordingType) -> Self {
        match value {
            RecordingType::Timing => Self::Timing,
            RecordingType::Motion => Self::Motion,
            RecordingType::Tamper => Self::Tamper,
            RecordingType::LineCrossing => Self::LineCrossing,
            RecordingType::AreaIntrusion => Self::AreaIntrusion,
            RecordingType::Person => Self::Person,
            RecordingType::BabyCry => Self::BabyCry,
            RecordingType::Vehicle => Self::Vehicle,
            RecordingType::Pet => Self::Pet,
            RecordingType::RingAlarm => Self::RingAlarm,
            RecordingType::Bark => Self::Bark,
            RecordingType::Meow => Self::Meow,
            RecordingType::GlassBreaking => Self::GlassBreaking,
            RecordingType::Smoke => Self::Smoke,
            RecordingType::PackageDelivered => Self::PackageDelivered,
            RecordingType::PackagePickedUp => Self::PackagePickedUp,
            RecordingType::MissedDoorbellRing => Self::MissedDoorbellRing,
            RecordingType::AnsweredDoorbellRing => Self::AnsweredDoorbellRing,
            RecordingType::AntiTheft => Self::AntiTheft,
            RecordingType::Face => Self::Face,
            RecordingType::UnfamiliarFace => Self::UnfamiliarFace,
            RecordingType::UnfamiliarPerson => Self::UnfamiliarPerson,
            RecordingType::BabyLeave => Self::BabyLeave,
            RecordingType::BabyCaregiver => Self::BabyCaregiver,
            RecordingType::BabyAsleep => Self::BabyAsleep,
            RecordingType::BabyAwake => Self::BabyAwake,
            RecordingType::FaceCover => Self::FaceCover,
            RecordingType::SafeFenceOut => Self::SafeFenceOut,
            RecordingType::BabyMotion => Self::BabyMotion,
            RecordingType::PanoramicVideo => Self::PanoramicVideo,
            RecordingType::Animal => Self::Animal,
            RecordingType::Other(_) => Self::Other,
        }
    }
}
