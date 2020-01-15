use crate::{
    backend::requests::{
        Request, RequestType, CONV_TAG, HASH_TAG, LIMIT_TAG, MAP_TAG, MODE_TAG, MODS_TAG, SET_TAG,
        SINCE_TAG, USER_TAG,
    },
    models::{GameMod, GameMode},
};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

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

impl Request for BeatmapRequest {
    fn add_args(self, args: &mut HashMap<String, String>) -> RequestType {
        if let Some(since) = self.since {
            args.insert(SINCE_TAG.to_owned(), since.format("%F%%T").to_string());
        }
        if let Some(id) = self.map_id {
            args.insert(MAP_TAG.to_owned(), id.to_string());
        }
        if let Some(id) = self.mapset_id {
            args.insert(SET_TAG.to_owned(), id.to_string());
        }
        if let Some(id) = self.user_id {
            args.insert(USER_TAG.to_owned(), id.to_string());
        } else if let Some(name) = self.username {
            args.insert(USER_TAG.to_owned(), name);
        }
        if let Some(mode) = self.mode {
            args.insert(MODE_TAG.to_owned(), (mode as u8).to_string());
        }
        if let Some(limit) = self.limit {
            args.insert(LIMIT_TAG.to_owned(), limit.to_string());
        }
        if let Some(mods) = self.mods {
            args.insert(MODS_TAG.to_owned(), mods.to_string());
        }
        if let Some(with_converted) = self.with_converted {
            args.insert(CONV_TAG.to_owned(), (with_converted as u8).to_string());
        }
        if let Some(hash) = self.hash {
            args.insert(HASH_TAG.to_owned(), hash);
        }
        RequestType::Beatmap
    }
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

    pub fn username(self, name: String) -> Self {
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
            hash: self.hash,
        }
    }

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

    pub fn hash(self, hash: String) -> Self {
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
            hash: Some(hash),
        }
    }
}
