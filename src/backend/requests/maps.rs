use crate::models::{GameMod, GameMode};
use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct BeatmapRequest {
    since: Option<DateTime<Utc>>,
    map_id: Option<u32>,
    mapset_id: Option<u32>,
    user_id: Option<u32>,
    username: Option<String>,
    mode: Option<GameMode>,
    limit: Option<u32>,
    mods: Option<u32>,
    with_converted: Option<bool>,
    hash: Option<String>,
}

impl BeatmapRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn since(&self, date: DateTime<Utc>) -> Self {
        Self {
            since: Some(date),
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username.clone(),
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash.clone(),
        }
    }

    pub fn map_id(&self, id: u32) -> Self {
        Self {
            since: self.since,
            map_id: Some(id),
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username.clone(),
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash.clone(),
        }
    }

    pub fn mapset_id(&self, id: u32) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: Some(id),
            user_id: self.user_id,
            username: self.username.clone(),
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash.clone(),
        }
    }

    pub fn user_id(&self, id: u32) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: Some(id),
            username: self.username.clone(),
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash.clone(),
        }
    }

    pub fn username(&self, name: String) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: Some(name),
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash.clone(),
        }
    }

    pub fn mode(&self, mode: GameMode) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username.clone(),
            mode: Some(mode),
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash.clone(),
        }
    }

    pub fn limit(&self, limit: u32) -> Self {
        assert!(limit <= 500);
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username.clone(),
            mode: self.mode,
            limit: Some(limit),
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash.clone(),
        }
    }

    pub fn mods(&self, mods: &[GameMod]) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username.clone(),
            mode: self.mode,
            limit: self.limit,
            mods: Some(GameMod::slice_to_u32(mods)),
            with_converted: self.with_converted,
            hash: self.hash.clone(),
        }
    }

    pub fn with_converted(&self, with_converted: bool) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username.clone(),
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: Some(with_converted),
            hash: self.hash.clone(),
        }
    }

    pub fn hash(&self, hash: String) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username.clone(),
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: Some(hash),
        }
    }

    pub(crate) fn get_since(&self) -> Option<DateTime<Utc>> {
        self.since
    }

    pub(crate) fn get_map_id(&self) -> Option<u32> {
        self.map_id
    }

    pub(crate) fn get_mapset_id(&self) -> Option<u32> {
        self.mapset_id
    }

    pub(crate) fn get_user_id(&self) -> Option<u32> {
        self.user_id
    }

    pub(crate) fn get_username(&self) -> Option<String> {
        self.username.clone()
    }

    pub(crate) fn get_mode(&self) -> Option<GameMode> {
        self.mode
    }

    pub(crate) fn get_limit(&self) -> Option<u32> {
        self.limit
    }

    pub(crate) fn get_mods(&self) -> Option<u32> {
        self.mods
    }

    pub(crate) fn get_with_converted(&self) -> Option<bool> {
        self.with_converted
    }

    pub(crate) fn get_hash(&self) -> Option<String> {
        self.hash.clone()
    }
}
