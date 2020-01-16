mod maps;
mod scores;
mod user_best;
mod user_recent;
mod users;

pub use maps::BeatmapRequest;
pub use scores::ScoreRequest;
pub use user_best::UserBestRequest;
pub use user_recent::UserRecentRequest;
pub use users::UserRequest;

use crate::backend::{Osu, OsuError};

use futures::TryFutureExt;
use hyper::Uri;
use serde::de::DeserializeOwned;
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

const API_BASE: &str = "https://osu.ppy.sh/api/";

pub(crate) const USER_TAG: &str = "u";
pub(crate) const MODE_TAG: &str = "m";
pub(crate) const SET_TAG: &str = "s";
pub(crate) const MAP_TAG: &str = "b";
pub(crate) const SINCE_TAG: &str = "since";
pub(crate) const CONV_TAG: &str = "a";
pub(crate) const HASH_TAG: &str = "h";
pub(crate) const LIMIT_TAG: &str = "limit";
pub(crate) const MODS_TAG: &str = "mods";
pub(crate) const EVENT_DAYS_TAG: &str = "event_days";

/// Helper trait to allow arbitrary requests as parameter for `Osu`'s `prepare_request` method.
pub trait Request {
    /// Artifact from the public `Request` trait. This method has no use outside of this library.
    fn add_args(self, args: &mut HashMap<String, String>) -> (RequestType, bool);
}

/// A completely built request, ready to retrieve data.
pub struct OsuRequest<'o, T: Debug + DeserializeOwned> {
    osu: &'o mut Osu,
    pub(crate) args: HashMap<String, String>,
    with_cache: bool,
    req_type: RequestType,
    pd: PhantomData<T>,
}

impl<'o, T: Debug + DeserializeOwned> OsuRequest<'o, T> {
    /// Asynchronously send the request and await the parsed data.
    pub async fn queue(&mut self) -> Result<Vec<T>, OsuError> {
        let url = self.get_url()?;
        // Try using cache when desired
        if self.with_cache {
            debug!("Using cache for {}", url);
            if let Some(res) = self.osu.lookup_cache(&url) {
                debug!("Found cached");
                Ok(res)
            } else {
                debug!("Nothing in cache. Fetching...");
                // Fetch response text
                let res: String = self
                    .osu
                    .fetch_response_future(url.clone())
                    .and_then(|res| hyper::body::to_bytes(res.into_body()))
                    .map_ok(|bytes| String::from_utf8(bytes.to_vec()).unwrap())
                    .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
                    .await?;
                let deserialized: Vec<T> = serde_json::from_str(&res)?;
                // Cache response text
                self.osu.insert_cache(url, res);
                Ok(deserialized)
            }
        } else {
            // Fetch response and deserialize in one go
            debug!("Fetching url {}", url);
            self.osu
                .fetch_response_future(url)
                .and_then(|res| hyper::body::to_bytes(res.into_body()))
                .map_ok(|bytes| Ok(serde_json::from_slice(&bytes).unwrap()))
                .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
                .await?
        }
    }

    pub(crate) fn new<R>(osu: &'o mut Osu, req: R) -> Self
    where
        R: Request,
    {
        let mut args = HashMap::new();
        let (req_type, with_cache) = req.add_args(&mut args);
        Self {
            osu,
            args,
            with_cache,
            req_type,
            pd: PhantomData,
        }
    }

    fn get_url(&self) -> Result<Uri, OsuError> {
        if self.args.is_empty() {
            return Err(OsuError::ReqBuilder(
                "No arguments specified for query".to_owned(),
            ));
        }
        let mut url = format!("{}{}?", API_BASE, self.req_type.get_endpoint());
        let query: String = self
            .args
            .iter()
            .map(|(tag, val)| format!("{}={}", tag, val))
            .collect::<Vec<String>>()
            .join("&");
        url.push_str(&query);
        Ok(self.osu.prepare_url(url)?)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RequestType {
    User,
    Beatmap,
    Score,
    UserBest,
    UserRecent,
}

impl RequestType {
    fn get_endpoint(self) -> String {
        match self {
            RequestType::User => "get_user".to_owned(),
            RequestType::Beatmap => "get_beatmaps".to_owned(),
            RequestType::Score => "get_scores".to_owned(),
            RequestType::UserBest => "get_user_best".to_owned(),
            RequestType::UserRecent => "get_user_recent".to_owned(),
        }
    }
}
