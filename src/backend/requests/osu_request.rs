use crate::{
    backend::{
        requests::{Request, RequestType, API_BASE},
        OsuApi, OsuError,
    },
    models::HasLazies,
};

use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
    sync::{Arc, RwLock},
};

/// A completely built request, ready to retrieve data.
pub struct OsuRequest<T: Debug + DeserializeOwned> {
    osu: Arc<RwLock<OsuApi>>,
    args: HashMap<String, String>,
    req_type: RequestType,
    pd: PhantomData<T>,
}

impl<T> OsuRequest<T>
where
    T: Debug + DeserializeOwned + HasLazies,
{
    /// Asynchronously send the request and await the parsed data.
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::{OsuRequest, UserRequest}},
    ///     models::User,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key".to_owned());
    /// let user_request = UserRequest::with_username("Badewanne3");
    /// let osu_request = osu.prepare_request(user_request);
    /// let mut users: Vec<User> = osu_request.queue().await?;
    /// if let Some(user) = users.pop() {
    ///     // ...
    /// }
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue(&self) -> Result<Vec<T>, OsuError> {
        let url = self.get_url();
        let mut api = self.osu.write().unwrap();
        let res: Result<Vec<T>, OsuError> = if self.with_cache() {
            api.query_request_with_cache(url, self.osu.clone()).await
        } else {
            api.query_request(url, self.osu.clone()).await
        };
        res
    }

    pub(crate) fn new<R>(osu: Arc<RwLock<OsuApi>>, req: R) -> Self
    where
        R: Request,
    {
        let mut args = HashMap::new();
        let req_type = req.add_args(&mut args);
        Self {
            osu,
            args,
            req_type,
            pd: PhantomData,
        }
    }

    pub(crate) fn get_url(&self) -> String {
        let mut url = format!("{}{}?", API_BASE, self.req_type.get_endpoint());
        let query: String = self
            .args
            .iter()
            .map(|(tag, val)| format!("{}={}", tag, val))
            .collect::<Vec<String>>()
            .join("&");
        url.push_str(&query);
        url
    }

    fn with_cache(&self) -> bool {
        self.req_type == RequestType::Beatmap
    }
}
