use crate::models::TeamType;
use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::fmt;

struct TeamTypeVisitor;

impl<'de> Visitor<'de> for TeamTypeVisitor {
    type Value = Option<TeamType>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u8, null, or a stringified number")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let team_type = match v {
            "0" => Some(TeamType::HeadToHead),
            "1" => Some(TeamType::TagCoop),
            "2" => Some(TeamType::TeamVS),
            "3" => Some(TeamType::TagTeamVS),
            _ => None,
        };
        Ok(team_type)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(TeamType::from(v)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub fn to_team_type<'de, D>(d: D) -> Result<TeamType, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(TeamTypeVisitor)?
        .expect("Could not unwrap team type"))
}
