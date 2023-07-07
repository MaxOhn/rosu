use crate::model::GameMods;
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{fmt, str::FromStr};

struct ModsVisitor;

impl<'de> Visitor<'de> for ModsVisitor {
    type Value = Option<GameMods>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u32, a stringified number, or null")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match u32::from_str(v) {
            Ok(n) => Ok(GameMods::from_bits(n)),
            Err(_) => GameMods::from_str(v)
                .map(Some)
                .map_err(|_| Error::invalid_value(Unexpected::Str(v), &"GameMods")),
        }
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(GameMods::from_bits(v as u32))
    }

    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(Self)
    }

    #[inline]
    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}

pub(crate) fn to_maybe_mods<'de, D: Deserializer<'de>>(d: D) -> Result<Option<GameMods>, D::Error> {
    d.deserialize_option(ModsVisitor)
}

impl<'de> Deserialize<'de> for GameMods {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(d.deserialize_any(ModsVisitor)?.unwrap_or_default())
    }
}

impl Serialize for GameMods {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u32(self.bits())
    }
}
