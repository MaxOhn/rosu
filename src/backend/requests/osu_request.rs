use crate::backend::{
    requests::{OsuArgs, API_BASE},
    OsuApi, OsuError,
};

use serde::de::DeserializeOwned;
use std::{
    marker::PhantomData,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
/// A completely built request, ready to retrieve data.
pub struct OsuRequest<T: DeserializeOwned> {
    osu: Arc<RwLock<OsuApi>>,
    pub(crate) args: OsuArgs,
    pd: PhantomData<T>,
}

impl<T> OsuRequest<T>
where
    T: DeserializeOwned,
{
    /// Asynchronously send the request and await the parsed data.
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::{OsuRequest, OsuArgs, UserArgs}},
    ///     models::User,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key".to_owned());
    /// let args = UserArgs::with_username("Badewanne3");
    /// let osu_request = osu.create_request(OsuArgs::Users(args));
    /// let mut users: Vec<User> = osu_request.queue().await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue(&self) -> Result<T, OsuError> {
        let url = self.get_url();
        let api = self.osu.read().unwrap();
        api.query_request(url).await
    }

    pub(crate) fn new(osu: Arc<RwLock<OsuApi>>, args: OsuArgs) -> Self {
        Self {
            osu,
            args,
            pd: PhantomData,
        }
    }

    pub(crate) fn get_url(&self) -> String {
        let args = self.args.get_args();
        let mut url = format!("{}{}?", API_BASE, self.args.get_endpoint());
        let query: String = args
            .iter()
            .map(|(tag, val)| format!("{}={}", tag, val))
            .collect::<Vec<String>>()
            .join("&");
        url.push_str(&query);
        url
    }
}

impl<T: DeserializeOwned> PartialEq for OsuRequest<T> {
    fn eq(&self, other: &Self) -> bool {
        self.args == other.args
    }
}

impl<T: DeserializeOwned> Eq for OsuRequest<T> {}
