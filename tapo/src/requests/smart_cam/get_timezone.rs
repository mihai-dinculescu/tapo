use serde::Serialize;

/// `module → section → data` envelope for `getTimezone`:
/// `{"system": {"name": "basic"}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamGetTimezoneParams {
    system: SystemParams,
}

#[derive(Debug, Serialize)]
struct SystemParams {
    /// A single section name, not a list as `get` requests take.
    name: &'static str,
}

impl SmartCamGetTimezoneParams {
    pub fn new() -> Self {
        Self {
            system: SystemParams { name: "basic" },
        }
    }
}
