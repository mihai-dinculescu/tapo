use serde::{Deserialize,Serialize};

/// Temperature unit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}