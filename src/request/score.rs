use super::{Pending, UserIdentification};
use crate::{
    model::{GameMode, GameMods, Score},
    routing::Route,
    Osu,
};

#[cfg(feature = "cache")]
use crate::client::cached::OsuCached;

/// Retrieve a [`Score`]
///
/// [`Score`]: ../model/struct.Score.html
pub struct GetScore<'a> {
    fut: Option<Pending<'a>>,
    osu: Option<&'a Osu>,

    limit: Option<u32>,
    map_id: u32,
    mode: Option<GameMode>,
    mods: Option<GameMods>,
    user: Option<UserIdentification>,
}

/// Retrieve [`Score`]s
///
/// [`Score`]: ../model/struct.Score.html
pub struct GetScores<'a> {
    fut: Option<Pending<'a>>,
    osu: Option<&'a Osu>,

    limit: Option<u32>,
    map_id: u32,
    mode: Option<GameMode>,
    mods: Option<GameMods>,
    user: Option<UserIdentification>,
}

macro_rules! impl_score {
    ($name: ident) => {
        impl<'a> $name<'a> {
            pub(crate) fn new(osu: &'a Osu, map_id: u32) -> Self {
                Self {
                    osu: Some(osu),
                    map_id,
                    fut: None,
                    limit: None,
                    mode: None,
                    mods: None,
                    user: None,
                }
            }

            /// Optional, specify a user either by id (`u32`) or name (`String`/`&str`).
            pub fn user(mut self, user: impl Into<UserIdentification>) -> Self {
                self.user.replace(user.into());

                self
            }

            /// Optional, amount of results from the top.
            /// Range between 1 and 100, defaults to 50.
            pub fn limit(mut self, limit: u32) -> Self {
                self.limit.replace(limit.max(1).min(100));

                self
            }

            /// Optional, defaults to `GameMode::STD`.
            pub fn mode(mut self, mode: GameMode) -> Self {
                self.mode.replace(mode);

                self
            }

            /// Optional, specify a mod combination.
            pub fn mods(mut self, mods: GameMods) -> Self {
                self.mods.replace(mods);

                self
            }

            fn start(&mut self) {
                let route = Route::GetScore {
                    limit: self.limit.take(),
                    map_id: self.map_id,
                    mode: self.mode.take(),
                    mods: self.mods.take(),
                    user: self.user.take(),
                };

                #[cfg(feature = "metrics")]
                self.osu.unwrap().0.metrics.scores.inc();

                #[cfg(feature = "cache")]
                self.fut.replace(Box::pin(
                    self.osu.unwrap().request_bytes(route, OsuCached::Score),
                ));

                #[cfg(not(feature = "cache"))]
                self.fut
                    .replace(Box::pin(self.osu.unwrap().request_bytes(route)));
            }
        }
    };
}

impl_score!(GetScores);
poll_vec_req!(GetScores<'_>, Score);

impl_score!(GetScore);
poll_req!(GetScore<'_>, Score);
