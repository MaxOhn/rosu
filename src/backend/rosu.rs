use crate::{
    backend::{requests::*, OsuApi},
    models::HasLazies,
};

use serde::de::DeserializeOwned;
use std::sync::{Arc, RwLock};

/// The main osu client that will request all the data and return corresponding rosu models structs
pub struct Osu {
    api: Arc<RwLock<OsuApi>>,
}

impl Osu {
    pub fn new(api_key: impl AsRef<str>) -> Self {
        let api = OsuApi::new(api_key);
        Osu {
            api: Arc::new(RwLock::new(api)),
        }
    }

    /// Method to prepare an `OsuRequest` which will then be processed via `OsuRequest::queue`.
    /// # Example
    /// ```
    /// use rosu::{
    ///     backend::{requests::{OsuRequest, UserRequest}, Osu},
    ///     models::User
    /// };
    ///
    /// let osu = Osu::new("osu_api_key".to_owned());
    /// let user_request = UserRequest::with_username("Badewanne3");
    /// let osu_request: OsuRequest<User> = osu.prepare_request(user_request);
    /// ```
    pub fn prepare_request<R, T>(&self, req: R) -> OsuRequest<T>
    where
        R: Request,
        T: std::fmt::Debug + DeserializeOwned + HasLazies,
    {
        OsuRequest::new(self.api.clone(), req)
    }

    /// An internal cache keeps track of retrieved data, currently only used for beatmaps.
    /// This function resets the cache.
    pub fn clear_cache(&self) {
        self.api.write().unwrap().clear_cache();
    }
}
