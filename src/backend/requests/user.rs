use crate::models::GameMode;

pub struct UserReq {
    user_id: Option<u32>,
    username: Option<String>,
    mode: Option<GameMode>,
}

impl UserReq {
    pub fn with_id(id: u32) -> Self {
        Self {
            user_id: Some(id),
            username: None,
            mode: None,
        }
    }

    pub fn with_name(name: String) -> Self {
        Self {
            user_id: None,
            username: Some(name),
            mode: None,
        }
    }

    pub fn mode(&self, mode: GameMode) -> Self {
        Self {
            user_id: self.user_id,
            username: self.username.clone(),
            mode: Some(mode),
        }
    }

    pub fn get_user_id(&self) -> Option<u32> {
        self.user_id
    }

    pub fn get_username(&self) -> Option<String> {
        self.username.clone()
    }

    pub fn get_mode(&self) -> Option<GameMode> {
        self.mode
    }
}
