use crate::{
    backend::{OsuError, OsuResult},
    util::RateLimiter,
};

use futures::TryFutureExt;
use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector},
    Body, Client as HttpClient, Uri,
};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use std::sync::Mutex;

type Client = HttpClient<HttpsConnector<HttpConnector<GaiResolver>>, Body>;

/// The main osu client.
/// Pass this into a `queue` method of some request to retrieve and parse the data.
pub struct Osu {
    client: Client,
    api_key: String,
    ratelimiter: Mutex<RateLimiter>,
}

impl Osu {
    pub fn new(api_key: impl AsRef<str>) -> Self {
        let https = HttpsConnector::new();
        Osu {
            client: HttpClient::builder().build::<_, Body>(https),
            api_key: api_key.as_ref().to_owned(),
            ratelimiter: Mutex::new(RateLimiter::new(10, 1)),
        }
    }

    pub(crate) fn prepare_url(&self, mut url: String) -> OsuResult<Uri> {
        url.push_str("&k=");
        url.push_str(&self.api_key);
        url.parse().map_err(OsuError::from)
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
            .and_then(|res| hyper::body::to_bytes(res.into_body()))
            .map_ok(|bytes| Ok(serde_json::from_slice(&bytes)?))
            .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
            .await?
    }
}
