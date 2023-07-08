use time::OffsetDateTime;

use super::{Pending, UserIdentification};
use crate::{
    model::{Beatmap, GameMode, GameMods},
    routing::Route,
    Osu,
};

/// Retrieve a [`Beatmap`](crate::model::Beatmap).
pub struct GetBeatmap<'a> {
    fut: Option<Pending<'a>>,
    osu: Option<&'a Osu>,

    creator: Option<UserIdentification>,
    hash: Option<String>,
    limit: Option<u32>,
    map_id: Option<u32>,
    mapset_id: Option<u32>,
    mode: Option<GameMode>,
    mods: Option<GameMods>,
    since: Option<OffsetDateTime>,
    with_converted: Option<bool>,
}

/// Retrieve [`Beatmap`](crate::model::Beatmap)s
pub struct GetBeatmaps<'a> {
    fut: Option<Pending<'a>>,
    osu: Option<&'a Osu>,

    creator: Option<UserIdentification>,
    hash: Option<String>,
    limit: Option<u32>,
    map_id: Option<u32>,
    mapset_id: Option<u32>,
    mode: Option<GameMode>,
    mods: Option<GameMods>,
    since: Option<OffsetDateTime>,
    with_converted: Option<bool>,
}

macro_rules! impl_beatmap {
    ($name: ident, $default_limit: expr) => {
        impl<'a> $name<'a> {
            pub(crate) fn new(osu: &'a Osu) -> Self {
                Self {
                    osu: Some(osu),
                    fut: None,
                    creator: None,
                    hash: None,
                    limit: $default_limit,
                    map_id: None,
                    mapset_id: None,
                    mode: None,
                    mods: None,
                    since: None,
                    with_converted: None,
                }
            }

            /// Optional, specify the creator of the mapset either by id (`u32`) or name (`String`/`&str`).
            #[inline]
            pub fn creator(mut self, creator: impl Into<UserIdentification>) -> Self {
                self.creator.replace(creator.into());

                self
            }

            /// Optional, the beatmap hash e.g. from a replay file.
            #[inline]
            pub fn hash(mut self, hash: impl Into<String>) -> Self {
                self.hash.replace(hash.into());

                self
            }

            /// Optional, amount of results.
            /// Default and maximum are 500.
            #[inline]
            pub fn limit(mut self, limit: u32) -> Self {
                self.limit.replace(limit.max(0).min(500));

                self
            }

            /// Optional, specify a beatmap_id
            #[inline]
            pub fn map_id(mut self, map_id: u32) -> Self {
                self.map_id.replace(map_id);

                self
            }

            /// Optional, specify a beatmapset_id
            #[inline]
            pub fn mapset_id(mut self, mapset_id: u32) -> Self {
                self.mapset_id.replace(mapset_id);

                self
            }

            /// Optional, defaults to `GameMode::Osu`
            #[inline]
            pub fn mode(mut self, mode: GameMode) -> Self {
                self.mode.replace(mode);

                self
            }

            /// Optional, mods that applies to the beatmap requested.
            /// Multiple mods is supported, but it should not contain any non-difficulty-increasing mods.
            #[inline]
            pub fn mods(mut self, mods: GameMods) -> Self {
                self.mods.replace(mods);

                self
            }

            /// Optional, only ranked/loved beatmaps approved since this date.
            #[inline]
            pub fn since(mut self, since: OffsetDateTime) -> Self {
                self.since.replace(since);

                self
            }

            /// Optional, specify whether converted beatmaps are included.
            /// Only has an effect if mode is chosen and not `GameMode::Osu`.
            /// Converted maps show their converted difficulty rating.
            /// Defaults to 0.
            #[inline]
            pub fn with_converted(mut self, with_converted: bool) -> Self {
                self.with_converted.replace(with_converted);

                self
            }

            fn start(&mut self) {
                let route = Route::GetBeatmaps {
                    creator: self.creator.take(),
                    hash: self.hash.take(),
                    limit: self.limit.take(),
                    map_id: self.map_id.take(),
                    mapset_id: self.mapset_id.take(),
                    mode: self.mode.take(),
                    mods: self.mods.take(),
                    since: self.since.take(),
                    with_converted: self.with_converted.take(),
                };

                #[cfg(feature = "metrics")]
                self.osu.unwrap().0.metrics.beatmaps.inc();

                self.fut
                    .replace(Box::pin(self.osu.unwrap().request_bytes(route)));
            }
        }
    };
}

impl_beatmap!(GetBeatmaps, None);
poll_vec_req!(GetBeatmaps<'_>, Beatmap);

impl_beatmap!(GetBeatmap, Some(1));
poll_req!(GetBeatmap<'_>, Beatmap);
