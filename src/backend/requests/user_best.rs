use crate::models::GameMode;

#[derive(Clone, Eq, PartialEq)]
/// Args struct to retrieve a user's best scores.
/// An instance __must__ contains either a user id or a username
pub struct UserBestArgs {
    pub(crate) user_id: Option<u32>,
    pub(crate) username: Option<String>,
    pub(crate) mode: Option<GameMode>,
    pub(crate) limit: Option<u32>,
}

impl UserBestArgs {
    /// Construct a `UserBestArgs` via user id
    pub fn with_user_id(id: u32) -> Self {
        Self {
            user_id: Some(id),
            username: None,
            mode: None,
            limit: None,
        }
    }

    /// Construct a `UserBestArgs` via username
    pub fn with_username(name: &str) -> Self {
        Self {
            user_id: None,
            username: Some(name.to_owned()),
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

    /// Specify a limit for the amount of retrieved scores. Must be at most 100, defaults to 10
    pub fn limit(self, limit: u32) -> Self {
        Self {
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: Some(limit),
        }
    }
}
