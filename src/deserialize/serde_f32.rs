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
        println!("checking str");
        f32::from_str(v).map(Some).map_err(de::Error::custom)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        println!("checking f64");
        Ok(Some(v as f32))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        println!("checking null");
        Ok(None)
    }
}

pub fn to_maybe_f32<'de, D>(d: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(F32Visitor)
}

pub fn to_f32<'de, D>(d: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(F32Visitor)?.unwrap_or_else(|| {
        debug!("WARN: Serializing None to f32");
        0.0
    }))
}
