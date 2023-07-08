use serde::{
    de::{Error, Visitor},
    Deserializer,
};
use std::{
    fmt::{Formatter, Result as FmtResult},
    str::FromStr,
};

struct U32Visitor;

impl<'de> Visitor<'de> for U32Visitor {
    type Value = Option<u32>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a u32, a stringified number, or null")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        let v = i32::from_str(v).map_err(Error::custom)?;

        Ok(Some(v.max(0) as u32))
    }

    #[inline]
    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Some(v as u32))
    }

    #[inline]
    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(Some(v.max(0) as u32))
    }

    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(Self)
    }

    #[inline]
    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}

pub(crate) fn to_maybe_u32<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u32>, D::Error> {
    d.deserialize_option(U32Visitor)
}

pub(crate) fn to_u32<'de, D: Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
    Ok(d.deserialize_option(U32Visitor)?.unwrap_or(0))
}
