use crate::models::{GameMod, GameMode};

pub struct ScoreRequest {
    pub map_id: Option<u32>,
    pub user_id: Option<u32>,
    pub username: Option<String>,
    pub mode: Option<GameMode>,
    pub mods: Option<u32>,
    pub limit: Option<u32>,
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

    pub fn user_id(&self, id: u32) -> Self {
        Self {
            map_id: self.map_id,
            user_id: Some(id),
            username: self.username.clone(),
            mode: self.mode,
            mods: self.mods,
            limit: self.limit,
        }
    }

    pub fn username(&self, name: String) -> Self {
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: Some(name),
            mode: self.mode,
            mods: self.mods,
            limit: self.limit,
        }
    }

    pub fn mode(&self, mode: GameMode) -> Self {
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: self.username.clone(),
            mode: Some(mode),
            mods: self.mods,
            limit: self.limit,
        }
    }

    pub fn mods(&self, mods: &[GameMod]) -> Self {
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: self.username.clone(),
            mode: self.mode,
            mods: Some(GameMod::slice_to_u32(mods)),
            limit: self.limit,
        }
    }

    pub fn limit(&self, limit: u32) -> Self {
        assert!(limit < 100);
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: self.username.clone(),
            mode: self.mode,
            mods: self.mods,
            limit: Some(limit),
        }
    }

    pub(crate) fn get_map_id(&self) -> Option<u32> {
        self.map_id
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

    pub(crate) fn get_mods(&self) -> Option<u32> {
        self.mods
    }

    pub(crate) fn get_limit(&self) -> Option<u32> {
        self.limit
    }
}
