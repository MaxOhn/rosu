use crate::{
    backend::requests::{Request, RequestType, EVENT_DAYS_TAG, MODE_TAG, USER_TAG},
    models::GameMode,
};
use std::collections::HashMap;

/// Request type to retrieve users. An instance __must__ contains either a user id or a username
pub struct UserRequest<'n> {
    user_id: Option<u32>,
    username: Option<&'n str>,
    mode: Option<GameMode>,
    event_days: Option<u32>,
}

impl<'n> Request for UserRequest<'n> {
    fn add_args(self, args: &mut HashMap<String, String>) -> (RequestType, bool) {
        if let Some(id) = self.user_id {
            args.insert(USER_TAG.to_owned(), id.to_string());
        } else if let Some(name) = self.username {
            args.insert(USER_TAG.to_owned(), name.to_owned().replace(" ", "%"));
        }
        if let Some(mode) = self.mode {
            args.insert(MODE_TAG.to_owned(), (mode as u8).to_string());
        }
        if let Some(amount) = self.event_days {
            args.insert(EVENT_DAYS_TAG.to_owned(), amount.to_string());
        }
        (RequestType::User, false)
    }
}

impl<'n> UserRequest<'n> {
    /// Construct a `UserRequest` via user id
    pub fn with_user_id(id: u32) -> Self {
        Self {
            user_id: Some(id),
            username: None,
            mode: None,
            event_days: None,
        }
    }

    /// Construct a `UserRequest` via username
    pub fn with_username(name: &'n str) -> Self {
        Self {
            user_id: None,
            username: Some(name),
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
