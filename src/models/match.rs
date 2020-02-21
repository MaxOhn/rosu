use crate::{
    backend::deserialize::*,
    models::{GameMode, GameMods},
    OsuError,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize as DerivedDeserialize;
use std::convert::TryFrom;

/// Match struct retrieved from the `/api/get_match` endpoint.
#[derive(Debug, Clone)]
pub struct Match {
    pub match_id: u32,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub games: Vec<MatchGame>,
}

impl<'de> Deserialize<'de> for Match {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(DerivedDeserialize)]
        struct Outer {
            #[serde(rename = "match")]
            osu_match: Inner,
            games: Vec<MatchGame>,
        }

        #[derive(DerivedDeserialize)]
        struct Inner {
            #[serde(deserialize_with = "str_to_u32")]
            pub match_id: u32,
            pub name: String,
            #[serde(deserialize_with = "str_to_date")]
            pub start_time: DateTime<Utc>,
            #[serde(deserialize_with = "str_to_maybe_date")]
            pub end_time: Option<DateTime<Utc>>,
        }

        let helper = Outer::deserialize(deserializer)?;
        Ok(Match {
            match_id: helper.osu_match.match_id,
            name: helper.osu_match.name,
            start_time: helper.osu_match.start_time,
            end_time: helper.osu_match.end_time,
            games: helper.games,
        })
    }
}

/// Each map that was not aborted during a [Match][match] will
/// produce a `MatchGame` which contains the data of
/// the game and all its scores
///
/// [match]: struct.Match.html
#[derive(Debug, Clone, DerivedDeserialize)]
pub struct MatchGame {
    #[serde(deserialize_with = "str_to_u32")]
    pub game_id: u32,
    #[serde(deserialize_with = "str_to_date")]
    pub start_time: DateTime<Utc>,
    #[serde(deserialize_with = "str_to_maybe_date")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "str_to_u32")]
    pub beatmap_id: u32,
    #[serde(rename = "play_mode", deserialize_with = "str_to_mode")]
    pub mode: GameMode,
    #[serde(deserialize_with = "str_to_scoring_type")]
    pub scoring_type: ScoringType,
    #[serde(deserialize_with = "str_to_team_type")]
    pub team_type: TeamType,
    #[serde(deserialize_with = "str_to_maybe_mods")]
    pub mods: Option<GameMods>,
    pub scores: Vec<GameScore>,
}

/// Each participating user of a [MatchGame][game] will produce a `GameScore`
/// which contains the data about the user's play
///
/// [game]: struct.MatchGame.html
#[derive(Debug, Clone, DerivedDeserialize)]
pub struct GameScore {
    #[serde(deserialize_with = "str_to_u32")]
    pub slot: u32,
    #[serde(deserialize_with = "str_to_team")]
    pub team: Team,
    #[serde(deserialize_with = "str_to_u32")]
    pub user_id: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub score: u32,
    #[serde(rename = "maxcombo", deserialize_with = "str_to_u32")]
    pub max_combo: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub count50: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub count100: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub count300: u32,
    #[serde(rename = "countmiss", deserialize_with = "str_to_u32")]
    pub count_miss: u32,
    #[serde(rename = "countgeki", deserialize_with = "str_to_u32")]
    pub count_geki: u32,
    #[serde(rename = "countkatu", deserialize_with = "str_to_u32")]
    pub count_katu: u32,
    #[serde(deserialize_with = "str_to_bool")]
    pub perfect: bool,
    #[serde(deserialize_with = "str_to_bool")]
    pub pass: bool,
    #[serde(deserialize_with = "str_to_maybe_mods")]
    pub enabled_mods: Option<GameMods>,
}

/// Basic enum to describe the scoring type of a match
/// i.e. the winning condition
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum ScoringType {
    Score = 0,
    Accuracy = 1,
    Combo = 2,
    ScoreV2 = 3,
}

impl TryFrom<u8> for ScoringType {
    type Error = OsuError;
    fn try_from(t: u8) -> Result<Self, Self::Error> {
        match t {
            0 => Ok(Self::Score),
            1 => Ok(Self::Accuracy),
            2 => Ok(Self::Combo),
            3 => Ok(Self::ScoreV2),
            _ => Err(OsuError::Other(format!(
                "Can not parse {} into ScoringType",
                t
            ))),
        }
    }
}

/// Basic enum to describe the team type of a match
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TeamType {
    HeadToHead = 0,
    TagCoop = 1,
    TeamVS = 2,
    TagTeamVS = 3,
}

impl TryFrom<u8> for TeamType {
    type Error = OsuError;
    fn try_from(t: u8) -> Result<Self, Self::Error> {
        match t {
            0 => Ok(Self::HeadToHead),
            1 => Ok(Self::TagCoop),
            2 => Ok(Self::TeamVS),
            3 => Ok(Self::TagTeamVS),
            _ => Err(OsuError::Other(format!(
                "Can not parse {} into TeamType",
                t
            ))),
        }
    }
}

/// Basic enum to declare a team
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Team {
    None = 0,
    Blue = 1,
    Red = 2,
}

impl TryFrom<u8> for Team {
    type Error = OsuError;
    fn try_from(t: u8) -> Result<Self, Self::Error> {
        match t {
            0 => Ok(Self::None),
            1 => Ok(Self::Blue),
            2 => Ok(Self::Red),
            _ => Err(OsuError::Other(format!("Can not parse {} into Team", t))),
        }
    }
}
