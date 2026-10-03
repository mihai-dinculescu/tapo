use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct SmartCamGetParams {
    /// The method that the request goes by inside a `multipleRequest`.
    #[serde(skip)]
    method: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<SectionNames>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<SectionNames>,
}

impl SmartCamGetParams {
    pub fn device_info() -> Self {
        Self {
            method: "getDeviceInfo",
            device_info: Some(SectionNames::new(&["basic_info"])),
            preset: None,
        }
    }

    pub fn preset() -> Self {
        Self {
            method: "getPresetConfig",
            device_info: None,
            preset: Some(SectionNames::new(&["preset"])),
        }
    }

    pub fn method(&self) -> &'static str {
        self.method
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct SectionNames {
    pub name: Vec<String>,
}

impl SectionNames {
    pub fn new(names: &[&str]) -> Self {
        Self {
            name: names.iter().map(|s| (*s).to_string()).collect(),
        }
    }
}
