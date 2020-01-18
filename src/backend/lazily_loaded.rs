use crate::{
    backend::{
        requests::{RequestType, API_BASE, MAP_TAG, USER_TAG},
        OsuApi, OsuError,
    },
    models::HasLazies,
};

use serde::de::DeserializeOwned;
use std::{
    fmt,
    marker::PhantomData,
    sync::{Arc, RwLock},
};

/// Fully prepared request, ready to query via `get` method
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LazilyLoaded<T: DeserializeOwned> {
    content: Option<LazyContent<T>>,
}

impl<T> LazilyLoaded<T>
where
    T: fmt::Debug + DeserializeOwned + HasLazies,
{
    pub(crate) fn new(osu: Arc<RwLock<OsuApi>>, result_id: u32, req_type: RequestType) -> Self {
        Self {
            content: Some(LazyContent::new(osu, result_id, req_type)),
        }
    }

    /// Retrieve data of this `LazilyLoaded`.
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::LazilyLoaded,
    ///     models::{Beatmap, Score, User},
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let score =    // created through previous Score/UserBest/UserRecent request
    /// # Score::default();
    /// let lazy_user: LazilyLoaded<User> = score.user;
    /// let user = lazy_user.get().await?;
    /// // ...
    /// if let Some(lazy_map) = score.beatmap {
    ///     let beatmap = lazy_map.get().await?;
    ///     // ...
    /// }
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn get(&self) -> Result<T, OsuError> {
        match &self.content {
            Some(content) => content.get().await,
            None => Err(OsuError::Other(
                "Can not get content of uninitialized LazilyLoaded".to_owned(),
            )),
        }
    }
}

#[derive(Clone)]
struct LazyContent<T> {
    osu: Arc<RwLock<OsuApi>>,
    result_id: u32,
    req_type: RequestType,
    pd: PhantomData<T>,
}

impl<T> fmt::Debug for LazyContent<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LazyContent {{ result_id: {}, type: {:?} }}",
            self.result_id, self.req_type
        )
    }
}

impl<T> PartialEq for LazyContent<T> {
    fn eq(&self, other: &Self) -> bool {
        self.result_id == other.result_id && self.req_type == other.req_type
    }
}

impl<T> Eq for LazyContent<T> {}

impl<T> LazyContent<T>
where
    T: fmt::Debug + DeserializeOwned + HasLazies,
{
    fn new(osu: Arc<RwLock<OsuApi>>, result_id: u32, req_type: RequestType) -> Self {
        Self {
            osu,
            result_id,
            req_type,
            pd: PhantomData,
        }
    }

    fn get_url(&self) -> String {
        use RequestType::*;
        let mut url = format!("{}{}?", API_BASE, self.req_type.get_endpoint());
        match self.req_type {
            User | UserBest | UserRecent => url.push(USER_TAG),
            Beatmap => url.push(MAP_TAG),
            Score => panic!("LazilyLoaded<Score> is not a thing"),
        }
        url.push('=');
        url.push_str(&self.result_id.to_string());
        url
    }

    async fn get(&self) -> Result<T, OsuError> {
        let url = self.get_url();
        let with_cache = self.with_cache();
        let mut res: Vec<T> = if with_cache {
            self.osu
                .write()
                .unwrap()
                .query_request_with_cache(url, self.osu.clone())
                .await?
        } else {
            self.osu
                .read()
                .unwrap()
                .query_request(url, self.osu.clone())
                .await?
        };
        if res.len() != 1 {
            let origin = if with_cache { "cache" } else { "api" };
            Err(OsuError::Other(format!(
                "Expected 1 object in response, {origin} returned {amount}",
                origin = origin,
                amount = res.len()
            )))
        } else {
            Ok(res.pop().unwrap())
        }
    }

    fn with_cache(&self) -> bool {
        self.req_type == RequestType::Beatmap
    }
}
