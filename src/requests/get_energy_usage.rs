use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct GetEnergyUsageParams;

impl GetEnergyUsageParams {
    pub fn new() -> Self {
        Self
    }
}
