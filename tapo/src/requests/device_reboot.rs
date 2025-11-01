use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub(crate) struct DeviceRebootParams {
    delay: u16,
}

impl DeviceRebootParams {
    pub fn new(delay: u16) -> Self {
        Self { delay }
    }
}
