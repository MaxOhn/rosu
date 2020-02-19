use crate::models::GameMode;

#[derive(Clone, Eq, PartialEq)]
/// Args struct to retrieve users.
/// An instance __must__ contains either a user id or a username
pub struct UserArgs {
    pub(crate) user_id: Option<u32>,
    pub(crate) username: Option<String>,
    pub(crate) mode: Option<GameMode>,
    pub(crate) event_days: Option<u32>,
}

impl UserArgs {
    /// Construct a `UserArgs` via user id
    pub fn with_user_id(id: u32) -> Self {
        Self {
            user_id: Some(id),
            username: None,
            mode: None,
            event_days: None,
        }
    }

    /// Construct a `UserArgs` via username
    pub fn with_username(name: &str) -> Self {
        Self {
            user_id: None,
            username: Some(name.to_owned()),
            mode: None,
            event_days: None,
        }
    }

    /// Specify a game mode for the request
    pub fn mode(self, mode: GameMode) -> Self {
        Self {
            user_id: self.user_id,
            username: self.username,
            mode: Some(mode),
            event_days: self.event_days,
        }
    }

    /// Specify event days for the request.
    ///
    /// From osu!api repo: Max number of days between now and last event date. Range of 1-31. Optional, default value is 1
    pub fn event_days(self, amount: u32) -> Self {
        assert!(0 < amount && amount < 32);
        Self {
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            event_days: Some(amount),
        }
    }
}
