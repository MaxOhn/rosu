use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::{fmt, str::FromStr};

struct U32Visitor;

impl<'de> Visitor<'de> for U32Visitor {
    type Value = Option<u32>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("xx a u32, a stringified number, or null")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        u32::from_str(v).map(Some).map_err(de::Error::custom)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v as u32))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
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

pub fn to_maybe_u32<'de, D>(d: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(U32Visitor)
}

pub fn to_u32<'de, D>(d: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(U32Visitor)?
        .expect("Could not unwrap u32"))
}
