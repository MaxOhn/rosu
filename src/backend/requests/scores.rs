use crate::{
    backend::requests::{Request, RequestType, LIMIT_TAG, MAP_TAG, MODE_TAG, MODS_TAG, USER_TAG},
    models::{GameMod, GameMode},
};
use std::collections::HashMap;

pub struct ScoreRequest {
    pub map_id: Option<u32>,
    pub user_id: Option<u32>,
    pub username: Option<String>,
    pub mode: Option<GameMode>,
    pub mods: Option<u32>,
    pub limit: Option<u32>,
}

impl Request for ScoreRequest {
    fn add_args(self, args: &mut HashMap<String, String>) -> RequestType {
        if let Some(id) = self.map_id {
            args.insert(MAP_TAG.to_owned(), id.to_string());
        }
        if let Some(id) = self.user_id {
            args.insert(USER_TAG.to_owned(), id.to_string());
        } else if let Some(name) = self.username {
            args.insert(USER_TAG.to_owned(), name);
        }
        if let Some(mode) = self.mode {
            args.insert(MODE_TAG.to_owned(), (mode as u8).to_string());
        }
        if let Some(mods) = self.mods {
            args.insert(MODS_TAG.to_owned(), mods.to_string());
        }
        if let Some(limit) = self.limit {
            args.insert(LIMIT_TAG.to_owned(), limit.to_string());
        }
        RequestType::Score
    }
}

impl ScoreRequest {
    pub fn with_map_id(id: u32) -> Self {
        Self {
            map_id: Some(id),
            user_id: None,
            username: None,
            mode: None,
            mods: None,
            limit: None,
        }
    }

    pub fn user_id(self, id: u32) -> Self {
        Self {
            map_id: self.map_id,
            user_id: Some(id),
            username: self.username,
            mode: self.mode,
            mods: self.mods,
            limit: self.limit,
        }
    }

    pub fn username(self, name: String) -> Self {
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: Some(name),
            mode: self.mode,
            mods: self.mods,
            limit: self.limit,
        }
    }

    pub fn mode(self, mode: GameMode) -> Self {
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: self.username,
            mode: Some(mode),
            mods: self.mods,
            limit: self.limit,
        }
    }

    pub fn mods(self, mods: &[GameMod]) -> Self {
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            mods: Some(GameMod::slice_to_u32(mods)),
            limit: self.limit,
        }
    }

    pub fn limit(self, limit: u32) -> Self {
        assert!(limit < 100);
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            mods: self.mods,
            limit: Some(limit),
        }
    }
}
