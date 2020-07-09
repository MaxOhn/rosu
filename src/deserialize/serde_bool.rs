use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::{fmt, str::FromStr};

struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = Option<bool>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "a bool, a stringified bool, null, or 0 or 1 in either number, string or char format",
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(b) = bool::from_str(v) {
            return Ok(Some(b));
        }
        u8::from_str(v)
            .map(|n| match n {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            })
            .map_err(de::Error::custom)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let b = match v {
            '0' => Some(false),
            '1' => Some(true),
            _ => None,
        };
        Ok(b)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
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

pub(crate) fn to_maybe_bool<'de, D>(d: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(BoolVisitor)
}

pub(crate) fn to_bool<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(BoolVisitor)?
        .expect("Could not unwrap bool"))
}
