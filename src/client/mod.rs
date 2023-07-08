mod builder;

pub use builder::OsuBuilder;

use crate::{
    error::ApiError,
    ratelimit::RateLimiter,
    request::{
        GetBeatmap, GetBeatmaps, GetMatch, GetScore, GetScores, GetUser, GetUserBest,
        GetUserRecent, Request, UserIdentification,
    },
    routing::Route,
    OsuError, OsuResult,
};

#[cfg(feature = "metrics")]
use crate::metrics::Metrics;

use bytes::Bytes;
use reqwest::{Client, Method, Response, StatusCode};
use std::sync::Arc;

#[cfg(feature = "metrics")]
use prometheus::IntCounterVec;

const USER_AGENT: &str = concat!(
    "(",
    env!("CARGO_PKG_HOMEPAGE"),
    ", ",
    env!("CARGO_PKG_VERSION"),
    ") rosu"
);

pub(crate) struct OsuRef {
    http: Client,
    ratelimiter: RateLimiter,
    api_key: String,
    #[cfg(feature = "metrics")]
    pub(crate) metrics: Metrics,
}

/// The main osu client.
/// Cheap to clone.
pub struct Osu(pub(crate) Arc<OsuRef>);

impl Osu {
    /// Create a new [`Osu`] client.
    pub fn new(api_key: impl Into<String>) -> Self {
        let ratelimiter = RateLimiter::new(15, 1);

        let osu = OsuRef {
            http: Client::new(),
            api_key: api_key.into(),
            ratelimiter,
            #[cfg(feature = "metrics")]
            metrics: Metrics::new(),
        };

        Self(Arc::new(osu))
    }

    /// Create a new builder to build an [`Osu`] struct.
    pub fn builder(api_key: impl Into<String>) -> OsuBuilder {
        OsuBuilder::new(api_key)
    }

    /// Request an optional [`User`](crate::model::User).
    pub fn user(&self, user: impl Into<UserIdentification>) -> GetUser<'_> {
        GetUser::new(self, user)
    }

    /// Request an optional [`Beatmap`](crate::model::Beatmap).
    pub fn beatmap(&self) -> GetBeatmap<'_> {
        GetBeatmap::new(self)
    }

    /// Request a vec of [`Beatmap`](crate::model::Beatmap)s.
    pub fn beatmaps(&self) -> GetBeatmaps<'_> {
        GetBeatmaps::new(self)
    }

    /// Request the [`Match`](crate::model::Match) with the given `match_id`.
    pub fn osu_match(&self, match_id: u32) -> GetMatch<'_> {
        GetMatch::new(self, match_id)
    }

    /// Request an optional [`Score`](crate::model::Score) on the given `map_id`.
    pub fn score(&self, map_id: u32) -> GetScore<'_> {
        GetScore::new(self, map_id)
    }

    /// Request a vec of [`Score`](crate::model::Score)s on the given `map_id`.
    pub fn scores(&self, map_id: u32) -> GetScores<'_> {
        GetScores::new(self, map_id)
    }

    /// Request a vec of [`Score`](crate::model::Score)s namely the top scores of the given user.
    pub fn top_scores(&self, user: impl Into<UserIdentification>) -> GetUserBest<'_> {
        GetUserBest::new(self, user)
    }

    /// Request a vec of [`Score`](crate::model::Score)s namely the most recent scores of the given user.
    pub fn recent_scores(&self, user: impl Into<UserIdentification>) -> GetUserRecent<'_> {
        GetUserRecent::new(self, user)
    }

    #[cfg(feature = "metrics")]
    /// Returns an [`IntCounterVec`] from [`prometheus`] containing a counter for each request type.
    ///
    /// [`IntCounterVec`]: crate::prelude::IntCounterVec
    /// [`prometheus`]: https://crates.io/crates/prometheus
    pub fn metrics(&self) -> IntCounterVec {
        self.0.metrics.counters.clone()
    }

    pub(crate) async fn request_bytes(&self, route: Route) -> OsuResult<Bytes> {
        let req = Request::from(route);
        let resp = self.make_request(req).await?;
        resp.bytes().await.map_err(OsuError::ChunkingResponse)
    }

    async fn make_request(&self, req: Request) -> OsuResult<Response> {
        let resp = self.raw(req).await?;
        let status = resp.status();

        match status {
            StatusCode::OK => return Ok(resp),
            StatusCode::SERVICE_UNAVAILABLE => {
                let body = resp.text().await.ok();
                return Err(OsuError::ServiceUnavailable(body));
            }
            StatusCode::TOO_MANY_REQUESTS => warn!("429 response: {:?}", resp),
            _ => {}
        }

        let bytes = resp.bytes().await.map_err(OsuError::ChunkingResponse)?;
        let body = String::from_utf8_lossy(bytes.as_ref()).into_owned();

        let error = match serde_json::from_str::<ApiError>(body.as_ref()) {
            Ok(error) => error,
            Err(source) => return Err(OsuError::Parsing { body, source }),
        };

        Err(OsuError::Response {
            body,
            error,
            status,
        })
    }

    async fn raw(&self, request: Request) -> OsuResult<Response> {
        let mut url = String::with_capacity(26 + request.0.len() + self.0.api_key.len());
        url.push_str("https://osu.ppy.sh/api/");
        url.push_str(&request.0);

        self.0.ratelimiter.await_access().await;

        debug!("URL: {:?}", url);

        url.push_str("&k=");
        url.push_str(&self.0.api_key);

        let mut builder = self.0.http.request(Method::GET, &url);

        builder = builder.header("User-Agent", USER_AGENT);
        let resp = builder.send().await.map_err(OsuError::RequestError)?;

        Ok(resp)
    }
}
