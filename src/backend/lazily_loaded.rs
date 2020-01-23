use crate::{
    backend::{
        requests::{API_BASE, MAP_TAG, USER_TAG, Request},
        OsuApi, OsuError,
    },
    models::HasLazies,
};

use serde::de::DeserializeOwned;
use std::{
    fmt,
    fmt::Write,
    marker::PhantomData,
    sync::{Arc, RwLock},
};

/// Fully prepared request, ready to query via `get` method
#[derive(Clone, Default, Eq, PartialEq)]
pub struct LazilyLoaded<T: DeserializeOwned> {
    content: Option<LazyContent<T>>,
}

impl<T> LazilyLoaded<T>
where
    T: DeserializeOwned + HasLazies,
{
    pub(crate) fn new(osu: Arc<RwLock<OsuApi>>, key_id: u32, request: Request) -> Self {
        Self {
            content: Some(LazyContent::new(osu, key_id, request)),
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

impl<T> fmt::Debug for LazilyLoaded<T> where T: DeserializeOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(content) = &self.content {
            let request = match content.request {
                Request::Users(_) => "UserRequest",
                Request::Beatmaps(_) => "BeatmapRequest",
                Request::Scores(_) => "ScoresRequest",
                Request::Best(_) => "BestRequest",
                Request::Recent(_) => "RecentRequest",
            };
            write!(
                f,
                "LazilyLoaded {{ key_id: {}, type: {} }}",
                content.key_id, request
            )
        } else {
            let mut buf = String::new();
            buf.write_str("LazilyLoaded {{ None }}")
        }
    }
}

#[derive(Clone)]
struct LazyContent<T> {
    osu: Arc<RwLock<OsuApi>>,
    key_id: u32,
    request: Request,
    pd: PhantomData<T>,
}

impl<T> PartialEq for LazyContent<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key_id == other.key_id && self.request.get_endpoint() == other.request.get_endpoint()
    }
}

impl<T> Eq for LazyContent<T> {}

impl<T> LazyContent<T>
where
    T: DeserializeOwned + HasLazies,
{
    fn new(osu: Arc<RwLock<OsuApi>>, key_id: u32, request: Request) -> Self {
        Self {
            osu,
            key_id,
            request,
            pd: PhantomData,
        }
    }

    fn get_url(&self) -> String {
        let mut url = format!("{}{}?", API_BASE, &self.request.get_endpoint());
        match self.request {
            Request::Users(_) | Request::Best(_) | Request::Recent(_) => url.push(USER_TAG),
            Request::Beatmaps(_) => url.push(MAP_TAG),
            Request::Scores(_) => panic!("LazilyLoaded<Score> is not a thing"),
        }
        url.push('=');
        url.push_str(&self.key_id.to_string());
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
        match self.request {
            Request::Beatmaps(_) => true,
            _ => false
        }
    }
}
