use super::{API_BASE, LIMIT_TAG, MODE_TAG, TYPE_TAG, USER_TAG};
use crate::{
    models::{GameMode, Score},
    Osu, OsuError, OsuResult,
};

use reqwest::Url;

const USER_RECENT_ENDPOINT: &str = "api/get_user_recent";

#[derive(Clone, Eq, PartialEq, Debug)]
/// Request struct to retrieve a user's recent scores.
/// An instance __must__ contain either a user id or a username
pub struct RecentRequest {
    url: Url,
}

impl RecentRequest {
    /// Construct a `RecentRequest` via user id
    pub fn with_user_id(id: u32) -> Self {
        let mut url =
            Url::parse_with_params(API_BASE, &[(TYPE_TAG, "id"), (USER_TAG, &id.to_string())])
                .unwrap();
        url.set_path(USER_RECENT_ENDPOINT);
        Self { url }
    }

    /// Construct a `RecentRequest` via username
    pub fn with_username(name: impl AsRef<str>) -> OsuResult<Self> {
        let mut url = Url::parse_with_params(
            API_BASE,
            &[
                (TYPE_TAG, "string"),
                (USER_TAG, &name.as_ref().replace(" ", "+")),
            ],
        )
        .map_err(|_| OsuError::ParseUrl)?;
        url.set_path(USER_RECENT_ENDPOINT);
        Ok(Self { url })
    }

    /// Specify a game mode for the request
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(MODE_TAG, &(mode as u8).to_string());
        self
    }

    /// Specify a limit for the amount of retrieved scores. Must be at most 50, defaults to 10
    pub fn limit(mut self, limit: u32) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(LIMIT_TAG, &limit.min(50).to_string());
        self
    }

    /// Asynchronously send the user-recent request and await the parsed `Vec<Score>`.
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::RecentRequest},
    ///     models::Score,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key");
    /// let request: RecentRequest = RecentRequest::with_username("Badewanne3").unwrap();
    /// let scores: Vec<Score> = request.queue(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue(self, osu: &Osu) -> OsuResult<Vec<Score>> {
        #[cfg(feature = "metrics")]
        {
            let req = crate::backend::api::RequestType::Recent;
            osu.send_request_metrics(self.url, req).await
        }

        #[cfg(not(feature = "metrics"))]
        osu.send_request(self.url).await
    }
}
