use crate::{backend::OsuError, models::HasLazies, util::RateLimiter};

use futures::TryFutureExt;
use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector},
    Body, Client as HttpClient, Uri,
};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex, RwLock},
};

type Client = HttpClient<HttpsConnector<HttpConnector<GaiResolver>>, Body>;
type Cache<K = String, V = String> = Arc<RwLock<HashMap<K, V>>>;

pub struct OsuApi {
    client: Client,
    api_key: String,
    ratelimiter: Mutex<RateLimiter>,
    cache: Cache,
}

impl OsuApi {
    pub(crate) fn new(api_key: impl AsRef<str>) -> Self {
        let https = HttpsConnector::new();
        OsuApi {
            client: HttpClient::builder().build::<_, Body>(https),
            api_key: api_key.as_ref().to_owned(),
            ratelimiter: Mutex::new(RateLimiter::new(10, 1)),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub(crate) fn clear_cache(&mut self) {
        self.cache = Arc::new(RwLock::new(HashMap::new()));
    }

    pub(crate) fn lookup_cache<T: DeserializeOwned>(&self, url: &str) -> Option<T> {
        self.cache
            .read()
            .unwrap()
            .get(url)
            .map(|res| serde_json::from_str(res).unwrap())
    }

    pub(crate) fn insert_cache(&mut self, url: String, response: String) {
        self.cache.write().unwrap().insert(url, response);
    }

    pub(crate) fn prepare_url(&self, mut url: String) -> Result<Uri, OsuError> {
        url.push_str("&k=");
        url.push_str(&self.api_key);
        url.parse().map_err(OsuError::from)
    }

    pub(crate) async fn query_request<T>(
        &self,
        url: String,
        osu: Arc<RwLock<OsuApi>>,
    ) -> Result<Vec<T>, OsuError>
    where
        T: Debug + DeserializeOwned + HasLazies,
    {
        // Fetch response and deserialize in one go
        debug!("Fetching url {}", url);
        let url = self.prepare_url(url)?;
        self.ratelimiter.lock().unwrap().await_access();
        self.client
            .get(url)
            .and_then(|res| hyper::body::to_bytes(res.into_body()))
            .map_ok(|bytes| {
                let mut deserialized: Vec<T> = serde_json::from_slice(&bytes)?;
                for elem in deserialized.iter_mut() {
                    elem.prepare_lazies(osu.clone());
                }
                Ok(deserialized)
            })
            .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
            .await?
    }

    pub(crate) async fn query_request_with_cache<T>(
        &mut self,
        url: String,
        osu: Arc<RwLock<OsuApi>>,
    ) -> Result<Vec<T>, OsuError>
    where
        T: Debug + DeserializeOwned + HasLazies,
    {
        if let Some(res) = self.lookup_cache(&url) {
            debug!("Found cached for {}", url);
            Ok(res)
        } else {
            debug!("Nothing in cache for {}. Fetching...", url);
            // Fetch response text
            let prepared_url = self.prepare_url(url.clone())?;
            self.ratelimiter.lock().unwrap().await_access();
            self.client
                .get(prepared_url)
                .and_then(|res| hyper::body::to_bytes(res.into_body()))
                .map_ok(|bytes| {
                    let json = String::from_utf8(bytes.to_vec())?;
                    let mut deserialized: Vec<T> = serde_json::from_str(&json)?;
                    for elem in deserialized.iter_mut() {
                        elem.prepare_lazies(osu.clone());
                    }
                    // Cache response text
                    self.insert_cache(url, json);
                    Ok(deserialized)
                })
                .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
                .await?
        }
    }
}
