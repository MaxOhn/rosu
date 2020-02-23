use crate::{
    backend::requests::{
        Request, LIMIT_TAG, MAP_TAG, MODE_TAG, MODS_TAG, SCORE_ENDPOINT, TYPE_TAG, USER_TAG,
    },
    models::{GameMode, GameMods, Score},
    Osu, OsuResult,
};

use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
/// Request struct to retrieve scores of a beatmap.
/// An instance __must__ contain a beatmap id.
///
/// **Don't forget to add the mode** if the given beatmap is no osu!standard map.
pub struct ScoreRequest<'s> {
    args: HashMap<&'s str, String>,
}

impl<'s> ScoreRequest<'s> {
    /// Construct a `ScoreRequest` via beatmap id
    pub fn with_map_id(id: u32) -> Self {
        let mut args = HashMap::new();
        args.insert(MAP_TAG, id.to_string());
        Self { args }
    }

    /// Specify a user id to only get scores from that user.
    pub fn user_id(mut self, id: u32) -> Self {
        self.args.insert(USER_TAG, id.to_string());
        self.args.insert(TYPE_TAG, "id".to_string());
        self
    }

    /// Specify a username to only get scores from that user.
    pub fn username(mut self, name: &str) -> Self {
        self.args.insert(USER_TAG, name.replace(" ", "+"));
        self.args.insert(TYPE_TAG, "string".to_string());
        self
    }

    /// Specify a game mode for the request
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.args.insert(MODE_TAG, (mode as u8).to_string());
        self
    }

    /// Specify enabled mods for the retrieved scores
    pub fn mods(mut self, mods: &GameMods) -> Self {
        self.args.insert(MODS_TAG, mods.as_bits().to_string());
        self
    }

    /// Specify a limit for the amount of retrieved scores. Must be at most 100, defaults to 50.
    /// Only matters if neither user id nor username is specified.
    pub fn limit(mut self, limit: u32) -> Self {
        self.args.insert(LIMIT_TAG, limit.to_string());
        self
    }

    /// Asynchronously send the score request and await the parsed `Vec<Score>`.
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::ScoreRequest},
    ///     models::Score,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key");
    /// let request: ScoreRequest = ScoreRequest::with_map_id(905576);
    /// let scores: Vec<Score> = request.queue(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue(self, osu: &Osu) -> OsuResult<Vec<Score>> {
        let url = Request::create_url(SCORE_ENDPOINT, self.args);
        osu.send_request(url).await
    }

    /// Asynchronously send the score request and await the parsed [Score][score].
    ///
    /// If the API's response contains more than one score, the method will
    /// return the last one.
    ///
    /// If the API response contains no scores, the method will return `None`.
    ///
    /// [score]: ../models/struct.Score.html
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::ScoreRequest},
    ///     models::Score,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key");
    /// let request: ScoreRequest = ScoreRequest::with_map_id(905576);
    /// let score: Option<Score> = request.queue_single(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue_single(self, osu: &Osu) -> OsuResult<Option<Score>> {
        Ok(self.queue(osu).await?.pop())
    }
}
