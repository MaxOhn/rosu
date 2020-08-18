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

/// The main osu client.
/// Pass this into a `queue` method of some request to retrieve and parse the data.
pub struct Osu {
    client: Client,
    api_key: String,
    ratelimiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
    #[cfg(feature = "metrics")]
    stats: IntCounterVec,
}

impl Osu {
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

    pub(crate) fn prepare_url(&self, mut url: String) -> Url {
        let _ = write!(url, "&k={}", &self.api_key);
        Url::parse(&url).unwrap()
    }

    #[cfg(not(feature = "metrics"))]
    pub(crate) async fn send_request<T>(&self, url: String) -> OsuResult<T>
    where
        T: DeserializeOwned,
    {
        // Fetch response and deserialize in one go
        debug!("Fetching url {}", url);
        let url = self.prepare_url(url);
        self.ratelimiter.until_ready().await;
        self.client
            .get(url)
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
    pub(crate) async fn send_request_metrics<T>(
        &self,
        url: String,
        req: RequestType,
    ) -> OsuResult<T>
    where
        T: DeserializeOwned,
    {
        // Fetch response and deserialize in one go
        debug!("Fetching url {}", url);
        let url = self.prepare_url(url);
        self.ratelimiter.until_ready().await;
        self.client
            .get(url)
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
