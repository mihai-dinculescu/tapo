use serde::Deserialize;

use crate::responses::{Component, TapoResponseExt};

/// App component list result (`getAppComponentList`).
#[derive(Debug, Deserialize)]
pub(crate) struct AppComponentListResultRaw {
    app_component: AppComponentRaw,
}

impl AppComponentListResultRaw {
    pub fn components(self) -> Vec<Component> {
        self.app_component
            .app_component_list
            .into_iter()
            .map(|component| Component {
                id: component.name,
                ver_code: component.version,
            })
            .collect()
    }
}

impl TapoResponseExt for AppComponentListResultRaw {}

#[derive(Debug, Deserialize)]
struct AppComponentRaw {
    app_component_list: Vec<AppComponentItemRaw>,
}

#[derive(Debug, Deserialize)]
struct AppComponentItemRaw {
    name: String,
    version: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_component_list_parses() {
        let json = r#"{"app_component": {"app_component_list": [
            {"name": "generalCameraManage", "version": 1},
            {"name": "faceDetection", "version": 2}
        ]}}"#;

        let parsed: AppComponentListResultRaw = serde_json::from_str(json).unwrap();
        let components = parsed.components();

        assert_eq!(components.len(), 2);
        assert_eq!(components[0].id, "generalCameraManage");
        assert_eq!(components[0].ver_code, 1);
        assert_eq!(components[1].id, "faceDetection");
        assert_eq!(components[1].ver_code, 2);
    }
}
