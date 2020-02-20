use crate::{
    backend::requests::{Request, EVENT_DAYS_TAG, MODE_TAG, TYPE_TAG, USER_ENDPOINT, USER_TAG},
    models::{GameMode, User},
    Osu, OsuResult,
};

use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
/// Request struct to retrieve users.
/// An instance __must__ contains either a user id or a username
pub struct UserRequest {
    user_id: Option<u32>,
    username: Option<String>,
    mode: Option<GameMode>,
    event_days: Option<u32>,
}

impl UserRequest {
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
        Self {
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            event_days: Some(amount),
        }
    }

    /// Asynchronously send the user request and await the parsed `Vec<User>`.
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::UserRequest},
    ///     models::User,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key");
    /// let request: UserRequest = UserRequest::with_username("Badewanne3");
    /// let users: Vec<User> = request.queue(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue(self, osu: &Osu) -> OsuResult<Vec<User>> {
        let url = self.get_url(USER_ENDPOINT);
        osu.send_request(url).await
    }

    /// Asynchronously send the user request and await the parsed `User`.
    /// If the API's response contains more than one user, the method will
    /// return the last one. If the API response contains no users, the
    /// method will return `None`.
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::UserRequest},
    ///     models::User,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key");
    /// let request: UserRequest = UserRequest::with_username("Badewanne3");
    /// let user: Option<User> = request.queue_single(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue_single(self, osu: &Osu) -> OsuResult<Option<User>> {
        Ok(self.queue(osu).await?.pop())
    }
}

impl Request for UserRequest {
    fn prepare_args<'s>(&self) -> HashMap<&'s str, String> {
        let mut args = HashMap::new();
        if let Some(id) = self.user_id {
            args.insert(USER_TAG, id.to_string());
            args.insert(TYPE_TAG, "id".to_string());
        } else if let Some(name) = &self.username {
            args.insert(USER_TAG, name.replace(" ", "+"));
            args.insert(TYPE_TAG, "string".to_string());
        }
        if let Some(mode) = self.mode {
            args.insert(MODE_TAG, (mode as u8).to_string());
        }
        if let Some(amount) = self.event_days {
            args.insert(EVENT_DAYS_TAG, amount.to_string());
        }
        args
    }
}
