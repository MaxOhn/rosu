use crate::{
    backend::{requests::*, OsuError},
    util::RateLimiter,
};

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
    sync::{Arc, RwLock},
};

type Client = HttpClient<HttpsConnector<HttpConnector<GaiResolver>>, Body>;
type Cache<K = Uri, V = String> = Arc<RwLock<HashMap<K, V>>>;

pub(crate) struct OsuApi {
    client: Client,
    api_key: String,
    ratelimiter: RateLimiter,
    cache: Cache,
}

impl OsuApi {
    fn new(api_key: impl AsRef<str>) -> Self {
        let https = HttpsConnector::new();
        OsuApi {
            client: HttpClient::builder().build::<_, Body>(https),
            api_key: api_key.as_ref().to_owned(),
            ratelimiter: RateLimiter::new(1000, 10),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub(crate) fn lookup_cache<T: DeserializeOwned>(&self, url: &Uri) -> Option<T> {
        self.cache
            .read()
            .unwrap()
            .get(url)
            .map(|res| serde_json::from_str(res).unwrap())
    }

    pub(crate) fn insert_cache(&mut self, key: Uri, val: String) {
        self.cache.write().unwrap().insert(key, val);
    }

    pub(crate) fn prepare_url(&self, mut url: String) -> Result<Uri, OsuError> {
        url.push_str("&k=");
        url.push_str(&self.api_key);
        println!("{}", url);
        url.parse().map_err(OsuError::from)
    }

    pub(crate) async fn query_request<T>(
        &mut self,
        url: Uri,
        with_cache: bool,
    ) -> Result<Vec<T>, OsuError>
    where
        T: Debug + DeserializeOwned,
    {
        // Try using cache when desired
        if with_cache {
            debug!("Using cache for {}", url);
            if let Some(res) = self.lookup_cache(&url) {
                debug!("Found cached");
                Ok(res)
            } else {
                debug!("Nothing in cache. Fetching...");
                // Fetch response text
                self.ratelimiter.wait_access();
                let res: String = self
                    .client
                    .get(url.clone())
                    //.fetch_response_future(url.clone())
                    .and_then(|res| hyper::body::to_bytes(res.into_body()))
                    .map_ok(|bytes| String::from_utf8(bytes.to_vec()).unwrap())
                    .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
                    .await?;
                let deserialized: Vec<T> = serde_json::from_str(&res)?;
                // Cache response text
                self.insert_cache(url, res);
                Ok(deserialized)
            }
        } else {
            // Fetch response and deserialize in one go
            debug!("Fetching url {}", url);
            self.ratelimiter.wait_access();
            self.client
                .get(url)
                .and_then(|res| hyper::body::to_bytes(res.into_body()))
                .map_ok(|bytes| Ok(serde_json::from_slice(&bytes)?))
                .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
                .await?
        }
    }
}

/// The main osu client that will request all the data and return corresponding rosu models structs
pub struct Osu {
    api: Arc<RwLock<OsuApi>>,
}

impl Osu {
    pub fn new(api_key: impl AsRef<str>) -> Self {
        let api = OsuApi::new(api_key);
        Osu {
            api: Arc::new(RwLock::new(api)),
        }
    }

    /// Method to prepare an `OsuRequest` which will then be processed via `OsuRequest::queue`.
    /// # Examples
    /// ```
    /// use rosu::{
    ///     backend::{requests::{OsuRequest, UserRequest}, Osu},
    ///     models::User
    /// };
    ///
    /// let mut osu = Osu::new("osu_api_key".to_owned());
    /// let user_request = UserRequest::with_username("Badewanne3");
    /// let osu_request: OsuRequest<User> = osu.prepare_request(user_request);
    /// ```
    pub fn prepare_request<R, T>(&self, req: R) -> OsuRequest<T>
    where
        R: Request,
        T: Debug + DeserializeOwned,
    {
        OsuRequest::new(self.api.clone(), req)
    }
}
