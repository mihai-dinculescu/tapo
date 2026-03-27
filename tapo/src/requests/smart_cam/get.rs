use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub(crate) struct SmartCamGetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<SectionNames>,
}

impl SmartCamGetParams {
    pub fn device_info() -> Self {
        Self {
            device_info: Some(SectionNames::new(&["basic_info"])),
        }
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
