use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub(crate) struct GetEnergyUsageParams;

impl GetEnergyUsageParams {
    pub fn new() -> Self {
        Self
    }
}
