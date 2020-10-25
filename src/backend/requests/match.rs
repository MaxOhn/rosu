use crate::{backend::requests::API_BASE, models::Match, Osu, OsuError, OsuResult};

use reqwest::Url;

const MP_TAG: &str = "mp";
const MATCH_ENDPOINT: &str = "api/get_match";

#[derive(Clone, Eq, PartialEq, Debug)]
/// Request struct to retrieve matches.
pub struct MatchRequest {
    url: Url,
}

impl MatchRequest {
    /// Construct a `MatchRequest` via match id
    pub fn with_match_id(id: u32) -> Self {
        let url = Url::parse(&format!(
            "{}/{}?{}={}",
            API_BASE, MATCH_ENDPOINT, MP_TAG, id
        ))
        .unwrap();
        Self { url }
    }

    /// Asynchronously send the match request and await the parsed [`Match`].
    ///
    /// [`Match`]: ../models/struct.Match.html
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::MatchRequest},
    ///     models::Match,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key");
    /// let request: MatchRequest = MatchRequest::with_match_id(58494587);
    /// let osu_match: Match = request.queue_single(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue_single(self, osu: &Osu) -> OsuResult<Match> {
        let result = match (cfg!(feature = "metrics"), cfg!(feature = "cache")) {
            (true, true) => {
                #[cfg(all(feature = "metrics", feature = "cache"))]
                {
                    let req = crate::backend::api::RequestType::Match;
                    let cached = osu.cached.contains(crate::backend::OsuCached::Match);
                    osu.send_request_metrics_cached(self.url, req, cached).await
                }
                #[cfg(not(all(feature = "metrics", feature = "cache")))]
                unreachable!()
            }
            (true, false) => {
                #[cfg(all(feature = "metrics", not(feature = "cache")))]
                {
                    let req = crate::backend::api::RequestType::Match;
                    osu.send_request_metrics(self.url, req).await
                }
                #[cfg(not(all(feature = "metrics", not(feature = "cache"))))]
                unreachable!()
            }
            (false, true) => {
                #[cfg(all(not(feature = "metrics"), feature = "cache"))]
                {
                    let cached = osu.cached.contains(crate::backend::OsuCached::Match);
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
        };
        result.map_err(|_| OsuError::InvalidMultiplayerMatch)
    }
}
