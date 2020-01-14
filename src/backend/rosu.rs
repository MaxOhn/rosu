use crate::{backend::requests::*, models::*, util::RateLimiter};

use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector, ResponseFuture},
    http::uri::InvalidUri,
    Body, Client as HttpClient, Uri,
};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    fmt::{self, Debug},
    string::FromUtf8Error,
    sync::{Arc, Mutex},
};

type Client = HttpClient<HttpsConnector<HttpConnector<GaiResolver>>, Body>;
type Cache<K = Uri, V = String> = Arc<Mutex<HashMap<K, V>>>;

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

    pub(crate) fn lookup_cache<T: DeserializeOwned>(&self, url: &Uri) -> Option<T> {
        self.cache
            .lock()
            .unwrap()
            .get(url)
            .map(|res| serde_json::from_str(res).unwrap())
    }

    pub(crate) fn prepare_url(&self, mut url: String) -> Result<Uri, OsuError> {
        url.push_str("k=");
        url.push_str(&self.api_key);
        url.parse().map_err(OsuError::from)
    }

    pub(crate) fn insert_cache(&mut self, key: Uri, val: String) {
        self.cache.lock().unwrap().insert(key, val);
    }

    pub(crate) fn fetch_response_future(&self, url: Uri) -> ResponseFuture {
        self.client.get(url)
    }

    pub fn get_users(&mut self, req: UserReq) -> Result<OsuRequest<User>, OsuError> {
        let mut osu_req = OsuRequest::new(self);
        osu_req.with_cache(false);
        osu_req.add_user(req)?;
        Ok(osu_req)
    }
}

#[derive(Debug)]
pub enum OsuError {
    ReqBuilder(String),
    Hyper(::hyper::Error),
    Json(::serde_json::Error),
    Uri(InvalidUri),
    FromUtf8(FromUtf8Error),
    BadResponse(String),
    Other(String),
}

impl From<::hyper::Error> for OsuError {
    fn from(err: ::hyper::Error) -> Self {
        OsuError::Hyper(err)
    }
}

impl From<::serde_json::Error> for OsuError {
    fn from(err: ::serde_json::Error) -> Self {
        OsuError::Json(err)
    }
}

impl From<InvalidUri> for OsuError {
    fn from(err: InvalidUri) -> Self {
        OsuError::Uri(err)
    }
}

impl From<FromUtf8Error> for OsuError {
    fn from(err: FromUtf8Error) -> Self {
        OsuError::FromUtf8(err)
    }
}

impl fmt::Display for OsuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReqBuilder(e) => write!(f, "{}", e),
            Self::Hyper(e) => write!(f, "{}", e),
            Self::Json(e) => write!(f, "{}", e),
            Self::Uri(e) => write!(f, "{}", e),
            Self::FromUtf8(e) => write!(f, "{}", e),
            Self::BadResponse(e) => write!(f, "{}", e),
            Self::Other(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for OsuError {}
