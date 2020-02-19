use crate::{
    backend::{deserialize::*, OsuApi},
    models::HasLazies,
};
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use std::sync::{Arc, RwLock};

/// User struct retrieved from the `/api/get_user` endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    #[serde(deserialize_with = "str_to_u32")]
    pub user_id: u32,
    pub username: String,
    #[serde(deserialize_with = "str_to_date")]
    pub join_date: DateTime<Utc>,
    #[serde(deserialize_with = "str_to_u32")]
    pub count300: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub count100: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub count50: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub playcount: u32,
    #[serde(deserialize_with = "str_to_u64")]
    pub ranked_score: u64,
    #[serde(deserialize_with = "str_to_u64")]
    pub total_score: u64,
    #[serde(deserialize_with = "str_to_u32")]
    pub pp_rank: u32,
    #[serde(deserialize_with = "str_to_f32")]
    pub level: f32,
    #[serde(deserialize_with = "str_to_f32")]
    pub pp_raw: f32,
    #[serde(deserialize_with = "str_to_f32")]
    pub accuracy: f32,
    #[serde(rename = "count_rank_ssh", deserialize_with = "str_to_u32")]
    pub count_ssh: u32,
    #[serde(rename = "count_rank_ss", deserialize_with = "str_to_u32")]
    pub count_ss: u32,
    #[serde(rename = "count_rank_sh", deserialize_with = "str_to_u32")]
    pub count_sh: u32,
    #[serde(rename = "count_rank_s", deserialize_with = "str_to_u32")]
    pub count_s: u32,
    #[serde(rename = "count_rank_a", deserialize_with = "str_to_u32")]
    pub count_a: u32,
    pub country: String,
    #[serde(deserialize_with = "str_to_u32")]
    pub total_seconds_played: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub pp_country_rank: u32,
    pub events: Vec<Event>,
}

impl User {
    pub fn get_total_hits(&self) -> u64 {
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

impl HasLazies for User {
    fn prepare_lazies(&mut self, _: Arc<RwLock<OsuApi>>) {}
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl Eq for User {}

/// Event struct for events whithin the `User` struct.
/// Since some events, like acquiring/extending supporter
/// status, do not include map id and mapset id, those
/// fields are whithin an `Option`
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Event {
    html: String,
    #[serde(deserialize_with = "str_to_maybe_u32")]
    beatmap_id: Option<u32>,
    #[serde(deserialize_with = "str_to_maybe_u32")]
    beatmapset_id: Option<u32>,
    #[serde(deserialize_with = "str_to_date")]
    date: DateTime<Utc>,
    #[serde(deserialize_with = "str_to_u32")]
    epic_factor: u32,
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
