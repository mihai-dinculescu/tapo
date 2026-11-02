use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct SetAutoUpdateInfoParams {
    enable: bool,
    time: u16,
    random_range: u16,
}

impl SetAutoUpdateInfoParams {
    pub fn new(enable: bool, time: u16, random_range: u16) -> Self {
        Self {
            enable,
            time,
            random_range,
        }
    }
}
