use crate::{
    backend::{OsuError, OsuResult},
    util::RateLimiter,
};

use futures::TryFutureExt;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use std::sync::Mutex;

/// The main osu client.
/// Pass this into a `queue` method of some request to retrieve and parse the data.
pub struct Osu {
    client: Client,
    api_key: String,
    ratelimiter: Mutex<RateLimiter>,
}

impl Osu {
    pub fn new(api_key: impl AsRef<str>) -> Self {
        Osu {
            client: Client::new(),
            api_key: api_key.as_ref().to_owned(),
            ratelimiter: Mutex::new(RateLimiter::new(10, 1)),
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
        {
            self.ratelimiter.lock().unwrap().await_access();
        }
        self.client
            .get(url)
            .send()
            .and_then(|res| res.bytes())
            .map_ok(|bytes| Ok(serde_json::from_slice(&bytes)?))
            .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
            .await?
    }
}
