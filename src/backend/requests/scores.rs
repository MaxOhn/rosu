use crate::models::{GameMod, GameMode};

#[derive(Clone)]
/// Request struct to retrieve scores on a beatmap. An instance __must__ contains a beatmap id.
pub struct ScoreArgs {
    pub(crate)  map_id: Option<u32>,
    pub(crate)  user_id: Option<u32>,
    pub(crate)  username: Option<String>,
    pub(crate)  mode: Option<GameMode>,
    pub(crate)  mods: Option<u32>,
    pub(crate)  limit: Option<u32>,
}

impl ScoreArgs {
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
    pub fn username(self, name: &str) -> Self {
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: Some(name.to_owned()),
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
