use crate::{
    backend::{
        requests::{OsuArgs, OsuRequest},
        OsuApi, OsuError,
    },
    models::HasLazies,
};

use serde::de::DeserializeOwned;
use std::{
    fmt,
    fmt::Write,
    sync::{Arc, RwLock},
};

/// Fully prepared request, ready to query via `get` method
#[derive(Clone, Default, Eq, PartialEq)]
pub struct LazilyLoaded<T: DeserializeOwned> {
    request: Option<OsuRequest<T>>,
}

impl<T> LazilyLoaded<T>
where
    T: DeserializeOwned + HasLazies,
{
    pub(crate) fn new(osu: Arc<RwLock<OsuApi>>, args: OsuArgs) -> Self {
        Self {
            request: Some(OsuRequest::new(osu, args)),
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
        match &self.request {
            Some(request) => {
                let with_cache = match request.args {
                    OsuArgs::Beatmaps(_) => true,
                    _ => false,
                };
                let mut res = request.queue().await?;
                if res.len() != 1 {
                    Err(OsuError::Other(format!(
                        "Expected 1 object in response, {origin} returned {amount}",
                        origin = if with_cache { "cache" } else { "api" },
                        amount = res.len()
                    )))
                } else {
                    Ok(res.pop().unwrap())
                }
            }
            None => Err(OsuError::Other(
                "Can not get content of uninitialized LazilyLoaded".to_owned(),
            )),
        }
    }
}

impl<T> fmt::Debug for LazilyLoaded<T>
where
    T: DeserializeOwned,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(request) = &self.request {
            let req_type = match request.args {
                OsuArgs::Users(_) => "UserRequest",
                OsuArgs::Beatmaps(_) => "BeatmapRequest",
                OsuArgs::Scores(_) => "ScoresRequest",
                OsuArgs::Best(_) => "BestRequest",
                OsuArgs::Recent(_) => "RecentRequest",
            };
            write!(f, "LazilyLoaded {{ {} }}", req_type)
        } else {
            let mut buf = String::new();
            buf.write_str("LazilyLoaded {{ None }}")
        }
    }
}
