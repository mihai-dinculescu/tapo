use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub(crate) struct GetChildDeviceListParams {
    start_index: u64,
}

impl GetChildDeviceListParams {
    pub fn new(start_index: u64) -> Self {
        Self { start_index }
    }
}
