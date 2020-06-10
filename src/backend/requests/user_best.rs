use crate::{
    backend::requests::{Request, LIMIT_TAG, MODE_TAG, TYPE_TAG, USER_BEST_ENDPOINT, USER_TAG},
    models::{GameMode, Score},
    Osu, OsuResult,
};

use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
/// Request struct to retrieve a user's best scores.
/// An instance __must__ contain either a user id or a username
pub struct BestRequest<'s> {
    args: HashMap<&'s str, String>,
}

impl<'s> BestRequest<'s> {
    /// Construct a `BestRequest` via user id
    pub fn with_user_id(id: u32) -> Self {
        let mut args = HashMap::new();
        args.insert(USER_TAG, id.to_string());
        args.insert(TYPE_TAG, "id".to_string());
        Self { args }
    }

    /// Construct a `BestRequest` via username
    pub fn with_username(name: &str) -> Self {
        let mut args = HashMap::new();
        args.insert(USER_TAG, name.replace(" ", "+"));
        args.insert(TYPE_TAG, "string".to_string());
        Self { args }
    }

    /// Specify a game mode for the request
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.args.insert(MODE_TAG, (mode as u8).to_string());
        self
    }

    /// Specify a limit for the amount of retrieved scores. Must be at most 100, defaults to 10
    pub fn limit(mut self, limit: u32) -> Self {
        self.args.insert(LIMIT_TAG, limit.to_string());
        self
    }

    /// Asynchronously send the user-best request and await the parsed `Vec<Score>`.
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
    /// let osu = Osu::new("osu_api_key".to_owned());
    /// let request: BestRequest = BestRequest::with_username("Badewanne3");
    /// let scores: Vec<Score> = request.queue(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue(self, osu: &Osu) -> OsuResult<Vec<Score>> {
        let url = Request::create_url(USER_BEST_ENDPOINT, self.args);
        osu.send_request(url).await
    }
}
