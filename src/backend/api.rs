use crate::{backend::OsuError, util::RateLimiter};

use futures::TryFutureExt;
use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector},
    Body, Client as HttpClient, Uri,
};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use std::sync::Mutex;

type Client = HttpClient<HttpsConnector<HttpConnector<GaiResolver>>, Body>;

pub struct OsuApi {
    client: Client,
    api_key: String,
    ratelimiter: Mutex<RateLimiter>,
}

impl OsuApi {
    pub(crate) fn new(api_key: impl AsRef<str>) -> Self {
        let https = HttpsConnector::new();
        OsuApi {
            client: HttpClient::builder().build::<_, Body>(https),
            api_key: api_key.as_ref().to_owned(),
            ratelimiter: Mutex::new(RateLimiter::new(10, 1)),
        }
    }

    pub(crate) fn prepare_url(&self, mut url: String) -> Result<Uri, OsuError> {
        url.push_str("&k=");
        url.push_str(&self.api_key);
        url.parse().map_err(OsuError::from)
    }

    pub(crate) async fn query_request<T>(&self, url: String) -> Result<T, OsuError>
    where
        T: DeserializeOwned,
    {
        // Fetch response and deserialize in one go
        debug!("Fetching url {}", url);
        let url = self.prepare_url(url)?;
        self.ratelimiter.lock().unwrap().await_access();
        self.client
            .get(url)
            .and_then(|res| hyper::body::to_bytes(res.into_body()))
            .map_ok(|bytes| Ok(serde_json::from_slice(&bytes)?))
            .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
            .await?
    }
}
