#![allow(clippy::enum_variant_names)]

use crate::{
    model::{GameMode, GameMods},
    request::{Request, UserIdentification},
    serde::NAIVE_DATETIME_FORMAT,
};

use std::fmt::Write;

use time::OffsetDateTime;

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
/// Base data to build the url for a request.
pub(crate) enum Route {
    /// Route information to get beatmaps
    GetBeatmaps {
        /// Creator of the mapset, specified either by id or username
        creator: Option<UserIdentification>,

        /// Hash of a beatmap
        hash: Option<String>,

        /// Upper limit of beatmaps to retrieve
        limit: Option<u32>,

        /// Beatmap id of map to retrieve
        map_id: Option<u32>,

        /// Beatmapset id of maps to retrieve
        mapset_id: Option<u32>,

        /// GameMode of maps to retrieve
        mode: Option<GameMode>,

        /// Adjust map fields like star rating by GameMods
        mods: Option<GameMods>,

        /// Only maps created after this date
        since: Option<OffsetDateTime>,

        /// With or without converted maps
        with_converted: Option<bool>,
    },
    /// Route information to get a multiplayer match
    GetMatch { match_id: u32 },
    /// Route information to get scores
    GetScore {
        /// Upper limit of beatmaps to retrieve
        limit: Option<u32>,

        /// The map on which the scores were set
        map_id: u32,

        /// The mode of scores to retrieve
        mode: Option<GameMode>,

        /// GameMods of scores to retrieve
        mods: Option<GameMods>,

        /// The user of scores to retrieve
        user: Option<UserIdentification>,
    },
    /// Route information to get a user
    GetUser {
        /// The user, specified either by id or username
        user: UserIdentification,

        /// The gamemode of the user
        mode: Option<GameMode>,

        /// Max number of days between now and last event date
        event_days: Option<u32>,
    },
    /// Route information to get the top scores of a user
    GetUserBest {
        /// Upper limit of scores to retrieve
        limit: Option<u32>,

        /// The gamemode of the scores
        mode: Option<GameMode>,

        /// The user who created the scores
        user: UserIdentification,
    },
    /// Route information to get a user's recent scores
    GetUserRecent {
        /// Upper limit of scores to retrieve
        limit: Option<u32>,

        /// The gamemode of the scores
        mode: Option<GameMode>,

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

                if let Some(Ok(date)) = since.map(|date| date.format(NAIVE_DATETIME_FORMAT)) {
                    let _ = write!(uri, "&{}={}", SINCE_TAG, date);
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

        Request(uri.into_boxed_str())
    }
}
