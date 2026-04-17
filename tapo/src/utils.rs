use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};

pub fn der_tapo_datetime_format<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s = String::deserialize(deserializer)?;
    if !s.contains('T') {
        s = s.replace(' ', "T");
    }
    let value = NaiveDateTime::from_str(&s).map_err(serde::de::Error::custom)?;

    Ok(value)
}

pub fn ok_or_default<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Default,
    D: Deserializer<'de>,
{
    Ok(Deserialize::deserialize(deserializer).unwrap_or_default())
}

/// Deserialize a boolean from either a JSON bool or an integer (0/1).
pub(crate) fn bool_from_int_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Bool(b) => Ok(b),
        serde_json::Value::Number(n) => Ok(n.as_i64().unwrap_or(0) != 0),
        _ => Err(serde::de::Error::custom("expected bool or integer")),
    }
}

/// Deserialize an optional boolean from either a JSON bool, an integer (0/1), or absence.
pub(crate) fn option_bool_from_int_or_bool<'de, D>(
    deserializer: D,
) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    match value {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::Bool(b)) => Ok(Some(b)),
        Some(serde_json::Value::Number(n)) => Ok(Some(n.as_i64().unwrap_or(0) != 0)),
        _ => Err(serde::de::Error::custom("expected bool, integer, or null")),
    }
}
