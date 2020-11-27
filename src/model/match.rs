use crate::{
    model::{GameMode, GameMods},
    serde::*,
};

use chrono::{offset::TimeZone, DateTime, Utc};
use serde::{
    de::{Error, MapAccess, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, hash::Hash};

#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "serialize")]
use serde_repr::Serialize_repr;

/// Match struct retrieved from the `/api/get_match` endpoint.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Match {
    pub match_id: u32,
    pub name: String,
    #[cfg_attr(feature = "serialize", serde(with = "serde_date"))]
    pub start_time: DateTime<Utc>,
    #[cfg_attr(
        feature = "serialize",
        serde(with = "serde_maybe_date", skip_serializing_if = "Option::is_none")
    )]
    pub end_time: Option<DateTime<Utc>>,
    pub games: Vec<MatchGame>,
}

impl<'de> Deserialize<'de> for Match {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Match,
            Games,
            MatchId,
            Name,
            StartTime,
            EndTime,
        }

        struct MatchVisitor;

        impl<'de> Visitor<'de> for MatchVisitor {
            type Value = Match;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Match")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Match, V::Error>
            where
                V: MapAccess<'de>,
            {
                #[derive(Deserialize)]
                struct InnerMatch {
                    #[serde(deserialize_with = "to_u32")]
                    pub match_id: u32,
                    pub name: String,
                    #[serde(with = "serde_date")]
                    pub start_time: DateTime<Utc>,
                    #[serde(with = "serde_maybe_date")]
                    pub end_time: Option<DateTime<Utc>>,
                }

                let mut inner_match: Option<InnerMatch> = None;
                let mut games = None;
                let mut match_id = None;
                let mut name = None;
                let mut start_time: Option<String> = None;
                let mut end_time: Option<String> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Match => inner_match = Some(map.next_value()?),
                        Field::Games => games = Some(map.next_value()?),
                        Field::MatchId => match_id = Some(map.next_value()?),
                        Field::Name => name = Some(map.next_value()?),
                        Field::StartTime => start_time = Some(map.next_value()?),
                        Field::EndTime => end_time = Some(map.next_value()?),
                    }
                }
                let games = games.ok_or_else(|| Error::missing_field("games"))?;
                let osu_match = match inner_match {
                    Some(inner_match) => Match {
                        match_id: inner_match.match_id,
                        name: inner_match.name,
                        start_time: inner_match.start_time,
                        end_time: inner_match.end_time,
                        games,
                    },
                    None => {
                        if match_id.is_none() || name.is_none() || start_time.is_none() {
                            return Err(Error::custom(
                                "Deserializing Match requires either the field `match`, \
                                or the fields `match_id`, `name`, and `start_time`",
                            ));
                        }
                        let start_time = start_time.unwrap();
                        let start_time =
                            Utc.datetime_from_str(&start_time, "%F %T").map_err(|_| {
                                Error::invalid_value(
                                    Unexpected::Str(&start_time),
                                    &"date time of the format YYYY-MM-DD HH:MM:SS",
                                )
                            })?;
                        let end_time = end_time
                            .map(|end_time| {
                                Utc.datetime_from_str(&end_time, "%F %T").map_err(|_| {
                                    Error::invalid_value(
                                        Unexpected::Str(&end_time),
                                        &"date time of the format YYYY-MM-DD HH:MM:SS",
                                    )
                                })
                            })
                            .transpose()?;
                        Match {
                            match_id: match_id.unwrap(),
                            name: name.unwrap(),
                            start_time,
                            end_time,
                            games,
                        }
                    }
                };
                Ok(osu_match)
            }
        }

        const FIELDS: &[&str] = &["match", "games"];
        deserializer.deserialize_struct("Match", FIELDS, MatchVisitor)
    }
}

/// Each map that was not aborted during a [`Match`](crate::model::Match) will
/// produce a [`MatchGame`](crate::model::MatchGame) which contains the data of
/// the game and all its scores
#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct MatchGame {
    #[serde(deserialize_with = "to_u32")]
    pub game_id: u32,
    #[serde(with = "serde_date")]
    pub start_time: DateTime<Utc>,
    #[serde(with = "serde_maybe_date", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "to_u32")]
    pub beatmap_id: u32,
    #[serde(alias = "play_mode")]
    pub mode: GameMode,
    pub scoring_type: ScoringType,
    pub team_type: TeamType,
    #[serde(
        default,
        deserialize_with = "to_maybe_mods",
        skip_serializing_if = "Option::is_none"
    )]
    pub mods: Option<GameMods>,
    pub scores: Vec<GameScore>,
}

/// Each participating user of a [`MatchGame`](crate::model::MatchGame) will produce a [`GameScore`](crate::model::GameScore)
/// which contains the data about the user's play
#[derive(Debug, Clone, Hash, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct GameScore {
    #[serde(deserialize_with = "to_u32")]
    pub slot: u32,
    pub team: Team,
    #[serde(deserialize_with = "to_u32")]
    pub user_id: u32,
    #[serde(deserialize_with = "to_u32")]
    pub score: u32,
    #[serde(alias = "maxcombo", deserialize_with = "to_u32")]
    pub max_combo: u32,
    #[serde(deserialize_with = "to_u32")]
    pub count50: u32,
    #[serde(deserialize_with = "to_u32")]
    pub count100: u32,
    #[serde(deserialize_with = "to_u32")]
    pub count300: u32,
    #[serde(alias = "countmiss", deserialize_with = "to_u32")]
    pub count_miss: u32,
    #[serde(alias = "countgeki", deserialize_with = "to_u32")]
    pub count_geki: u32,
    #[serde(alias = "countkatu", deserialize_with = "to_u32")]
    pub count_katu: u32,
    #[serde(deserialize_with = "to_bool")]
    pub perfect: bool,
    #[serde(deserialize_with = "to_bool")]
    pub pass: bool,
    #[serde(
        default,
        deserialize_with = "to_maybe_mods",
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_mods: Option<GameMods>,
}

/// Basic enum to describe the scoring type of a [`Match`](crate::model::Match)
/// i.e. the winning condition
#[derive(Debug, Clone, Hash, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize_repr))]
#[repr(u8)]
pub enum ScoringType {
    Score = 0,
    Accuracy = 1,
    Combo = 2,
    ScoreV2 = 3,
}

impl From<u8> for ScoringType {
    fn from(t: u8) -> Self {
        match t {
            1 => Self::Accuracy,
            2 => Self::Combo,
            3 => Self::ScoreV2,
            _ => Self::Score,
        }
    }
}

/// Basic enum to describe the team type of a [`Match`](crate::model::Match)
#[derive(Debug, Clone, Hash, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize_repr))]
#[repr(u8)]
pub enum TeamType {
    HeadToHead = 0,
    TagCoop = 1,
    TeamVS = 2,
    TagTeamVS = 3,
}

impl From<u8> for TeamType {
    fn from(t: u8) -> Self {
        match t {
            1 => Self::TagCoop,
            2 => Self::TeamVS,
            3 => Self::TagTeamVS,
            _ => Self::HeadToHead,
        }
    }
}

/// Basic enum to declare a team of a [`Match`](crate::model::Match)
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize_repr))]
#[repr(u8)]
pub enum Team {
    None = 0,
    Blue = 1,
    Red = 2,
}

impl From<u8> for Team {
    fn from(t: u8) -> Self {
        match t {
            1 => Self::Blue,
            2 => Self::Red,
            _ => Self::None,
        }
    }
}
