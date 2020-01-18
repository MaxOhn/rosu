use crate::{
    backend::requests::{Request, RequestType, LIMIT_TAG, MODE_TAG, USER_TAG},
    models::GameMode,
};
use std::collections::HashMap;

/// Request struct to retrieve a user's recent scores. An instance __must__ contains either a user id or a username
pub struct UserRecentRequest<'n> {
    pub user_id: Option<u32>,
    pub username: Option<&'n str>,
    pub mode: Option<GameMode>,
    pub limit: Option<u32>,
}

impl<'n> Request for UserRecentRequest<'n> {
    fn add_args(self, args: &mut HashMap<String, String>) -> RequestType {
        if let Some(id) = self.user_id {
            args.insert(USER_TAG.to_string(), id.to_string());
        } else if let Some(name) = self.username {
            args.insert(USER_TAG.to_string(), name.to_owned().replace(" ", "%"));
        }
        if let Some(mode) = self.mode {
            args.insert(MODE_TAG.to_string(), (mode as u8).to_string());
        }
        if let Some(limit) = self.limit {
            args.insert(LIMIT_TAG.to_owned(), limit.to_string());
        }
        RequestType::UserRecent
    }
}

impl<'n> UserRecentRequest<'n> {
    /// Construct a `UserRecentRequest` via user id
    pub fn with_user_id(id: u32) -> Self {
        Self {
            user_id: Some(id),
            username: None,
            mode: None,
            limit: None,
        }
    }

    /// Construct a `UserRecentRequest` via username
    pub fn with_username(name: &'n str) -> Self {
        Self {
            user_id: None,
            username: Some(name),
            mode: None,
            limit: None,
        }
    }

    /// Specify a game mode for the request
    pub fn mode(self, mode: GameMode) -> Self {
        Self {
            user_id: self.user_id,
            username: self.username,
            mode: Some(mode),
            limit: self.limit,
        }
    }

    /// Specify a limit for the amount of retrieved scores. Must be at most 50, defaults to 10
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
