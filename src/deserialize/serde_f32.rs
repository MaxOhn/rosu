use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::{fmt, str::FromStr};

struct F32Visitor;

impl<'de> Visitor<'de> for F32Visitor {
    type Value = Option<f32>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a f32, a stringified number, or null")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        f32::from_str(v).map(Some).map_err(de::Error::custom)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
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

pub(crate) fn to_maybe_f32<'de, D>(d: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(F32Visitor)
}

pub(crate) fn to_f32<'de, D>(d: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(F32Visitor)?
        .expect("Could not unwrap f32"))
}
