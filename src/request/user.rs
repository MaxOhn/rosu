use super::{Pending, UserIdentification};
use crate::{
    model::{GameMode, User},
    routing::Route,
    Osu,
};

#[cfg(feature = "cache")]
use crate::client::cached::OsuCached;

/// Retrieve a [`User`]
///
/// [`User`]: ../model/struct.User.html
pub struct GetUser<'a> {
    fut: Option<Pending<'a>>,
    osu: &'a Osu,

    user: Option<UserIdentification>,
    mode: Option<GameMode>,
    event_days: Option<u32>,
}

impl<'a> GetUser<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user: impl Into<UserIdentification>) -> Self {
        Self {
            fut: None,
            osu,
            event_days: None,
            mode: None,
            user: Some(user.into()),
        }
    }

    /// Optional, defaults to `GameMode::Osu`
    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    /// Optional, must be between 1 and 31, defaults to 1
    #[inline]
    pub fn event_days(mut self, event_days: u32) -> Self {
        self.event_days.replace(event_days.min(31).max(1));

        self
    }

    fn start(&mut self) {
        let route = Route::GetUser {
            user: self.user.take().unwrap(),
            mode: self.mode.take(),
            event_days: self.event_days.take(),
        };

        #[cfg(feature = "metrics")]
        self.osu.0.metrics.users.inc();

        #[cfg(feature = "cache")]
        self.fut
            .replace(Box::pin(self.osu.request_bytes(route, OsuCached::User)));

        #[cfg(not(feature = "cache"))]
        self.fut.replace(Box::pin(self.osu.request_bytes(route)));
    }
}

poll_req!(GetUser<'_>, User);
