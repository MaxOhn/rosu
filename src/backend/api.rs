use crate::backend::{OsuError, OsuResult};

use futures::TryFutureExt;
use governor::{
    clock::DefaultClock,
    state::{direct::NotKeyed, InMemoryState},
    Quota, RateLimiter,
};
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use std::{fmt::Write, num::NonZeroU32};

#[cfg(feature = "metrics")]
use prometheus::{IntCounterVec, Opts};

#[cfg(feature = "metrics")]
use futures::FutureExt;

#[cfg(feature = "cache")]
use darkredis::ConnectionPool;

#[cfg(feature = "cache")]
use serde::Serialize;

/// The main osu client.
/// Pass this into a `queue` method of some request to retrieve and parse the data.
pub struct Osu {
    client: Client,
    api_key: String,
    ratelimiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
    #[cfg(feature = "metrics")]
    stats: IntCounterVec,
    #[cfg(feature = "cache")]
    redis: ConnectionPool,
    #[cfg(feature = "cache")]
    duration: u32,
}

impl Osu {
    pub(crate) fn prepare_url(&self, mut url: String) -> Url {
        let _ = write!(url, "&k={}", &self.api_key);
        Url::parse(&url).unwrap()
    }

    #[cfg(not(feature = "metrics"))]
    async fn _send_request<T>(&self, url: String) -> OsuResult<T>
    where
        T: DeserializeOwned,
    {
        // Fetch response and deserialize in one go
        debug!("Fetching url {}", url);
        self.ratelimiter.until_ready().await;
        self.client
            .get(self.prepare_url(url))
            .send()
            .and_then(|res| res.bytes())
            .map_ok(|bytes| {
                let parse_result = serde_json::from_slice(&bytes).map_err(|e| {
                    let content = String::from_utf8_lossy(&bytes).into_owned();
                    OsuError::Serde(e, content)
                })?;
                Ok(parse_result)
            })
            .await?
    }

    #[cfg(feature = "metrics")]
    async fn _send_request_metrics<T>(&self, url: String, req: RequestType) -> OsuResult<T>
    where
        T: DeserializeOwned,
    {
        // Fetch response and deserialize in one go
        debug!("Fetching url {}", url);
        self.ratelimiter.until_ready().await;
        self.client
            .get(self.prepare_url(url))
            .send()
            .then(|res| async {
                self.inc_counter(req);
                res
            })
            .and_then(|res| res.bytes())
            .map_ok(|bytes| {
                let parse_result = serde_json::from_slice(&bytes).map_err(|e| {
                    let content = String::from_utf8_lossy(&bytes).into_owned();
                    OsuError::Serde(e, content)
                })?;
                Ok(parse_result)
            })
            .await?
    }

    #[cfg(feature = "metrics")]
    pub fn metrics(&self) -> IntCounterVec {
        self.stats.clone()
    }

    #[cfg(feature = "metrics")]
    fn inc_counter(&self, req: RequestType) {
        let counter_res = match req {
            RequestType::Beatmap => self.stats.get_metric_with_label_values(&["Beatmaps"]),
            RequestType::Best => self.stats.get_metric_with_label_values(&["TopScores"]),
            RequestType::Match => self.stats.get_metric_with_label_values(&["Matches"]),
            RequestType::Recent => self.stats.get_metric_with_label_values(&["RecentScores"]),
            RequestType::Score => self.stats.get_metric_with_label_values(&["Scores"]),
            RequestType::User => self.stats.get_metric_with_label_values(&["Users"]),
        };
        match counter_res {
            Ok(counter) => counter.inc(),
            Err(why) => debug!("Could not get {:?} counter: {}", req, why),
        }
    }
}

#[cfg(feature = "metrics")]
#[derive(Debug)]
pub(crate) enum RequestType {
    Beatmap,
    Best,
    Match,
    Recent,
    Score,
    User,
}

#[cfg(feature = "metrics")]
fn init_stats() -> IntCounterVec {
    let vec = IntCounterVec::new(
        Opts::new("osu_requests", "osu!api request count"),
        &["type"],
    )
    .unwrap();
    let _ = vec.get_metric_with_label_values(&[
        "Beatmaps",
        "TopScores",
        "Matches",
        "RecentScores",
        "Scores",
        "Users",
    ]);
    vec
}

