use crate::{
    backend::requests::{Request, MATCH_ENDPOINT, MP_TAG},
    models::Match,
    Osu, OsuResult,
};

use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
/// Request struct to retrieve matches.
pub struct MatchRequest {
    args: HashMap<&'static str, String>,
}

impl MatchRequest {
    /// Construct a `MatchRequest` via match id
    pub fn with_match_id(id: u32) -> Self {
        let mut args = HashMap::new();
        args.insert(MP_TAG, id.to_string());
        Self { args }
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
    /// let osu = Osu::new("osu_api_key".to_owned());
    /// let request: MatchRequest = MatchRequest::with_match_id(58494587);
    /// let osu_match: Match = request.queue_single(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue_single(self, osu: &Osu) -> OsuResult<Match> {
        let url = Request::create_url(MATCH_ENDPOINT, self.args);

        #[cfg(feature = "metrics")]
        {
            let req = crate::backend::api::RequestType::Match;
            osu.send_request_metrics(url, req).await
        }

        #[cfg(not(feature = "metrics"))]
        osu.send_request(url).await
    }
}
