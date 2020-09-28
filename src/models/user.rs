use crate::{
    backend::{
        requests::{BestRequest, RecentRequest},
        Osu, OsuResult,
    },
    models::{GameMode, Score},
    serde::*,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[cfg(feature = "serialize")]
use serde::Serialize;

/// User struct retrieved from the `/api/get_user` endpoint.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct User {
    #[serde(deserialize_with = "to_u32")]
    pub user_id: u32,
    pub username: String,
    #[serde(with = "serde_date")]
    pub join_date: DateTime<Utc>,
    #[serde(deserialize_with = "to_u32")]
    pub count300: u32,
    #[serde(deserialize_with = "to_u32")]
    pub count100: u32,
    #[serde(deserialize_with = "to_u32")]
    pub count50: u32,
    #[serde(deserialize_with = "to_u32")]
    pub playcount: u32,
    #[serde(deserialize_with = "to_u64")]
    pub ranked_score: u64,
    #[serde(deserialize_with = "to_u64")]
    pub total_score: u64,
    #[serde(deserialize_with = "to_u32")]
    pub pp_rank: u32,
    #[serde(deserialize_with = "to_f32")]
    pub level: f32,
    #[serde(deserialize_with = "to_f32")]
    pub pp_raw: f32,
    #[serde(deserialize_with = "to_f32")]
    pub accuracy: f32,
    #[serde(alias = "count_rank_ssh", deserialize_with = "to_u32")]
    pub count_ssh: u32,
    #[serde(alias = "count_rank_ss", deserialize_with = "to_u32")]
    pub count_ss: u32,
    #[serde(alias = "count_rank_sh", deserialize_with = "to_u32")]
    pub count_sh: u32,
    #[serde(alias = "count_rank_s", deserialize_with = "to_u32")]
    pub count_s: u32,
    #[serde(alias = "count_rank_a", deserialize_with = "to_u32")]
    pub count_a: u32,
    pub country: String,
    #[serde(deserialize_with = "to_u32")]
    pub total_seconds_played: u32,
    #[serde(deserialize_with = "to_u32")]
    pub pp_country_rank: u32,
    #[serde(default, skip_serializing_if = "default_vec")]
    pub events: Vec<Event>,
}

impl User {
    /// Retrieve the user's top scores from the API `(0 < amount <= 100)`
    pub async fn get_top_scores(
        &self,
        osu: &Osu,
        amount: u32,
        mode: GameMode,
    ) -> OsuResult<Vec<Score>> {
        BestRequest::with_user_id(self.user_id)
            .limit(amount)
            .mode(mode)
            .queue(osu)
            .await
    }

    /// Retrieve the user's recent scores from the API `(0 < amount <= 50)`
    pub async fn get_recent_scores(
        &self,
        osu: &Osu,
        amount: u32,
        mode: GameMode,
    ) -> OsuResult<Vec<Score>> {
        RecentRequest::with_user_id(self.user_id)
            .limit(amount)
            .mode(mode)
            .queue(osu)
            .await
    }

    /// Count all 300s, 100s, and 50s of a user
    pub fn total_hits(&self) -> u64 {
        self.count300 as u64 + self.count100 as u64 + self.count50 as u64
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            user_id: 0,
            username: String::default(),
            join_date: Utc::now(),
            count300: 0,
            count100: 0,
            count50: 0,
            playcount: 0,
            ranked_score: 0,
            total_score: 0,
            pp_rank: 0,
            level: 0.0,
            pp_raw: 0.0,
            accuracy: 0.0,
            count_ssh: 0,
            count_ss: 0,
            count_sh: 0,
            count_s: 0,
            count_a: 0,
            country: String::default(),
            total_seconds_played: 0,
            pp_country_rank: 0,
            events: Vec::default(),
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl Eq for User {}

/// Event struct for events within the [`User`] struct.
///
/// [`User`]: struct.User.html
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Event {
    #[serde(alias = "display_html")]
    pub html: String,
    #[serde(
        deserialize_with = "to_maybe_u32",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub beatmap_id: Option<u32>,
    #[serde(
        deserialize_with = "to_maybe_u32",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub beatmapset_id: Option<u32>,
    #[serde(with = "serde_date")]
    pub date: DateTime<Utc>,
    #[serde(alias = "epicfactor", deserialize_with = "to_u32")]
    pub epic_factor: u32,
}

impl Event {
    pub fn new(
        html: String,
        beatmap_id: Option<u32>,
        beatmapset_id: Option<u32>,
        date: DateTime<Utc>,
        epic_factor: u32,
    ) -> Self {
        Self {
            html,
            beatmap_id,
            beatmapset_id,
            date,
            epic_factor,
        }
    }
}
