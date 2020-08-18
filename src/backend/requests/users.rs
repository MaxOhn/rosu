use super::{API_BASE, MODE_TAG, TYPE_TAG, USER_TAG};
use crate::{
    models::{GameMode, User},
    Osu, OsuError, OsuResult,
};

use reqwest::Url;

const EVENT_DAYS_TAG: &str = "event_days";
const USER_ENDPOINT: &str = "api/get_user";

#[derive(Clone, Eq, PartialEq)]
/// Request struct to retrieve users.
/// An instance __must__ contain either a user id or a username
pub struct UserRequest {
    url: Url,
}

impl UserRequest {
    /// Construct a `UserRequest` via user id
    pub fn with_user_id(id: u32) -> Self {
        let mut url =
            Url::parse_with_params(API_BASE, &[(TYPE_TAG, "id"), (USER_TAG, &id.to_string())])
                .unwrap();
        url.set_path(USER_ENDPOINT);
        Self { url }
    }

    /// Construct a `UserRequest` via username
    pub fn with_username(name: impl AsRef<str>) -> OsuResult<Self> {
        let mut url = Url::parse_with_params(
            API_BASE,
            &[
                (TYPE_TAG, "string"),
                (USER_TAG, &name.as_ref().replace(" ", "+")),
            ],
        )
        .map_err(|_| OsuError::ParseUrl)?;
        url.set_path(USER_ENDPOINT);
        Ok(Self { url })
    }

    /// Specify a game mode for the request
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(MODE_TAG, &(mode as u8).to_string());
        self
    }

    /// Specify event days for the request.
    ///
    /// Max number of days between now and last event date. Range of 1-31. Optional, default value is 1
    pub fn event_days(mut self, amount: u32) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(EVENT_DAYS_TAG, &amount.to_string());
        self
    }

    /// Asynchronously send the user request and await the parsed `Vec<User>`.
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::UserRequest},
    ///     models::User,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key");
    /// let request: UserRequest = UserRequest::with_username("Badewanne3").unwrap();
    /// let users: Vec<User> = request.queue(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue(self, osu: &Osu) -> OsuResult<Vec<User>> {
        match (cfg!(feature = "metrics"), cfg!(feature = "cache")) {
            (true, true) => {
                #[cfg(all(feature = "metrics", feature = "cache"))]
                {
                    let req = crate::backend::api::RequestType::User;
                    let cached = osu.cached.contains(crate::backend::OsuCached::User);
                    osu.send_request_metrics_cached(self.url, req, cached).await
                }
                #[cfg(not(all(feature = "metrics", feature = "cache")))]
                unreachable!()
            }
            (true, false) => {
                #[cfg(all(feature = "metrics", not(feature = "cache")))]
                {
                    let req = crate::backend::api::RequestType::User;
                    osu.send_request_metrics(self.url, req).await
                }
                #[cfg(not(all(feature = "metrics", not(feature = "cache"))))]
                unreachable!()
            }
            (false, true) => {
                #[cfg(all(not(feature = "metrics"), feature = "cache"))]
                {
                    let cached = osu.cached.contains(crate::backend::OsuCached::User);
                    osu.send_request_cached(self.url, cached).await
                }
                #[cfg(not(all(not(feature = "metrics"), feature = "cache")))]
                unreachable!()
            }
            (false, false) => {
                #[cfg(not(any(feature = "metrics", feature = "cache")))]
                {
                    osu.send_request(self.url).await
                }
                #[cfg(any(feature = "metrics", feature = "cache"))]
                unreachable!()
            }
        }
    }

    /// Asynchronously send the user request and await the parsed [`User`].
    ///
    /// If the API's response contains more than one user, the method will
    /// return the last one.
    ///
    /// If the API response contains no users, the method will return `None`.
    ///
    /// [`User`]: ../models/struct.User.html
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::UserRequest},
    ///     models::User,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key");
    /// let request: UserRequest = UserRequest::with_username("Badewanne3").unwrap();
    /// let user: Option<User> = request.queue_single(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue_single(self, osu: &Osu) -> OsuResult<Option<User>> {
        Ok(self.queue(osu).await?.pop())
    }
}
