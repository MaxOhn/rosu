mod builder;

pub use builder::OsuBuilder;

#[cfg(feature = "cache")]
pub use cached::OsuCached;

use crate::{
    error::APIError,
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
use reqwest::{header::HeaderValue, Client, Method, Response, StatusCode};
use std::sync::Arc;

#[cfg(feature = "metrics")]
use prometheus::IntCounterVec;

#[cfg(feature = "cache")]
use darkredis::ConnectionPool;

pub(crate) struct OsuRef {
    http: Client,
    ratelimiter: RateLimiter,
    api_key: String,
    #[cfg(feature = "metrics")]
    pub(crate) metrics: Metrics,
    #[cfg(feature = "cache")]
    redis: ConnectionPool,
    #[cfg(feature = "cache")]
    duration: u32,
    #[cfg(feature = "cache")]
    pub(crate) cached: OsuCached,
}

/// The main osu client.
/// Cheap to clone.
pub struct Osu(pub(crate) Arc<OsuRef>);

impl Osu {
    #[cfg(not(feature = "cache"))]
    /// Create a new osu client.
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
    ///
    /// [`Osu`]: struct.Osu.html
    #[cfg(not(feature = "cache"))]
    pub fn builder(api_key: impl Into<String>) -> OsuBuilder {
        OsuBuilder::new(api_key)
    }

    #[cfg(feature = "cache")]
    /// Create a new osu client that caches the specified structs for a default duration of 300 seconds.
    ///
    /// Requires the api key and a [`ConnectionPool`] from [darkredis](https://crates.io/crates/darkredis).
    ///
    /// [`ConnectionPool`]: prelude/struct.ConnectionPool.html
    pub fn new(api_key: impl Into<String>, pool: ConnectionPool, cached: OsuCached) -> Self {
        let ratelimiter = RateLimiter::new(15, 1);

        let osu = OsuRef {
            http: Client::new(),
            api_key: api_key.into(),
            ratelimiter,
            redis: pool,
            duration: 300,
            cached,
            #[cfg(feature = "metrics")]
            metrics: Metrics::new(),
        };

        Self(Arc::new(osu))
    }

    /// Create a new builder to build an [`Osu`] struct.
    ///
    /// Requires the api key and a [`ConnectionPool`] from [darkredis](https://crates.io/crates/darkredis).
    ///
    /// [`Osu`]: struct.Osu.html
    /// [`ConnectionPool`]: prelude/struct.ConnectionPool.html
    #[cfg(feature = "cache")]
    pub fn builder(api_key: impl Into<String>, pool: ConnectionPool) -> OsuBuilder {
        OsuBuilder::new(api_key, pool)
    }

    /// Request an `Option<User>`.
    pub fn user(&self, user: impl Into<UserIdentification>) -> GetUser {
        GetUser::new(self, user)
    }

    /// Request an `Option<Beatmap>`.
    pub fn beatmap(&self) -> GetBeatmap {
        GetBeatmap::new(self)
    }

    /// Request a `Vec<Beatmap>`.
    pub fn beatmaps(&self) -> GetBeatmaps {
        GetBeatmaps::new(self)
    }

    /// Request the [`Match`] with the given `match_id`.
    ///
    /// [`Match`]: struct.Match.html
    pub fn osu_match(&self, match_id: u32) -> GetMatch {
        GetMatch::new(self, match_id)
    }

    /// Request an `Option<Score>` on the given `map_id`.
    pub fn score(&self, map_id: u32) -> GetScore {
        GetScore::new(self, map_id)
    }

    /// Request a `Vec<Score>` on the given `map_id`.
    pub fn scores(&self, map_id: u32) -> GetScores {
        GetScores::new(self, map_id)
    }

    /// Request a `Vec<Score>` namely the top scores of the given user.
    pub fn top_scores(&self, user: impl Into<UserIdentification>) -> GetUserBest {
        GetUserBest::new(self, user)
    }

    /// Request a `Vec<Score>` namely the most recent scores of the given user.
    pub fn recent_scores(&self, user: impl Into<UserIdentification>) -> GetUserRecent {
        GetUserRecent::new(self, user)
    }

    #[cfg(feature = "metrics")]
    /// Returns an [`IntCounterVec`] from [prometheus](https://crates.io/crates/prometheus) containing a counter for each request type.
    ///
    /// [`IntCounterVec`]: prelude/type.IntCounterVec.html
    pub fn metrics(&self) -> IntCounterVec {
        self.0.metrics.counters.clone()
    }

    #[cfg(not(feature = "cache"))]
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

        let error = match serde_json::from_str::<APIError>(body.as_ref()) {
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

        let user_agent = HeaderValue::from_static(concat!(
            "(",
            env!("CARGO_PKG_HOMEPAGE"),
            ", ",
            env!("CARGO_PKG_VERSION"),
            ")",
        ));
        builder = builder.header("User-Agent", user_agent);
        let resp = builder.send().await.map_err(OsuError::RequestError)?;

        Ok(resp)
    }

    #[cfg(feature = "cache")]
    pub(crate) async fn request_bytes(&self, route: Route, cached: OsuCached) -> OsuResult<Bytes> {
        let key = match self.check_cache(&route, cached).await {
            Ok(bytes) => return Ok(bytes),
            Err(key) => key,
        };
        let req = Request::from(route);
        let resp = self.make_request(req).await?;
        let bytes = resp.bytes().await.map_err(OsuError::ChunkingResponse)?;
        self.insert_cache(key, bytes.as_ref()).await;

        Ok(bytes)
    }

    #[cfg(feature = "cache")]
    async fn check_cache(&self, route: &Route, cached: OsuCached) -> Result<Bytes, Option<String>> {
        if !self.0.cached.contains(cached) {
            return Err(None);
        }
        match serde_json::to_string(route) {
            Ok(key) => {
                let mut conn = self.0.redis.get().await;

                if let Ok(Some(bytes)) = conn.get(&key).await {
                    #[cfg(feature = "metrics")]
                    self.0.metrics.cached.inc();
                    debug!("Found in cache: {}", key);

                    return Ok(bytes.into());
                } else {
                    return Err(Some(key));
                }
            }
            Err(why) => debug!("Error while serializing route {:?}: {}", route, why),
        }
        Err(None)
    }

    #[cfg(feature = "cache")]
    async fn insert_cache(&self, key: Option<String>, bytes: &[u8]) {
        if let Some(key) = key {
            let mut conn = self.0.redis.get().await;
            let set_fut = conn.set_and_expire_seconds(key, bytes.as_ref(), self.0.duration);

            if let Err(why) = set_fut.await {
                debug!("Error while inserting bytes into cache: {}", why);
            }
        }
    }
}

#[cfg(feature = "cache")]
pub(crate) mod cached {
    #![allow(non_upper_case_globals)]
    bitflags! {
        #[derive(Default)]
        /// Bitflags to decide which structs to cache.
        /// Before requesting from the API, the client will check for the value in the cache.
        pub struct OsuCached: u8 {
            const User = 1;
            const Score = 2;
            const Beatmap = 4;
            const Match = 8;
        }
    }
}
