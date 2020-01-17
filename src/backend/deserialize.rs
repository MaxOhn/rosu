use crate::models::{ApprovalStatus, GameMod, GameMode, Genre, Language};
use chrono::{offset::TimeZone, DateTime, Utc};
use serde::{de, Deserialize, Deserializer};
use std::{convert::TryFrom, str::FromStr};

pub(crate) fn str_to_maybe_date<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = match Deserialize::deserialize(d) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    Utc.datetime_from_str(&s, "%F %T")
        .map(Some)
        .map_err(serde::de::Error::custom)
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
    let s: String = match Deserialize::deserialize(d) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    let digit = u8::from_str(&s).map_err(de::Error::custom)?;
    Ok(Some(digit == 1))
}

pub(crate) fn str_to_bool<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(str_to_maybe_bool(d)?.unwrap())
}

pub(crate) fn str_to_maybe_u32<'de, D>(d: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = match Deserialize::deserialize(d) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    u32::from_str(&s).map_err(de::Error::custom).map(Some)
}

pub(crate) fn str_to_u32<'de, D>(d: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(str_to_maybe_u32(d)?.unwrap())
}

pub(crate) fn str_to_u64<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(d)?;
    u64::from_str(&s).map_err(de::Error::custom)
}

pub(crate) fn str_to_maybe_f64<'de, D>(d: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = match Deserialize::deserialize(d) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    f64::from_str(&s).map_err(de::Error::custom).map(Some)
}

pub(crate) fn str_to_f64<'de, D>(d: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(str_to_maybe_f64(d)?.unwrap())
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
        .map(GameMod::try_from)?
        .map_err(de::Error::custom)
}
