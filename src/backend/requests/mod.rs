pub mod maps;
pub mod scores;
pub mod user;
pub mod user_best;
pub mod user_recent;

pub use maps::MapsReq;
pub use scores::ScoresReq;
pub use user::UserReq;
pub use user_best::UserBestReq;
pub use user_recent::UserRecentReq;

use crate::backend::{Osu, OsuError};

use bytes::buf::ext::BufExt;
use futures::TryFutureExt;
use hyper::{body::Buf, Uri};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

const API_BASE: &str = "https://osu.ppy.sh/api/";

const USER_TAG: &str = "u";
const MODE_TAG: &str = "m";

impl<'o, T: Debug + DeserializeOwned> OsuRequest<'o, T> {
    pub(crate) fn new(osu: &'o mut Osu) -> Self {
        Self {
            osu,
            args: HashMap::new(),
            with_cache: true,
            req_type: None,
            pd: PhantomData,
        }
    }

    pub(crate) fn with_cache(&mut self, with_cache: bool) {
        self.with_cache = with_cache;
    }

    pub(crate) fn add_user(&mut self, req: UserReq) -> Result<(), OsuError> {
        self.check_type(ReqType::User)?;
        if let Some(id) = req.get_user_id() {
            self.args.insert(USER_TAG.to_owned(), id.to_string());
        } else if let Some(name) = req.get_username() {
            self.args.insert(USER_TAG.to_owned(), name);
        }
        if let Some(mode) = req.get_mode() {
            self.args
                .insert(MODE_TAG.to_owned(), (mode as u8).to_string());
        }
        Ok(())
    }

    fn get_url(&self) -> Result<Uri, OsuError> {
        let rt = self
            .req_type
            .ok_or_else(|| OsuError::ReqBuilder("No request type specified".to_owned()))?;
        if self.args.is_empty() {
            return Err(OsuError::ReqBuilder(
                "No arguments specified for query".to_owned(),
            ));
        }
        let mut url = format!("{}{}?", API_BASE, rt.get_endpoint());
        for (tag, val) in self.args.iter() {
            url.push_str(&tag);
            url.push('=');
            url.push_str(&val);
            url.push('&');
        }
        Ok(self.osu.prepare_url(url)?)
    }

    pub async fn queue(&mut self) -> Result<Vec<T>, OsuError> {
        let url = self.get_url()?;
        // Try using cache when desired
        if self.with_cache {
            debug!("Using cache for {}", url);
            println!("Using cache for {}", url);
            if let Some(res) = self.osu.lookup_cache(&url) {
                debug!("Found cached: {:?}", res);
                Ok(res)
            } else {
                debug!("Nothing in cache. Fetching...");
                println!("Nothing in cache. Fetching...");
                // Fetch response text
                let res: String = self
                    .osu
                    .fetch_response_future(url.clone())
                    .and_then(|res| hyper::body::aggregate(res.into_body()))
                    .map_ok(|buf| String::from_utf8_lossy(buf.bytes()).into())
                    .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
                    .await?;
                println!("res: {}", res);
                let deserialized: Vec<T> = serde_json::from_str(&res)?;
                // Cache response text
                self.osu.insert_cache(url, res);
                Ok(deserialized)
            }
        } else {
            // Fetch response and deserialize in one go
            debug!("Fetching url {}", url);
            println!("Fetching url {}", url);
            self.osu
                .fetch_response_future(url)
                .and_then(|res| hyper::body::aggregate(res.into_body()))
                .map_ok(|buf| Ok(serde_json::from_reader(buf.reader()).unwrap()))
                .map_err(|e| OsuError::Other(format!("Error while fetching: {}", e)))
                .await?
        }
    }

    fn check_type(&mut self, req_type: ReqType) -> Result<(), OsuError> {
        if let Some(rt) = self.req_type {
            Err(OsuError::Other(format!(
                "Cannot add {} element to request because request already already has type {}",
                req_type.get_endpoint(),
                rt.get_endpoint()
            )))
        } else {
            self.req_type = Some(req_type);
            Ok(())
        }
    }
}

#[derive(Copy, Clone)]
enum ReqType {
    User,
    Maps,
    Scores,
    UserBest,
    UserRecent,
}

impl ReqType {
    fn get_endpoint(self) -> String {
        match self {
            ReqType::User => "get_user".to_owned(),
            ReqType::Maps => "get_beatmaps".to_owned(),
            ReqType::Scores => "get_scores".to_owned(),
            ReqType::UserBest => "get_user_best".to_owned(),
            ReqType::UserRecent => "get_user_recent".to_owned(),
        }
    }
}

pub struct OsuRequest<'o, T: Debug + DeserializeOwned> {
    osu: &'o mut Osu,
    args: HashMap<String, String>,
    with_cache: bool,
    req_type: Option<ReqType>,
    pd: PhantomData<T>,
}
