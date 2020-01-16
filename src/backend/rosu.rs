use crate::{
    backend::{requests::*, OsuError},
    util::RateLimiter,
};

use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector, ResponseFuture},
    Body, Client as HttpClient, Uri,
};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};

type Client = HttpClient<HttpsConnector<HttpConnector<GaiResolver>>, Body>;
type Cache<K = Uri, V = String> = Arc<Mutex<HashMap<K, V>>>;

/// The main osu client that will request all the data and return corresponding rosu models structs
pub struct Osu {
    client: Client,
    api_key: String,
    ratelimiter: RateLimiter,
    cache: Cache,
}

impl Osu {
    pub fn new(api_key: impl AsRef<str>) -> Self {
        let https = HttpsConnector::new();
        Osu {
            client: HttpClient::builder().build::<_, Body>(https),
            api_key: api_key.as_ref().to_owned(),
            ratelimiter: RateLimiter::new(1000, 10),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Method to prepare an `OsuRequest` which will then be processed via `OsuRequest::queue`.
    /// # Examples
    /// ```
    /// use rosu::{backend::{requests::{OsuRequest, UserRequest}, Osu}, models::User};
    ///
    /// let mut osu = Osu::new("osu api key".to_owned());
    /// let user_request = UserRequest::with_username("Badewanne3");
    /// let osu_request: OsuRequest<User> = osu.prepare_request(user_request);
    /// ```
    pub fn prepare_request<R, T>(&mut self, req: R) -> OsuRequest<'_, T>
    where
        R: Request,
        T: Debug + DeserializeOwned,
    {
        OsuRequest::new(self, req)
    }

    pub(crate) fn lookup_cache<T: DeserializeOwned>(&self, url: &Uri) -> Option<T> {
        self.cache
            .lock()
            .unwrap()
            .get(url)
            .map(|res| serde_json::from_str(res).unwrap())
    }

    pub(crate) fn prepare_url(&self, mut url: String) -> Result<Uri, OsuError> {
        url.push_str("&k=");
        url.push_str(&self.api_key);
        url.parse().map_err(OsuError::from)
    }

    pub(crate) fn insert_cache(&mut self, key: Uri, val: String) {
        self.cache.lock().unwrap().insert(key, val);
    }

    pub(crate) fn fetch_response_future(&mut self, url: Uri) -> ResponseFuture {
        self.ratelimiter.wait_access();
        self.client.get(url)
    }
}
