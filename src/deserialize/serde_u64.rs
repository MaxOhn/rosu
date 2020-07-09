use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::{fmt, str::FromStr};

struct U64Visitor;

impl<'de> Visitor<'de> for U64Visitor {
    type Value = Option<u64>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u64, a stringified number, or null")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        u64::from_str(v).map(Some).map_err(de::Error::custom)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub(crate) fn to_maybe_u64<'de, D>(d: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(U64Visitor)
}

pub(crate) fn to_u64<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(U64Visitor)?
        .expect("Could not unwrap u64"))
}
