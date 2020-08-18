use super::{API_BASE, LIMIT_TAG, MAP_TAG, MODE_TAG, MODS_TAG, TYPE_TAG, USER_TAG};
use crate::{
    models::{GameMode, GameMods, Score},
    Osu, OsuResult,
};

use reqwest::Url;

const SCORE_ENDPOINT: &str = "api/get_scores";

#[derive(Clone, Eq, PartialEq, Debug)]
/// Request struct to retrieve scores of a beatmap.
/// An instance __must__ contain a beatmap id.
///
/// **Don't forget to add the mode** if the given beatmap is no osu!standard map.
pub struct ScoreRequest {
    url: Url,
}

impl ScoreRequest {
    /// Construct a `ScoreRequest` via beatmap id
    pub fn with_map_id(id: u32) -> Self {
        let mut url = Url::parse_with_params(API_BASE, &[(MAP_TAG, &id.to_string())]).unwrap();
        url.set_path(SCORE_ENDPOINT);
        Self { url }
    }

    /// Specify a user id to only get scores from that user.
    pub fn user_id(mut self, id: u32) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(TYPE_TAG, "id")
            .append_pair(USER_TAG, &id.to_string());
        self
    }

    /// Specify a username to only get scores from that user.
    pub fn username(mut self, name: impl AsRef<str>) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(TYPE_TAG, "string")
            .append_pair(USER_TAG, &name.as_ref().replace(" ", "+"));
        self
    }

    /// Specify a game mode for the request
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(MODE_TAG, &(mode as u8).to_string());
        self
    }

    /// Specify enabled mods for the retrieved scores
    pub fn mods(mut self, mods: GameMods) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(MODS_TAG, &mods.bits().to_string());
        self
    }

    /// Specify a limit for the amount of retrieved scores. Must be at most 100, defaults to 50.
    /// Only matters if neither user id nor username is specified.
    pub fn limit(mut self, limit: u32) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(LIMIT_TAG, &limit.min(100).to_string());
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
        match (cfg!(feature = "metrics"), cfg!(feature = "cache")) {
            (true, true) => {
                #[cfg(all(feature = "metrics", feature = "cache"))]
                {
                    let req = crate::backend::api::RequestType::Score;
                    let cached = osu.cached.contains(crate::backend::OsuCached::Score);
                    osu.send_request_metrics_cached(self.url, req, cached).await
                }
                #[cfg(not(all(feature = "metrics", feature = "cache")))]
                unreachable!()
            }
            (true, false) => {
                #[cfg(all(feature = "metrics", not(feature = "cache")))]
                {
                    let req = crate::backend::api::RequestType::Score;
                    osu.send_request_metrics(self.url, req).await
                }
                #[cfg(not(all(feature = "metrics", not(feature = "cache"))))]
                unreachable!()
            }
            (false, true) => {
                #[cfg(all(not(feature = "metrics"), feature = "cache"))]
                {
                    let cached = osu.cached.contains(crate::backend::OsuCached::Beatmap);
                    osu.send_request_cached(self.url, cached).await
                }
                #[cfg(not(all(not(feature = "metrics"), feature = "cache")))]
                unreachable!()
            }
            (false, false) => {
                #[cfg(not(any(feature = "metrics", feature = "cache")))]
                {
                    osu.send_request(self.url).await
                }
                #[cfg(any(feature = "metrics", feature = "cache"))]
                unreachable!()
            }
        }
    }

    /// Asynchronously send the score request and await the parsed [`Score`].
    ///
    /// If the API's response contains more than one score, the method will
    /// return the last one.
    ///
    /// If the API response contains no scores, the method will return `None`.
    ///
    /// [`Score`]: ../models/struct.Score.html
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
    /// let osu = Osu::new("osu_api_key".to_owned());
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
