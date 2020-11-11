use super::{Pending, UserIdentification};
use crate::{
    model::{GameMode, Score},
    routing::Route,
    Osu,
};

#[cfg(feature = "cache")]
use crate::client::cached::OsuCached;

macro_rules! define_user_score {
    ($name: ident, $limit: literal, $metric: ident) => {
        /// Retrieve [`Score`]s of a [`User`].
        ///
        /// [`Score`]: ../model/struct.Score.html
        /// [`User`]: ../model/struct.User.html
        pub struct $name<'a> {
            fut: Option<Pending<'a>>,
            osu: &'a Osu,

            limit: Option<u32>,
            mode: Option<GameMode>,
            user: Option<UserIdentification>,
        }

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
            pub fn limit(mut self, limit: u32) -> Self {
                self.limit.replace(limit.max(0).min($limit));

                self
            }

            /// Optional, defaults to `GameMode::STD`.
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

define_user_score!(GetUserBest, 100, top_scores);
poll_vec_req!(GetUserBest<'_>, Score);

define_user_score!(GetUserRecent, 50, recent_scores);
poll_vec_req!(GetUserRecent<'_>, Score);
