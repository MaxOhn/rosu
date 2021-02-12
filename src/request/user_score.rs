use super::{Pending, UserIdentification};
use crate::{
    model::{GameMode, Score},
    routing::Route,
    Osu,
};

#[cfg(feature = "cache")]
use crate::client::cached::OsuCached;

/// Retrieve the top scores of a [`User`](crate::model::User).
pub struct GetUserBest<'a> {
    fut: Option<Pending<'a>>,
    osu: &'a Osu,

    limit: Option<u32>,
    mode: Option<GameMode>,
    user: Option<UserIdentification>,
}

/// Retrieve the most recent scores of a [`User`](crate::model::User).
pub struct GetUserRecent<'a> {
    fut: Option<Pending<'a>>,
    osu: &'a Osu,

    limit: Option<u32>,
    mode: Option<GameMode>,
    user: Option<UserIdentification>,
}

macro_rules! impl_user_score {
    ($name: ident, $limit: literal, $metric: ident) => {
        impl<'a> $name<'a> {
            pub(crate) fn new(osu: &'a Osu, user: impl Into<UserIdentification>) -> Self {
                Self {
                    fut: None,
                    osu,
                    limit: None,
                    mode: None,
                    user: Some(user.into()),
                }
            }

            /// Optional, amount of results, defaults to 10.
            ///
            /// # Upper limit
            ///
            ///   - `GetUserBest`: 100
            ///   - `GetUserRecent`: 50
            #[inline]
            pub fn limit(mut self, limit: u32) -> Self {
                self.limit.replace(limit.max(0).min($limit));

                self
            }

            /// Optional, defaults to `GameMode::STD`.
            #[inline]
            pub fn mode(mut self, mode: GameMode) -> Self {
                self.mode.replace(mode);

                self
            }

            fn start(&mut self) {
                let route = Route::$name {
                    limit: self.limit.take(),
                    mode: self.mode.take(),
                    user: self.user.take().unwrap(),
                };

                #[cfg(feature = "metrics")]
                self.osu.0.metrics.$metric.inc();

                #[cfg(feature = "cache")]
                self.fut
                    .replace(Box::pin(self.osu.request_bytes(route, OsuCached::Score)));

                #[cfg(not(feature = "cache"))]
                self.fut.replace(Box::pin(self.osu.request_bytes(route)));
            }
        }
    };
}

impl_user_score!(GetUserBest, 100, top_scores);
poll_vec_req!(GetUserBest<'_>, Score);

impl_user_score!(GetUserRecent, 50, recent_scores);
poll_vec_req!(GetUserRecent<'_>, Score);
