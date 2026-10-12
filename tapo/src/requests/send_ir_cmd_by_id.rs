use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct SendIrCmdByIdParams {
    name: String,
}

impl SendIrCmdByIdParams {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
