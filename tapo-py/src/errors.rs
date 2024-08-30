use pyo3::exceptions::PyException;
use pyo3::PyErr;
use tapo::Error;

pub struct ErrorWrapper(pub Error);

impl From<Error> for ErrorWrapper {
    fn from(err: Error) -> Self {
        Self(err)
    }
}

impl From<anyhow::Error> for ErrorWrapper {
    fn from(err: anyhow::Error) -> Self {
        Self(err.into())
    }
}

impl From<ErrorWrapper> for PyErr {
    fn from(err: ErrorWrapper) -> PyErr {
        PyException::new_err(format!("{:?}", err.0))
    }
}
