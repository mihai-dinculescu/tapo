use serde::{Deserialize, Deserializer, Serializer};
use time::{format_description, OffsetDateTime, PrimitiveDateTime};

const FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let format = format_description::parse(FORMAT).map_err(serde::de::Error::custom)?;

    let primitive = PrimitiveDateTime::parse(&s, &format).map_err(serde::de::Error::custom)?;

    // assume system offset
    // it's pretty safe to use the offset of the current dt, as we're deserializing the response right away
    let offset = OffsetDateTime::now_utc().offset();

    Ok(primitive.assume_offset(offset))
}

pub fn serialize<S>(data: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let format = format_description::parse(FORMAT).map_err(serde::ser::Error::custom)?;

    let formatted = data.format(&format).map_err(serde::ser::Error::custom)?;

    serializer.serialize_str(&formatted)
}
