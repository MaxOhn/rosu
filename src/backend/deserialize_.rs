use crate::models::{ScoringType, Team, TeamType};
use serde::{de, Deserialize, Deserializer};
use std::{convert::TryFrom, str::FromStr};

pub(crate) fn str_to_scoring_type<'de, D>(d: D) -> Result<ScoringType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    u8::from_str(s)
        .map_err(de::Error::custom)
        .map(ScoringType::try_from)?
        .map_err(de::Error::custom)
}

pub(crate) fn str_to_team_type<'de, D>(d: D) -> Result<TeamType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    u8::from_str(s)
        .map_err(de::Error::custom)
        .map(TeamType::try_from)?
        .map_err(de::Error::custom)
}

pub(crate) fn str_to_team<'de, D>(d: D) -> Result<Team, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    u8::from_str(s)
        .map_err(de::Error::custom)
        .map(Team::try_from)?
        .map_err(de::Error::custom)
}
