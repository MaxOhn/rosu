use crate::models::GameMode;
use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::fmt;

struct ModeVisitor;

impl<'de> Visitor<'de> for ModeVisitor {
    type Value = Option<GameMode>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u8, null, or a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mode = match v {
            "0" | "osu" => Some(GameMode::STD),
            "1" | "taiko" | "tko" => Some(GameMode::TKO),
            "2" | "ctb" | "fruits" => Some(GameMode::CTB),
            "3" | "mania" | "mna" => Some(GameMode::MNA),
            _ => None,
        };
        Ok(mode)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mode = match v {
            0 => Some(GameMode::STD),
            1 => Some(GameMode::TKO),
            2 => Some(GameMode::CTB),
            3 => Some(GameMode::MNA),
            _ => None,
        };
        Ok(mode)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub fn to_mode<'de, D>(d: D) -> Result<GameMode, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(ModeVisitor)?
        .expect("Could not unwrap mode"))
}
