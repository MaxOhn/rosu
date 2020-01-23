use crate::models::{GameMod, GameMode};
use chrono::{DateTime, Utc};

#[derive(Clone, Default)]
/// Request struct to retrieve beatmaps. Unless specified otherwise through the `with_cache` method,
/// it will try to use rosu's cache and check if this url has been requested already
pub struct BeatmapArgs {
    pub(crate) since: Option<DateTime<Utc>>,
    pub(crate) map_id: Option<u32>,
    pub(crate) mapset_id: Option<u32>,
    pub(crate) user_id: Option<u32>,
    pub(crate) username: Option<String>,
    pub(crate) mode: Option<GameMode>,
    pub(crate) limit: Option<u32>,
    pub(crate) mods: Option<u32>,
    pub(crate) with_converted: Option<bool>,
    pub(crate) hash: Option<String>,
}

impl BeatmapArgs {
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify a date to only consider maps from this date onwards.
    pub fn since(self, date: DateTime<Utc>) -> Self {
        Self {
            since: Some(date),
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify a beatmap id to only retrieve that map.
    pub fn map_id(self, id: u32) -> Self {
        Self {
            since: self.since,
            map_id: Some(id),
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify a beatmapset id to retrieve all maps of that set.
    pub fn mapset_id(self, id: u32) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: Some(id),
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify a user id to only get beatmaps created by that user.
    pub fn user_id(self, id: u32) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: Some(id),
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify a username to only get beatmaps created by that user.
    pub fn username(self, name: &str) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: Some(name.to_owned()),
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify a game mode for the request
    pub fn mode(self, mode: GameMode) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: Some(mode),
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify a limit for the amount of retrieved beatmaps. Default and limit are 500.
    pub fn limit(self, limit: u32) -> Self {
        assert!(limit <= 500);
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: Some(limit),
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify mods for the retrieved beatmaps
    pub fn mods(self, mods: &[GameMod]) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: Some(GameMod::slice_to_u32(mods)),
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify whether converted maps should be included, default is false.
    pub fn with_converted(self, with_converted: bool) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: Some(with_converted),
            hash: self.hash,
        }
    }

    /// Specify the hash value of a beatmap that will be retrieved
    pub fn hash(self, hash: &str) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: Some(hash.to_owned()),
        }
    }
}
