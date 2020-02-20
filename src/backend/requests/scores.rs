use crate::{
    backend::requests::{
        Request, LIMIT_TAG, MAP_TAG, MODE_TAG, MODS_TAG, SCORE_ENDPOINT, TYPE_TAG, USER_TAG,
    },
    models::{GameMode, GameMods, Score},
    Osu, OsuError, OsuResult,
};

use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
/// Request struct to retrieve scores of a beatmap.
/// An instance __must__ contains a beatmap id.
pub struct ScoreRequest {
    map_id: Option<u32>,
    user_id: Option<u32>,
    username: Option<String>,
    mode: Option<GameMode>,
    mods: Option<u32>,
    limit: Option<u32>,
}

impl ScoreRequest {
    /// Construct a `ScoreRequest` via beatmap id
    pub fn with_map_id(id: u32) -> Self {
        Self {
            map_id: Some(id),
            user_id: None,
            username: None,
            mode: None,
            mods: None,
            limit: None,
        }
    }

    /// Specify a user id to only get scores from that user.
    pub fn user_id(self, id: u32) -> Self {
        Self {
            map_id: self.map_id,
            user_id: Some(id),
            username: self.username,
            mode: self.mode,
            mods: self.mods,
            limit: self.limit,
        }
    }

    /// Specify a username to only get scores from that user.
    pub fn username(self, name: &str) -> Self {
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: Some(name.to_owned()),
            mode: self.mode,
            mods: self.mods,
            limit: self.limit,
        }
    }

    /// Specify a game mode for the request
    pub fn mode(self, mode: GameMode) -> Self {
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: self.username,
            mode: Some(mode),
            mods: self.mods,
            limit: self.limit,
        }
    }

    /// Specify enabled mods for the retrieved scores
    pub fn mods(self, mods: &GameMods) -> Self {
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            mods: Some(mods.get_bits()),
            limit: self.limit,
        }
    }

    /// Specify a limit for the amount of retrieved scores. Must be at most 100, defaults to 50.
    /// Only matters if neither user id nor username is specified.
    pub fn limit(self, limit: u32) -> Self {
        Self {
            map_id: self.map_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            mods: self.mods,
            limit: Some(limit),
        }
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
        let url = self.get_url(SCORE_ENDPOINT);
        osu.send_request(url).await
    }

    /// Asynchronously send the score request and await the parsed `Score`.
    /// If the API's response contains more than one score, the method will
    /// return the last one.
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
    /// let score: Score = request.queue_single(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue_single(self, osu: &Osu) -> OsuResult<Score> {
        self.queue(osu)
            .await?
            .pop()
            .ok_or_else(|| OsuError::NoResults("Score".to_owned()))
    }
}

impl Request for ScoreRequest {
    fn prepare_args<'s>(&self) -> HashMap<&'s str, String> {
        let mut args = HashMap::new();
        if let Some(id) = self.map_id {
            args.insert(MAP_TAG, id.to_string());
        }
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
        if let Some(mods) = self.mods {
            args.insert(MODS_TAG, mods.to_string());
        }
        if let Some(limit) = self.limit {
            args.insert(LIMIT_TAG, limit.to_string());
        }
        args
    }
}
