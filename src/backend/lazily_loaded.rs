use crate::{
    backend::{
        requests::{OsuArgs, OsuRequest},
        OsuApi, OsuError,
    },
    models::{GameMode, HasLazies},
};

use serde::de::DeserializeOwned;
use std::{
    fmt,
    marker::PhantomData,
    sync::{Arc, RwLock},
};

/// Fully prepared request, ready to query via `get` method
#[derive(Clone, Default)]
pub struct LazilyLoaded<T: DeserializeOwned> {
    osu: Option<Arc<RwLock<OsuApi>>>,
    args: Option<OsuArgs>,
    pd: PhantomData<T>,
}

impl<T> LazilyLoaded<T>
where
    T: DeserializeOwned + HasLazies,
{
    pub(crate) fn create(osu: Arc<RwLock<OsuApi>>, args: OsuArgs) -> Self {
        Self {
            osu: Some(osu),
            args: Some(args),
            pd: PhantomData,
        }
    }

    /// Retrieve data of this `LazilyLoaded` for the optionally given `GameMode`.
    /// If `mode` is not specified, the api will default it to `GameMode::STD`.
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::LazilyLoaded,
    ///     models::{Beatmap, GameMode, Score, User},
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let score =    // created through previous Score/UserBest/UserRecent request
    /// # Score::default();
    /// let lazy_user: LazilyLoaded<User> = score.user;
    /// let user = lazy_user.get(GameMode::MNA).await?;
    /// // ...
    /// if let Some(lazy_map) = score.beatmap {
    ///     let beatmap = lazy_map.get(GameMode::STD).await?;
    ///     // ...
    /// }
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn get(&self, mode: GameMode) -> Result<T, OsuError> {
        match &self.args {
            Some(args) => {
                let with_cache = match args {
                    OsuArgs::Beatmaps(_) => true,
                    _ => false,
                };
                let args = match args {
                    OsuArgs::Users(a) => OsuArgs::Users(a.clone().mode(mode)),
                    OsuArgs::Beatmaps(a) => OsuArgs::Beatmaps(a.clone().mode(mode)),
                    _ => {
                        return Err(OsuError::Other(String::from(
                            "LazilyLoaded<Score> is not a thing",
                        )));
                    }
                };
                let request = OsuRequest::new(self.osu.clone().unwrap(), args);
                let mut res = request.queue().await?;
                if res.len() != 1 {
                    Err(OsuError::Other(format!(
                        "Expected one object in response, {origin} returned {amount}",
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
        if let Some(args) = &self.args {
            let req_type = match args {
                OsuArgs::Users(_) => "UserRequest",
                OsuArgs::Beatmaps(_) => "BeatmapRequest",
                OsuArgs::Scores(_) => "ScoresRequest",
                OsuArgs::Best(_) => "BestRequest",
                OsuArgs::Recent(_) => "RecentRequest",
            };
            write!(f, "LazilyLoaded {{ {} }}", req_type)
        } else {
            write!(f, "LazilyLoaded {{ None }}")
        }
    }
}

impl<T: DeserializeOwned> PartialEq for LazilyLoaded<T> {
    fn eq(&self, other: &Self) -> bool {
        self.args == other.args
    }
}

impl<T: DeserializeOwned> Eq for LazilyLoaded<T> {}
