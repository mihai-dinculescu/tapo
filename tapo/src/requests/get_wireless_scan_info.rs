use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct GetWirelessScanInfoParams {
    start_index: usize,
}

impl GetWirelessScanInfoParams {
    pub fn new(start_index: usize) -> Self {
        Self { start_index }
    }
}
