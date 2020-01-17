use crate::{
    backend::requests::{Request, RequestType, LIMIT_TAG, MAP_TAG, MODE_TAG, MODS_TAG, USER_TAG},
    models::{GameMod, GameMode},
};
use std::collections::HashMap;

/// Request struct to retrieve scores on a beatmap. An instance __must__ contains a beatmap id.
pub struct ScoreRequest<'n> {
    pub map_id: Option<u32>,
    pub user_id: Option<u32>,
    pub username: Option<&'n str>,
    pub mode: Option<GameMode>,
    pub mods: Option<u32>,
    pub limit: Option<u32>,
}

impl<'n> Request for ScoreRequest<'n> {
    fn add_args(self, args: &mut HashMap<String, String>) -> (RequestType, bool) {
        if let Some(id) = self.map_id {
            args.insert(MAP_TAG.to_owned(), id.to_string());
        }
        if let Some(id) = self.user_id {
            args.insert(USER_TAG.to_owned(), id.to_string());
        } else if let Some(name) = self.username {
            args.insert(USER_TAG.to_owned(), name.to_owned().replace(" ", "%"));
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
        (RequestType::Score, false)
    }
}

impl<'n> ScoreRequest<'n> {
    /// Construct a `ScoreRequest` via beatmap id
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

    /// Specify a user id to only get scores from that user.
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

    /// Specify a username to only get scores from that user.
    pub fn username(self, name: &'n str) -> Self {
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: Some(name),
            mode: self.mode,
            mods: self.mods,
            limit: self.limit,
        }
    }

    /// Specify a game mode for the request
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

    /// Specify enabled mods for the retrieved scores
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

    /// Specify a limit for the amount of retrieved scores. Must be at most 100, defaults to 50.
    /// Only matters if neither user id nor username is specified.
    pub fn limit(self, limit: u32) -> Self {
        assert!(limit <= 100);
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
