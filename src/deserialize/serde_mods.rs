use crate::models::GameMods;
use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::{convert::TryFrom, fmt};

// TODO: Visit array

struct ModsVisitor;

impl<'de> Visitor<'de> for ModsVisitor {
    type Value = Option<GameMods>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u32, a stringified number, or null")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        GameMods::try_from(v).map(Some).map_err(de::Error::custom)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(GameMods::from_bits(v))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub(crate) fn to_maybe_mods<'de, D>(d: D) -> Result<Option<GameMods>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(ModsVisitor)
}

pub(crate) fn to_mods<'de, D>(d: D) -> Result<GameMods, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(ModsVisitor)?
        .expect("Could not unwrap mods"))
}
