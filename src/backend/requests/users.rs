use crate::{
    backend::requests::{Request, RequestType, EVENT_DAYS_TAG, MODE_TAG, USER_TAG},
    models::GameMode,
};
use std::collections::HashMap;

pub struct UserRequest {
    user_id: Option<u32>,
    username: Option<String>,
    mode: Option<GameMode>,
    event_days: Option<u32>,
}

impl Request for UserRequest {
    fn add_args(self, args: &mut HashMap<String, String>) -> RequestType {
        if let Some(id) = self.user_id {
            args.insert(USER_TAG.to_owned(), id.to_string());
        } else if let Some(name) = self.username {
            args.insert(USER_TAG.to_owned(), name);
        }
        if let Some(mode) = self.mode {
            args.insert(MODE_TAG.to_owned(), (mode as u8).to_string());
        }
        if let Some(amount) = self.event_days {
            args.insert(EVENT_DAYS_TAG.to_owned(), amount.to_string());
        }
        RequestType::User
    }
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

    pub fn mode(self, mode: GameMode) -> Self {
        Self {
            user_id: self.user_id,
            username: self.username,
            mode: Some(mode),
            event_days: self.event_days,
        }
    }

    pub fn event_days(self, amount: u32) -> Self {
        Self {
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            event_days: Some(amount),
        }
    }
}
