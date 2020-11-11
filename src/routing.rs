#![allow(clippy::enum_variant_names)]

use crate::{
    model::{GameMode, GameMods},
    request::{Request, UserIdentification},
};

#[cfg(feature = "cache")]
use crate::serde::serde_maybe_date;

use chrono::{DateTime, Utc};
use std::fmt::Write;

#[cfg(feature = "cache")]
use serde::{Deserialize, Serialize};

const CONV_TAG: &str = "a";
const EVENT_DAYS_TAG: &str = "event_days";
const HASH_TAG: &str = "h";
const LIMIT_TAG: &str = "limit";
const MAP_TAG: &str = "b";
const MODE_TAG: &str = "m";
const MODS_TAG: &str = "mods";
const MP_TAG: &str = "mp";
const SET_TAG: &str = "s";
const SINCE_TAG: &str = "since";

#[derive(Debug)]
#[cfg_attr(feature = "cache", derive(Deserialize, Serialize))]
/// Base data to build the url for a request.
/// Serves as key for cached values.
pub(crate) enum Route {
    #[cfg_attr(feature = "cache", serde(rename = "A"))]
    /// Route information to get beatmaps
    GetBeatmaps {
        #[cfg_attr(
            feature = "cache",
            serde(rename = "a", default, skip_serializing_if = "Option::is_none")
        )]
        /// Creator of the mapset, specified either by id or username
        creator: Option<UserIdentification>,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "b", default, skip_serializing_if = "Option::is_none")
        )]
        /// Hash of a beatmap
        hash: Option<String>,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "c", default, skip_serializing_if = "Option::is_none")
        )]
        /// Upper limit of beatmaps to retrieve
        limit: Option<u32>,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "d", default, skip_serializing_if = "Option::is_none")
        )]
        /// Beatmap id of map to retrieve
        map_id: Option<u32>,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "e", default, skip_serializing_if = "Option::is_none")
        )]
        /// Beatmapset id of maps to retrieve
        mapset_id: Option<u32>,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "f", default, skip_serializing_if = "Option::is_none")
        )]
        /// GameMode of maps to retrieve
        mode: Option<GameMode>,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "g", default, skip_serializing_if = "Option::is_none")
        )]
        /// Adjust map fields like star rating by GameMods
        mods: Option<GameMods>,

        #[cfg_attr(
            feature = "cache",
            serde(
                rename = "h",
                with = "serde_maybe_date",
                default,
                skip_serializing_if = "Option::is_none"
            )
        )]
        /// Only maps created after this date
        since: Option<DateTime<Utc>>,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "i", default, skip_serializing_if = "Option::is_none")
        )]
        /// With or without converted maps
        with_converted: Option<bool>,
    },
    #[cfg_attr(feature = "cache", serde(rename = "B"))]
    /// Route information to get a multiplayer match
    GetMatch {
        #[cfg_attr(feature = "cache", serde(rename = "a"))]
        match_id: u32,
    },
    #[cfg_attr(feature = "cache", serde(rename = "C"))]
    /// Route information to get scores
    GetScore {
        #[cfg_attr(
            feature = "cache",
            serde(rename = "a", default, skip_serializing_if = "Option::is_none")
        )]
        /// Upper limit of beatmaps to retrieve
        limit: Option<u32>,

        #[cfg_attr(feature = "cache", serde(rename = "b"))]
        /// The map on which the scores were set
        map_id: u32,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "c", default, skip_serializing_if = "Option::is_none")
        )]
        /// The mode of scores to retrieve
        mode: Option<GameMode>,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "d", default, skip_serializing_if = "Option::is_none")
        )]
        /// GameMods of scores to retrieve
        mods: Option<GameMods>,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "e", default, skip_serializing_if = "Option::is_none")
        )]
        /// The user of scores to retrieve
        user: Option<UserIdentification>,
    },
    #[cfg_attr(feature = "cache", serde(rename = "D"))]
    /// Route information to get a user
    GetUser {
        #[cfg_attr(feature = "cache", serde(rename = "a"))]
        /// The user, specified either by id or username
        user: UserIdentification,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "b", default, skip_serializing_if = "Option::is_none")
        )]
        /// The gamemode of the user
        mode: Option<GameMode>,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "c", default, skip_serializing_if = "Option::is_none")
        )]
        /// Max number of days between now and last event date
        event_days: Option<u32>,
    },
    #[cfg_attr(feature = "cache", serde(rename = "E"))]
    /// Route information to get the top scores of a user
    GetUserBest {
        #[cfg_attr(
            feature = "cache",
            serde(rename = "a", default, skip_serializing_if = "Option::is_none")
        )]
        /// Upper limit of scores to retrieve
        limit: Option<u32>,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "b", default, skip_serializing_if = "Option::is_none")
        )]
        /// The gamemode of the scores
        mode: Option<GameMode>,

        #[cfg_attr(feature = "cache", serde(rename = "c"))]
        /// The user who created the scores
        user: UserIdentification,
    },
    #[cfg_attr(feature = "cache", serde(rename = "F"))]
    /// Route information to get a user's recent scores
    GetUserRecent {
        #[cfg_attr(
            feature = "cache",
            serde(rename = "a", default, skip_serializing_if = "Option::is_none")
        )]
        /// Upper limit of scores to retrieve
        limit: Option<u32>,

        #[cfg_attr(
            feature = "cache",
            serde(rename = "b", default, skip_serializing_if = "Option::is_none")
        )]
        /// The gamemode of the scores
        mode: Option<GameMode>,

        #[cfg_attr(feature = "cache", serde(rename = "c"))]
        /// The user who created the scores
        user: UserIdentification,
    },
}

