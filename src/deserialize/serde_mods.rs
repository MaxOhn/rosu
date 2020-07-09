use crate::models::GameMods;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
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

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(GameMods::from_bits(v as u32))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub fn to_maybe_mods<'de, D>(d: D) -> Result<Option<GameMods>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(ModsVisitor)
}

impl<'de> Deserialize<'de> for GameMods {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_any(ModsVisitor)
            .transpose()
            .expect("Could not unwrap mods")
    }
}

impl Serialize for GameMods {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}