// ###################
// ## Without cache ##
// ###################
#[cfg(not(feature = "cache"))]
impl Osu {
    /// Create a new osu client.
    pub fn new(api_key: impl Into<String>) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(15u32).unwrap());
        let ratelimiter = RateLimiter::direct(quota);
        let client = Client::builder()
            .use_rustls_tls()
            .build()
            .unwrap_or_else(|why| panic!("Could not build reqwest client for osu!: {}", why));
        Osu {
            client,
            api_key: api_key.into(),
            ratelimiter,
            #[cfg(feature = "metrics")]
            stats: init_stats(),
        }
    }

    #[cfg(not(feature = "metrics"))]
    pub(crate) async fn send_request<T>(&self, url: String) -> OsuResult<T>
    where
        T: DeserializeOwned,
    {
        self._send_request(url).await
    }

    #[cfg(feature = "metrics")]
    pub(crate) async fn send_request_metrics<T>(
        &self,
        url: String,
        req: RequestType,
    ) -> OsuResult<T>
    where
        T: DeserializeOwned,
    {
        self._send_request_metrics(url, req).await
    }
}

// ################
// ## With cache ##
// ################
#[cfg(feature = "cache")]
impl Osu {
    /// Create a new osu client.
    ///
    /// darkredis' `ConnectionPool` is cheap to clone.
    ///
    /// `cache_duration_seconds` decides how long values will stay in the cache.
    ///
    /// Keep in mind that e.g. for cached [`User`]s, if their actual total pp change
    /// while being inside the cache, the stored value will not have the actual value,
    /// hence the cache duration should not be too long.
    ///
    /// [`User`]: struct.User.html
    pub fn new(
        api_key: impl Into<String>,
        redis_pool: ConnectionPool,
        cache_duration_seconds: u32,
    ) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(15u32).unwrap());
        let ratelimiter = RateLimiter::direct(quota);
        let client = Client::builder()
            .use_rustls_tls()
            .build()
            .unwrap_or_else(|why| panic!("Could not build reqwest client for osu!: {}", why));
        Osu {
            client,
            api_key: api_key.into(),
            ratelimiter,
            #[cfg(feature = "metrics")]
            stats: init_stats(),
            redis: redis_pool,
            duration: cache_duration_seconds,
        }
    }

    #[cfg(not(feature = "metrics"))]
    pub(crate) async fn send_request<T>(&self, url: String) -> OsuResult<T>
    where
        T: DeserializeOwned + std::fmt::Debug + serde::Serialize,
    {
        if let Some(value) = self.check_cache(&url).await {
            return Ok(value);
        }
        let result = self._send_request(url.clone()).await?;
        self.insert_cache(&url, &result).await;
        Ok(result)
    }

    #[cfg(feature = "metrics")]
    pub(crate) async fn send_request_metrics<T>(
        &self,
        url: String,
        req: RequestType,
    ) -> OsuResult<T>
    where
        T: DeserializeOwned + std::fmt::Debug + serde::Serialize,
    {
        if let Some(value) = self.check_cache(&url).await {
            return Ok(value);
        }
        let result = self._send_request_metrics(url.clone(), req).await?;
        self.insert_cache(&url, &result).await;
        Ok(result)
    }

    async fn check_cache<T>(&self, url: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let mut conn = self.redis.get().await;
        match conn.get(url).await {
            Ok(Some(bytes)) => match serde_json::from_slice(&bytes) {
                Ok(value) => {
                    debug!("Found in cache: {}", url);
                    Some(value)
                }
                Err(why) => {
                    debug!("Error while deserializing cache entry: {}", why);
                    None
                }
            },
            Err(_) | Ok(None) => None,
        }
    }

    async fn insert_cache<T>(&self, url: &str, value: &T)
    where
        T: Serialize + std::fmt::Debug,
    {
        match serde_json::to_string(value) {
            Ok(data) => {
                let mut conn = self.redis.get().await;
                match conn.set_and_expire_seconds(url, data, self.duration).await {
                    Ok(_) => {}
                    Err(why) => debug!("Error while inserting value into cache: {}", why),
                }
            }
            Err(why) => debug!(
                "Error while serializing to cache: {}, value: {:?}",
                why, value,
            ),
        }
    }
}
