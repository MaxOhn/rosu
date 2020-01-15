use crate::{
    backend::requests::{Request, RequestType, LIMIT_TAG, MODE_TAG, USER_TAG},
    models::GameMode,
};
use std::collections::HashMap;

pub struct UserRecentRequest {
    pub user_id: Option<u32>,
    pub username: Option<String>,
    pub mode: Option<GameMode>,
    pub limit: Option<u32>,
}

impl Request for UserRecentRequest {
    fn add_args(self, args: &mut HashMap<String, String>) -> RequestType {
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
        RequestType::UserRecent
    }
}

impl UserRecentRequest {
    pub fn with_user_id(id: u32) -> Self {
        Self {
            user_id: Some(id),
            username: None,
            mode: None,
            limit: None,
        }
    }

    pub fn with_username(name: String) -> Self {
        Self {
            user_id: None,
            username: Some(name),
            mode: None,
            limit: None,
        }
    }

    pub fn mode(self, mode: GameMode) -> Self {
        Self {
            user_id: self.user_id,
            username: self.username,
            mode: Some(mode),
            limit: self.limit,
        }
    }

    pub fn limit(self, limit: u32) -> Self {
        assert!(limit <= 50);
        Self {
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: Some(limit),
        }
    }
}
