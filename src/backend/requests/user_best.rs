use super::{API_BASE, LIMIT_TAG, MODE_TAG, TYPE_TAG, USER_TAG};
use crate::{
    models::{GameMode, Score},
    Osu, OsuError, OsuResult,
};

use reqwest::Url;

const USER_BEST_ENDPOINT: &str = "api/get_user_best";

#[derive(Clone, Eq, PartialEq, Debug)]
/// Request struct to retrieve a user's best scores.
/// An instance __must__ contain either a user id or a username
pub struct BestRequest {
    url: Url,
}

impl BestRequest {
    /// Construct a `BestRequest` via user id
    pub fn with_user_id(id: u32) -> Self {
        let mut url =
            Url::parse_with_params(API_BASE, &[(TYPE_TAG, "id"), (USER_TAG, &id.to_string())])
                .unwrap();
        url.set_path(USER_BEST_ENDPOINT);
        Self { url }
    }

    /// Construct a `BestRequest` via username
    pub fn with_username(name: impl AsRef<str>) -> OsuResult<Self> {
        let mut url = Url::parse_with_params(
            API_BASE,
            &[
                (TYPE_TAG, "string"),
                (USER_TAG, &name.as_ref().replace(" ", "+")),
            ],
        )
        .map_err(|_| OsuError::ParseUrl)?;
        url.set_path(USER_BEST_ENDPOINT);
        Ok(Self { url })
    }

    /// Specify a game mode for the request
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(MODE_TAG, &(mode as u8).to_string());
        self
    }

    /// Specify a limit for the amount of retrieved scores. Must be at most 100, defaults to 10
    pub fn limit(mut self, limit: u32) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(LIMIT_TAG, &limit.min(100).to_string());
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
    /// let osu = Osu::new("osu_api_key");
    /// let request: BestRequest = BestRequest::with_username("Badewanne3").unwrap();
    /// let scores: Vec<Score> = request.queue(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue(self, osu: &Osu) -> OsuResult<Vec<Score>> {
        #[cfg(feature = "metrics")]
        {
            let req = crate::backend::api::RequestType::Best;
            osu.send_request_metrics(self.url, req).await
        }

        #[cfg(not(feature = "metrics"))]
        osu.send_request(self.url).await
    }
}
