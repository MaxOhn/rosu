use serde::{
    de::{Error, Visitor},
    Deserializer,
};
use std::{fmt, str::FromStr};

struct U64Visitor;

impl<'de> Visitor<'de> for U64Visitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u64, a stringified number, or null")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        u64::from_str(v).map_err(Error::custom)
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(v)
    }
}

pub(crate) fn to_u64<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
    d.deserialize_any(U64Visitor)
}
