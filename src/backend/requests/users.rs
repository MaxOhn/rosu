use crate::models::GameMode;

pub struct UserRequest {
    user_id: Option<u32>,
    username: Option<String>,
    mode: Option<GameMode>,
    event_days: Option<u32>,
}

impl UserRequest {
    pub fn with_user_id(id: u32) -> Self {
        Self {
            user_id: Some(id),
            username: None,
            mode: None,
            event_days: None,
        }
    }

    pub fn with_username(name: String) -> Self {
        Self {
            user_id: None,
            username: Some(name),
            mode: None,
            event_days: None,
        }
    }

    pub fn mode(&self, mode: GameMode) -> Self {
        Self {
            user_id: self.user_id,
            username: self.username.clone(),
            mode: Some(mode),
            event_days: self.event_days,
        }
    }

    pub fn event_days(&self, amount: u32) -> Self {
        Self {
            user_id: self.user_id,
            username: self.username.clone(),
            mode: self.mode,
            event_days: Some(amount),
        }
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

    pub(crate) fn get_event_days(&self) -> Option<u32> {
        self.event_days
    }
}
