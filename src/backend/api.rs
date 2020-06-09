use crate::backend::{OsuError, OsuResult};

use futures::TryFutureExt;
use governor::{
    clock::DefaultClock,
    state::{direct::NotKeyed, InMemoryState},
    Quota, RateLimiter,
};
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use std::num::NonZeroU32;

/// The main osu client.
/// Pass this into a `queue` method of some request to retrieve and parse the data.
pub struct Osu {
    client: Client,
    api_key: String,
    ratelimiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
}

impl Osu {
    pub fn new(api_key: impl AsRef<str>) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(15u32).unwrap());
        let ratelimiter = RateLimiter::direct(quota);
        Osu {
            client: Client::new(),
            api_key: api_key.as_ref().to_owned(),
            ratelimiter,
        }
    }

    pub(crate) fn prepare_url(&self, mut url: String) -> OsuResult<Url> {
        url.push_str("&k=");
        url.push_str(&self.api_key);
        Url::parse(&url)
            .map_err(|_| OsuError::Other(format!("Could not parse \"{}\" into url", url)))
    }

    pub(crate) async fn send_request<T>(&self, url: String) -> OsuResult<T>
    where
        T: DeserializeOwned,
    {
        // Fetch response and deserialize in one go
        debug!("Fetching url {}", url);
        let url = self.prepare_url(url)?;
        self.ratelimiter.until_ready().await;
        self.client
            .get(url)
            .send()
            .and_then(|res| res.bytes())
            .map_ok(|bytes| Ok(serde_json::from_slice(&bytes)?))
            .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
            .await?
    }
}
