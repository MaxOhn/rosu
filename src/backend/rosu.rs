use crate::{
    backend::{
        requests::{OsuArgs, OsuRequest},
        OsuApi,
    },
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
    ///     backend::{requests::{OsuRequest, OsuArgs, UserArgs}, Osu},
    ///     models::User
    /// };
    ///
    /// let osu = Osu::new("osu_api_key".to_owned());
    /// let user_args = UserArgs::with_username("Badewanne3");
    /// let wrapped_args = OsuArgs::Users(user_args);
    /// let osu_request: OsuRequest<User> = osu.create_request(wrapped_args);
    /// ```
    pub fn create_request<T>(&self, request: OsuArgs) -> OsuRequest<T>
    where
        T: DeserializeOwned + HasLazies,
    {
        OsuRequest::new(self.api.clone(), request)
    }
}
