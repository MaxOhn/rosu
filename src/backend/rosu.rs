use crate::{backend::requests::*, models::*, util::RateLimiter};

use bytes::buf::ext::BufExt;
use futures::{
    future::{ok, Either},
    Future, FutureExt, TryFutureExt, TryStreamExt,
};
use hyper::{
    body::{Buf, Bytes},
    client::{connect::dns::GaiResolver, HttpConnector},
    http::uri::InvalidUri,
    Body, Client, Request, Response, Uri,
};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use std::{
    char,
    collections::HashMap,
    fmt::{self, Debug},
    string::FromUtf8Error,
    sync::{Arc, Mutex},
};

const API_BASE: &'static str = "https://osu.ppy.sh/api/";
const USER: &'static str = "get_user";

type Cache<K = Uri, V = String> = Arc<Mutex<HashMap<K, V>>>;

pub struct Osu {
    client: Client<HttpsConnector<HttpConnector<GaiResolver>>, Body>,
    api_key: String,
    ratelimiter: RateLimiter,
    cache: Cache,
}

impl Osu {
    pub fn new(api_key: impl AsRef<str>) -> Self {
        let https = HttpsConnector::new();
        Osu {
            client: Client::builder().build::<_, Body>(https),
            api_key: api_key.as_ref().to_owned(),
            ratelimiter: RateLimiter::new(1000, 10),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get_user(&self, req: UserReq) -> Result<User, OsuError> {
        if req.user_id.is_none() && req.username.is_none() {
            return Err(OsuError::ReqBuilder(
                "Neither user id nor username were specified for retrieving a user from the osu! API".to_owned()
            ));
        }
        let mut url = format!("{}{}?k={}&u=", API_BASE, USER, self.api_key);
        if let Some(username) = req.username {
            url.push_str(&username);
        } else if let Some(user_id) = req.user_id {
            url.push_str(&user_id.to_string());
        }
        if let Some(mode) = req.mode {
            url.push_str("&m=");
            url.push(char::from_digit(mode as u32, 10).ok_or_else(|| {
                OsuError::ReqBuilder(format!("Could not parse mode {} into char", mode as u32))
            })?);
        }
        println!("URL: {}", url);
        let json = self.fetch_reponse(url.parse()?).await?;
        println!("json: {}", json);
        let mut result: Vec<User> = serde_json::from_str(&json)?;
        match result.pop() {
            Some(user) => Ok(user),
            None => Err(OsuError::Other("No user found".to_owned())),
        }
    }

    /// Util function that fetches a response from url
    pub(crate) async fn fetch_reponse(&self, url: Uri) -> Result<String, OsuError> {
        self.client
            .get(url)
            .and_then(|res| hyper::body::aggregate(res.into_body()))
            .map_ok(|buf| Ok(String::from_utf8_lossy(buf.bytes()).into()))
            .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
            .await?
    }

    /// Util function that either returns deserialized response from cache or fetches response from url and then deserializes it
    pub(crate) async fn cached_resp<T: Debug + DeserializeOwned>(
        &self,
        url: Uri,
    ) -> Result<T, OsuError> {
        let maybe_res: Option<T> = self
            .cache
            .lock()
            .unwrap()
            .get(&url)
            .map(|res| serde_json::from_str(res).unwrap());
        if let Some(res) = maybe_res {
            debug!("Found cached: {:?}", res);
            Ok(res)
        } else {
            debug!("Nothing in cache. Fetching...");
            let json = self.fetch_reponse(url.clone()).await?;
            println!("json: {}", json);
            debug!("Deserializing...");
            let deserialized: T = serde_json::from_str(&json)?;
            self.cache.lock().unwrap().insert(url, json.into());
            Ok(deserialized)
        }
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
