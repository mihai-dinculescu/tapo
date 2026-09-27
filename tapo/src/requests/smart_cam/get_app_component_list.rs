use serde::Serialize;

use crate::requests::SectionNames;

/// `module → section → data` envelope for `getAppComponentList`:
/// `{"app_component": {"name": ["app_component_list"]}}`.
#[derive(Debug, Serialize)]
pub(crate) struct SmartCamGetAppComponentListParams {
    app_component: SectionNames,
}

impl SmartCamGetAppComponentListParams {
    pub fn new() -> Self {
        Self {
            app_component: SectionNames::new(&["app_component_list"]),
        }
    }
}
