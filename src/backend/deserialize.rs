use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

pub(crate) fn date_from_str<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s: String = Deserialize::deserialize(deserializer)?;
    s.push_str(" +0000");
    DateTime::parse_from_str(&s, "%F %T %z")
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(de::Error::custom)
}

pub(crate) fn u32_from_str<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    u32::from_str(&s).map_err(de::Error::custom)
}

pub(crate) fn u64_from_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    u64::from_str(&s).map_err(de::Error::custom)
}

pub(crate) fn f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    f64::from_str(&s).map_err(de::Error::custom)
}
