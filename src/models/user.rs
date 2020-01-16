use crate::{backend::deserialize::*, models::Event};
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

/// User struct retrieved from the `/api/get_user` endpoint
#[derive(Debug, Clone, PartialEq, Deserialize)]
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
    #[serde(deserialize_with = "str_to_f64")]
    pub level: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub pp_raw: f64,
    #[serde(deserialize_with = "str_to_f64")]
    pub accuracy: f64,
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
    pub fn default() -> Self {
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
