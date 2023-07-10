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
