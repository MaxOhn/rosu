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

#[derive(Clone, Default, Eq, PartialEq, Debug)]
/// Request struct to retrieve beatmaps.
pub struct BeatmapRequest<'s> {
    args: HashMap<&'s str, String>,
}

impl<'s> BeatmapRequest<'s> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify a date to only consider maps from this date onwards.
    pub fn since(mut self, date: DateTime<Utc>) -> Self {
        self.args
            .insert(SINCE_TAG, date.format("%F%%T").to_string());
        self
    }

    /// Specify a beatmap id to only retrieve that map.
    pub fn map_id(mut self, id: u32) -> Self {
        self.args.insert(MAP_TAG, id.to_string());
        self
    }

    /// Specify a beatmapset id to retrieve all maps of that set.
    pub fn mapset_id(mut self, id: u32) -> Self {
        self.args.insert(SET_TAG, id.to_string());
        self
    }

    /// Specify a user id to only get beatmaps created by that user.
    pub fn user_id(mut self, id: u32) -> Self {
        self.args.insert(USER_TAG, id.to_string());
        self.args.insert(TYPE_TAG, "id".to_string());
        self
    }

    /// Specify a username to only get beatmaps created by that user.
    pub fn username(mut self, name: &str) -> Self {
        self.args.insert(USER_TAG, name.replace(" ", "+"));
        self.args.insert(TYPE_TAG, "string".to_string());
        self
    }

    /// Specify a game mode for the request
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.args.insert(MODE_TAG, (mode as u8).to_string());
        self
    }

    /// Specify a limit for the amount of retrieved beatmaps. Default and limit are 500.
    pub fn limit(mut self, limit: u32) -> Self {
        self.args.insert(LIMIT_TAG, limit.to_string());
        self
    }

    /// Specify mods for the retrieved beatmaps.
    /// Note that __all__ given mods should be difficulty-changing
    pub fn mods(mut self, mods: GameMods) -> Self {
        self.args.insert(MODS_TAG, mods.bits().to_string());
        self
    }

    /// Specify whether converted maps should be included, default is false.
    pub fn with_converted(mut self, with_converted: bool) -> Self {
        self.args
            .insert(CONV_TAG, (with_converted as u8).to_string());
        self
    }

    /// Specify the hash value of a beatmap that will be retrieved
    pub fn hash(mut self, hash: String) -> Self {
        self.args.insert(HASH_TAG, hash);
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
    /// let osu = Osu::new("osu_api_key".to_owned());
    /// let request: BeatmapRequest = BeatmapRequest::new()
    ///     .mapset_id(1086483)
    ///     .limit(2);
    /// let maps: Vec<Beatmap> = request.queue(&osu).await?;
    /// // ...
    /// # Ok::<_, OsuError>(())
    /// # });
    /// ```
    pub async fn queue(self, osu: &Osu) -> OsuResult<Vec<Beatmap>> {
        let url = Request::create_url(BEATMAP_ENDPOINT, self.args);
        osu.send_request(url).await
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
    /// let osu = Osu::new("osu_api_key".to_owned());
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
