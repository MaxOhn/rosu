use crate::models::{ApprovalStatus, GameMod, GameMode, Genre, Language};
use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer};
use std::{convert::TryFrom, str::FromStr};

pub(crate) fn str_to_date<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(str_to_maybe_date(d)?.unwrap())
}

pub(crate) fn str_to_maybe_date<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s: String = match Deserialize::deserialize(d) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    s.push_str(" +0000");
    DateTime::parse_from_str(&s, "%F %T %z")
        .map(|dt| Some(dt.with_timezone(&Utc)))
        .map_err(de::Error::custom)
}

pub(crate) fn str_to_bool<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    let digit = u8::from_str(&s).map_err(de::Error::custom)?;
    Ok(digit == 1)
}

pub(crate) fn str_to_u32<'de, D>(d: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    u32::from_str(&s).map_err(de::Error::custom)
}

pub(crate) fn str_to_u64<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    u64::from_str(&s).map_err(de::Error::custom)
}

pub(crate) fn str_to_f64<'de, D>(d: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    f64::from_str(&s).map_err(de::Error::custom)
}

pub(crate) fn str_to_mode<'de, D>(d: D) -> Result<GameMode, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    u8::from_str(&s)
        .map_err(de::Error::custom)
        .map(GameMode::try_from)?
        .map_err(de::Error::custom)
}

pub(crate) fn str_to_approved<'de, D>(d: D) -> Result<ApprovalStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    i8::from_str(&s)
        .map_err(de::Error::custom)
        .map(ApprovalStatus::try_from)?
        .map_err(de::Error::custom)
}

pub(crate) fn str_to_genre<'de, D>(d: D) -> Result<Genre, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    u8::from_str(&s)
        .map_err(de::Error::custom)
        .map(Genre::try_from)?
        .map_err(de::Error::custom)
}

pub(crate) fn str_to_language<'de, D>(d: D) -> Result<Language, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    u8::from_str(&s)
        .map_err(de::Error::custom)
        .map(Language::try_from)?
        .map_err(de::Error::custom)
}

pub(crate) fn str_to_mods<'de, D>(d: D) -> Result<Vec<GameMod>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    u32::from_str(&s)
        .map_err(de::Error::custom)
        .map(GameMod::mods_from_u32)?
        .map_err(de::Error::custom)
}
