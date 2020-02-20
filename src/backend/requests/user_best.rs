use crate::{
    backend::requests::{Request, LIMIT_TAG, MODE_TAG, TYPE_TAG, USER_BEST_ENDPOINT, USER_TAG},
    models::{GameMode, Score},
    Osu, OsuResult,
};

use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
/// Request struct to retrieve a user's best scores.
/// An instance __must__ contains either a user id or a username
pub struct BestRequest {
    user_id: Option<u32>,
    username: Option<String>,
    mode: Option<GameMode>,
    limit: Option<u32>,
}

impl BestRequest {
    /// Construct a `BestRequest` via user id
    pub fn with_user_id(id: u32) -> Self {
        Self {
            user_id: Some(id),
            username: None,
            mode: None,
            limit: None,
        }
    }

    /// Construct a `BestRequest` via username
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

    /// Asynchronously send the user request and await the parsed `Vec<User>`.
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::BestRequest},
    ///     models::Score,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key");
    /// let request: BestRequest = BestRequest::with_username("Badewanne3");
    /// let scores: Vec<Score> = request.queue(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue(self, osu: &Osu) -> OsuResult<Vec<Score>> {
        let url = self.get_url(USER_BEST_ENDPOINT);
        osu.send_request(url).await
    }
}

impl Request for BestRequest {
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
        if let Some(limit) = self.limit {
            args.insert(LIMIT_TAG, limit.to_string());
        }
        args
    }
}
