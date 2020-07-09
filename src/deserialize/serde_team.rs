use crate::models::Team;
use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::fmt;

struct TeamVisitor;

impl<'de> Visitor<'de> for TeamVisitor {
    type Value = Option<Team>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u8, null, or a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let team = match v {
            "0" | "none" => Some(Team::None),
            "1" | "blue" => Some(Team::Blue),
            "2" | "red" => Some(Team::Red),
            _ => None,
        };
        Ok(team)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(Team::from(v)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub(crate) fn to_maybe_team<'de, D>(d: D) -> Result<Option<Team>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(TeamVisitor)
}

pub(crate) fn to_team<'de, D>(d: D) -> Result<Team, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(TeamVisitor)?
        .expect("Could not unwrap team"))
}