impl From<Route> for Request {
    fn from(route: Route) -> Self {
        let uri = match route {
            Route::GetBeatmaps {
                creator,
                hash,
                limit,
                map_id,
                mapset_id,
                mode,
                mods,
                since,
                with_converted,
            } => {
                let mut uri = String::from("get_beatmaps?");
                if let Some(creator) = creator {
                    let _ = write!(uri, "&{}", creator);
                }
                if let Some(hash) = hash {
                    let _ = write!(uri, "&{}={}", HASH_TAG, hash);
                }
                if let Some(limit) = limit {
                    let _ = write!(uri, "&{}={}", LIMIT_TAG, limit);
                }
                if let Some(map_id) = map_id {
                    let _ = write!(uri, "&{}={}", MAP_TAG, map_id);
                }
                if let Some(mapset_id) = mapset_id {
                    let _ = write!(uri, "&{}={}", SET_TAG, mapset_id);
                }
                if let Some(mode) = mode {
                    let _ = write!(uri, "&{}={}", MODE_TAG, mode as u8);
                }
                if let Some(mods) = mods {
                    let _ = write!(uri, "&{}={}", MODS_TAG, mods.bits());
                }
                if let Some(date) = since {
                    let _ = write!(uri, "&{}={}", SINCE_TAG, date.format("%F%%T"));
                }
                if let Some(with_converted) = with_converted {
                    let _ = write!(uri, "&{}={}", CONV_TAG, with_converted as u8);
                }
                uri
            }
            Route::GetMatch { match_id } => format!("get_match?{}={}", MP_TAG, match_id),
            Route::GetScore {
                limit,
                map_id,
                mode,
                mods,
                user,
            } => {
                let mut uri = format!("get_scores?{}={}", MAP_TAG, map_id);
                if let Some(limit) = limit {
                    let _ = write!(uri, "&{}={}", LIMIT_TAG, limit);
                }
                if let Some(mode) = mode {
                    let _ = write!(uri, "&{}={}", MODE_TAG, mode as u8);
                }
                if let Some(mods) = mods {
                    let _ = write!(uri, "&{}={}", MODS_TAG, mods.bits());
                }
                if let Some(user) = user {
                    let _ = write!(uri, "&{}", user);
                }
                uri
            }
            Route::GetUser {
                user,
                mode,
                event_days,
            } => {
                let mut uri = format!("get_user?{}", user);
                if let Some(mode) = mode {
                    let _ = write!(uri, "&{}={}", MODE_TAG, mode as u8);
                }
                if let Some(days) = event_days {
                    let _ = write!(uri, "&{}={}", EVENT_DAYS_TAG, days);
                }
                uri
            }
            Route::GetUserBest { limit, mode, user } => {
                let mut uri = format!("get_user_best?{}", user);
                if let Some(limit) = limit {
                    let _ = write!(uri, "&{}={}", LIMIT_TAG, limit);
                }
                if let Some(mode) = mode {
                    let _ = write!(uri, "&{}={}", MODE_TAG, mode as u8);
                }
                uri
            }
            Route::GetUserRecent { limit, mode, user } => {
                let mut uri = format!("get_user_recent?{}", user);
                if let Some(limit) = limit {
                    let _ = write!(uri, "&{}={}", LIMIT_TAG, limit);
                }
                if let Some(mode) = mode {
                    let _ = write!(uri, "&{}={}", MODE_TAG, mode as u8);
                }
                uri
            }
        };
        Request(uri)
    }
}
