use crate::{
    backend::requests::{
        Request, BEATMAP_ENDPOINT, CONV_TAG, HASH_TAG, LIMIT_TAG, MAP_TAG, MODE_TAG, MODS_TAG,
        SET_TAG, SINCE_TAG, TYPE_TAG, USER_TAG,
    },
    models::{Beatmap, GameMode, GameMods},
    Osu, OsuResult,
};

use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Clone, Default, Eq, PartialEq)]
/// Request struct to retrieve beatmaps.
pub struct BeatmapRequest {
    since: Option<DateTime<Utc>>,
    map_id: Option<u32>,
    mapset_id: Option<u32>,
    user_id: Option<u32>,
    username: Option<String>,
    mode: Option<GameMode>,
    limit: Option<u32>,
    mods: Option<u32>,
    with_converted: Option<bool>,
    hash: Option<String>,
}

impl BeatmapRequest {
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify a date to only consider maps from this date onwards.
    pub fn since(self, date: DateTime<Utc>) -> Self {
        Self {
            since: Some(date),
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify a beatmap id to only retrieve that map.
    pub fn map_id(self, id: u32) -> Self {
        Self {
            since: self.since,
            map_id: Some(id),
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify a beatmapset id to retrieve all maps of that set.
    pub fn mapset_id(self, id: u32) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: Some(id),
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify a user id to only get beatmaps created by that user.
    pub fn user_id(self, id: u32) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: Some(id),
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify a username to only get beatmaps created by that user.
    pub fn username(self, name: &str) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: Some(name.to_owned()),
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify a game mode for the request
    pub fn mode(self, mode: GameMode) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: Some(mode),
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify a limit for the amount of retrieved beatmaps. Default and limit are 500.
    pub fn limit(self, limit: u32) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: Some(limit),
            mods: self.mods,
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify mods for the retrieved beatmaps
    pub fn mods(self, mods: &GameMods) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: Some(mods.get_bits()),
            with_converted: self.with_converted,
            hash: self.hash,
        }
    }

    /// Specify whether converted maps should be included, default is false.
    pub fn with_converted(self, with_converted: bool) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: Some(with_converted),
            hash: self.hash,
        }
    }

    /// Specify the hash value of a beatmap that will be retrieved
    pub fn hash(self, hash: &str) -> Self {
        Self {
            since: self.since,
            map_id: self.map_id,
            mapset_id: self.mapset_id,
            user_id: self.user_id,
            username: self.username,
            mode: self.mode,
            limit: self.limit,
            mods: self.mods,
            with_converted: self.with_converted,
            hash: Some(hash.to_owned()),
        }
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
        let url = self.get_url(BEATMAP_ENDPOINT);
        osu.send_request(url).await
    }

    /// Asynchronously send the beatmap request and await the parsed `Beatmap`.
    /// If the API's response contains more than one beatmap, the method will
    /// return the last one. If the API response contains no beatmaps, the
    /// method will return `None`.
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

impl Request for BeatmapRequest {
    fn prepare_args<'s>(&self) -> HashMap<&'s str, String> {
        let mut args = HashMap::new();
        if let Some(since) = self.since {
            args.insert(SINCE_TAG, since.format("%F%%T").to_string());
        }
        if let Some(id) = self.map_id {
            args.insert(MAP_TAG, id.to_string());
        }
        if let Some(id) = self.mapset_id {
            args.insert(SET_TAG, id.to_string());
        }
        if let Some(id) = self.user_id {
            args.insert(USER_TAG, id.to_string());
            args.insert(TYPE_TAG, "id".to_string());
        } else if let Some(name) = &self.username {
            args.insert(USER_TAG, name.replace(" ", "+"));
            args.insert(TYPE_TAG, "string".to_string());
        }
        if let Some(mode) = self.mode {
            args.insert(MODE_TAG, (mode as u8).to_string());
        }
        if let Some(limit) = self.limit {
            args.insert(LIMIT_TAG, limit.to_string());
        }
        if let Some(mods) = self.mods {
            args.insert(MODS_TAG, mods.to_string());
        }
        if let Some(with_converted) = self.with_converted {
            args.insert(CONV_TAG, (with_converted as u8).to_string());
        }
        if let Some(hash) = &self.hash {
            args.insert(HASH_TAG, hash.to_owned());
        }
        args
    }
}
