use super::{API_BASE, LIMIT_TAG, MAP_TAG, MODE_TAG, MODS_TAG, TYPE_TAG, USER_TAG};
use crate::{
    models::{Beatmap, GameMode, GameMods},
    Osu, OsuResult,
};

use chrono::{DateTime, Utc};
use reqwest::Url;

const SET_TAG: &str = "s";
const HASH_TAG: &str = "h";
const SINCE_TAG: &str = "since";
const CONV_TAG: &str = "a";
const BEATMAP_ENDPOINT: &str = "api/get_beatmaps";

#[derive(Clone, Eq, PartialEq, Debug)]
/// Request struct to retrieve beatmaps.
pub struct BeatmapRequest {
    url: Url,
}

impl Default for BeatmapRequest {
    fn default() -> Self {
        let mut url = Url::parse(API_BASE).unwrap();
        url.set_path(BEATMAP_ENDPOINT);
        Self { url }
    }
}

impl BeatmapRequest {
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify a date to only consider maps from this date onwards.
    pub fn since(mut self, date: DateTime<Utc>) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(SINCE_TAG, &date.format("%F%%T").to_string());
        self
    }

    /// Specify a beatmap id to only retrieve that map.
    pub fn map_id(mut self, id: u32) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(MAP_TAG, &id.to_string());
        self
    }

    /// Specify a beatmapset id to retrieve all maps of that set.
    pub fn mapset_id(mut self, id: u32) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(SET_TAG, &id.to_string());
        self
    }

    /// Specify a user id to only get beatmaps created by that user.
    pub fn user_id(mut self, id: u32) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(TYPE_TAG, "id")
            .append_pair(USER_TAG, &id.to_string());
        self
    }

    /// Specify a username to only get beatmaps created by that user.
    pub fn username(mut self, name: impl AsRef<str>) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(TYPE_TAG, "string")
            .append_pair(USER_TAG, name.as_ref());
        self
    }

    /// Specify a game mode for the request
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(MODE_TAG, &(mode as u8).to_string());
        self
    }

    /// Specify a limit for the amount of retrieved beatmaps. Default and limit are 500.
    pub fn limit(mut self, limit: u32) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(LIMIT_TAG, &limit.min(500).to_string());
        self
    }

    /// Specify mods for the retrieved beatmaps.
    /// Note that __all__ given mods should be difficulty-changing
    pub fn mods(mut self, mods: GameMods) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(MODS_TAG, &mods.bits().to_string());
        self
    }

    /// Specify whether converted maps should be included, default is false.
    pub fn with_converted(mut self, with_converted: bool) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(CONV_TAG, &(with_converted as u8).to_string());
        self
    }

    /// Specify the hash value of a beatmap that will be retrieved
    pub fn hash(mut self, hash: impl AsRef<str>) -> Self {
        self.url
            .query_pairs_mut()
            .append_pair(HASH_TAG, hash.as_ref());
        self
    }

    /// Asynchronously send the beatmap request and await the parsed `Vec<Beatmap>`.
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::BeatmapRequest},
    ///     models::Beatmap,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key");
    /// let request: BeatmapRequest = BeatmapRequest::new()
    ///     .mapset_id(1086483)
    ///     .limit(2);
    /// let maps: Vec<Beatmap> = request.queue(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue(self, osu: &Osu) -> OsuResult<Vec<Beatmap>> {
        match (cfg!(feature = "metrics"), cfg!(feature = "cache")) {
            (true, true) => {
                #[cfg(all(feature = "metrics", feature = "cache"))]
                {
                    let req = crate::backend::api::RequestType::Beatmap;
                    let cached = osu.cached.contains(crate::backend::OsuCached::Beatmap);
                    osu.send_request_metrics_cached(self.url, req, cached).await
                }
                #[cfg(not(all(feature = "metrics", feature = "cache")))]
                unreachable!()
            }
            (true, false) => {
                #[cfg(all(feature = "metrics", not(feature = "cache")))]
                {
                    let req = crate::backend::api::RequestType::Beatmap;
                    osu.send_request_metrics(self.url, req).await
                }
                #[cfg(not(all(feature = "metrics", not(feature = "cache"))))]
                unreachable!()
            }
            (false, true) => {
                #[cfg(all(not(feature = "metrics"), feature = "cache"))]
                {
                    let cached = osu.cached.contains(crate::backend::OsuCached::Beatmap);
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

    /// Asynchronously send the beatmap request and await the parsed [`Beatmap`].
    ///
    /// If the API's response contains more than one beatmap, the method will
    /// return the last one.
    ///
    /// If the API response contains no beatmaps, the method will return `None`.
    ///
    /// [`Beatmap`]: ../models/struct.Beatmap.html
    /// # Example
    /// ```no_run
    /// # use tokio::runtime::Runtime;
    /// # use rosu::OsuError;
    /// use rosu::{
    ///     backend::{Osu, requests::BeatmapRequest},
    ///     models::Beatmap,
    /// };
    ///
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async move {
    /// let osu = Osu::new("osu_api_key");
    /// let request: BeatmapRequest = BeatmapRequest::new()
    ///     .mapset_id(1086483)
    ///     .limit(1);
    /// let map: Option<Beatmap> = request.queue_single(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue_single(self, osu: &Osu) -> OsuResult<Option<Beatmap>> {
        Ok(self.queue(osu).await?.pop())
    }
}
