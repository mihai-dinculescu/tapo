use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct GetTriggerLogsParams {
    page_size: u64,
    start_id: u64,
}

impl GetTriggerLogsParams {
    pub fn new(page_size: u64, start_id: u64) -> Self {
        Self {
            page_size,
            start_id,
        }
    }
}
