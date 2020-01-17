use crate::backend::{requests::RequestType, OsuApi, OsuError};

use hyper::Uri;
use serde::de::DeserializeOwned;
use std::{
    marker::PhantomData,
    sync::{Arc, RwLock},
};

pub struct LazilyLoaded<T: DeserializeOwned> {
    osu: Arc<RwLock<OsuApi>>,
    url: Uri,
    req_type: RequestType,
    pd: PhantomData<T>,
}

impl<T> LazilyLoaded<T>
where
    T: std::fmt::Debug + DeserializeOwned,
{
    pub async fn get(&self) -> Result<T, OsuError> {
        let mut api = self.osu.write().unwrap();
        let with_cache = self.req_type == RequestType::Beatmap;
        let mut res: Vec<T> = api.query_request(self.url.clone(), with_cache).await?;
        match res.pop() {
            Some(elem) => Ok(elem),
            None => {
                if with_cache {
                    Err(OsuError::Other(
                        "Result in cache did not contain any elements".to_owned(),
                    ))
                } else {
                    Err(OsuError::Other(
                        "Error while fetching lazily loaded: {}".to_owned(),
                    ))
                }
            }
        }
    }
}
