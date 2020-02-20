use crate::{
    backend::requests::{Request, MATCH_ENDPOINT, MP_TAG},
    models::Match,
    Osu, OsuResult,
};

use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
/// Request struct to retrieve matches.
pub struct MatchRequest {
    pub(crate) match_id: u32,
}

impl MatchRequest {
    /// Construct a `MatchRequest` via match id
    pub fn with_match_id(id: u32) -> Self {
        Self { match_id: id }
    }

    /// Asynchronously send the match request and await the parsed `Match`.
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
        let url = self.get_url(MATCH_ENDPOINT);
        osu.send_request(url).await
    }
}

impl Request for MatchRequest {
    fn prepare_args<'s>(&self) -> HashMap<&'s str, String> {
        let mut args = HashMap::new();
        args.insert(MP_TAG, self.match_id.to_string());
        args
    }
}
