use crate::models::{
    ApprovalStatus, GameMode, GameMods, Genre, Grade, Language, ScoringType, Team, TeamType,
};
use chrono::{offset::TimeZone, DateTime, Utc};
use serde::{de, Deserialize, Deserializer};
use std::{convert::TryFrom, str::FromStr};

pub(crate) fn str_to_maybe_date<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = match Deserialize::deserialize(d) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    Utc.datetime_from_str(s, "%F %T")
        .map(Some)
        .map_err(de::Error::custom)
}

pub(crate) fn str_to_date<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(str_to_maybe_date(d)?.unwrap())
}

pub(crate) fn str_to_maybe_bool<'de, D>(d: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Deserialize::deserialize(d)?;
    Ok(s.and_then(|s| u8::from_str(s).map(|digit| digit == 1).ok()))
}

pub(crate) fn str_to_bool<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(str_to_maybe_bool(d)?.unwrap_or_else(|| false))
}

pub(crate) fn str_to_maybe_u32<'de, D>(d: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Deserialize::deserialize(d)?;
    Ok(s.and_then(|s| u32::from_str(s).ok()))
}

pub(crate) fn str_to_u32<'de, D>(d: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(str_to_maybe_u32(d)?.unwrap_or_else(|| 0))
}

pub(crate) fn str_to_u64<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    u64::from_str(s).map_err(de::Error::custom)
}

pub(crate) fn str_to_maybe_f32<'de, D>(d: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Deserialize::deserialize(d)?;
    Ok(s.and_then(|s| f32::from_str(s).ok()))
}

pub(crate) fn str_to_f32<'de, D>(d: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(str_to_maybe_f32(d)?.unwrap_or_else(|| 0.0))
}

pub(crate) fn str_to_mode<'de, D>(d: D) -> Result<GameMode, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    u8::from_str(s)
        .map_err(de::Error::custom)
        .map(GameMode::try_from)?
        .map_err(de::Error::custom)
}

pub(crate) fn str_to_approved<'de, D>(d: D) -> Result<ApprovalStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    i8::from_str(s)
        .map_err(de::Error::custom)
        .map(ApprovalStatus::try_from)?
        .map_err(de::Error::custom)
}

pub(crate) fn str_to_genre<'de, D>(d: D) -> Result<Genre, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    u8::from_str(s)
        .map_err(de::Error::custom)
        .map(Genre::try_from)?
        .map_err(de::Error::custom)
}

pub(crate) fn str_to_language<'de, D>(d: D) -> Result<Language, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    u8::from_str(s)
        .map_err(de::Error::custom)
        .map(Language::try_from)?
        .map_err(de::Error::custom)
}

pub(crate) fn str_to_maybe_mods<'de, D>(d: D) -> Result<Option<GameMods>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = match Deserialize::deserialize(d) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    u32::from_str(s)
        .map_err(de::Error::custom)
        .map(GameMods::try_from)?
        .map_err(de::Error::custom)
        .map(Some)
}

pub(crate) fn str_to_mods<'de, D>(d: D) -> Result<GameMods, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(str_to_maybe_mods(d)?.unwrap())
}

pub(crate) fn str_to_grade<'de, D>(d: D) -> Result<Grade, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    Grade::try_from(s).map_err(de::Error::custom)
}

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
