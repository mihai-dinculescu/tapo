use serde::Serialize;

/// The sections that a request reads of a module: `{"name": ["basic_info"]}`.
#[derive(Debug, Serialize)]
pub(crate) struct SectionNames {
    name: Vec<String>,
}

impl SectionNames {
    pub fn new(names: &[&str]) -> Self {
        Self {
            name: names.iter().map(|s| (*s).to_string()).collect(),
        }
    }
}
