use crate::backend::deserialize::*;
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    #[serde(deserialize_with = "u32_from_str")]
    pub user_id: u32,
    pub username: String,
    #[serde(deserialize_with = "date_from_str")]
    pub join_date: DateTime<Utc>,
    #[serde(deserialize_with = "u32_from_str")]
    pub count300: u32,
    #[serde(deserialize_with = "u32_from_str")]
    pub count100: u32,
    #[serde(deserialize_with = "u32_from_str")]
    pub count50: u32,
    #[serde(deserialize_with = "u32_from_str")]
    pub playcount: u32,
    #[serde(deserialize_with = "u64_from_str")]
    pub ranked_score: u64,
    #[serde(deserialize_with = "u64_from_str")]
    pub total_score: u64,
    #[serde(deserialize_with = "u32_from_str")]
    pub pp_rank: u32,
    #[serde(deserialize_with = "f64_from_str")]
    pub level: f64,
    #[serde(deserialize_with = "f64_from_str")]
    pub pp_raw: f64,
    #[serde(deserialize_with = "f64_from_str")]
    pub accuracy: f64,
    #[serde(rename = "count_rank_ssh", deserialize_with = "u32_from_str")]
    pub count_ssh: u32,
    #[serde(rename = "count_rank_ss", deserialize_with = "u32_from_str")]
    pub count_ss: u32,
    #[serde(rename = "count_rank_sh", deserialize_with = "u32_from_str")]
    pub count_sh: u32,
    #[serde(rename = "count_rank_s", deserialize_with = "u32_from_str")]
    pub count_s: u32,
    #[serde(rename = "count_rank_a", deserialize_with = "u32_from_str")]
    pub count_a: u32,
    pub country: String,
    #[serde(deserialize_with = "u32_from_str")]
    pub total_seconds_played: u32,
    #[serde(deserialize_with = "u32_from_str")]
    pub pp_country_rank: u32,
    //#[serde(skip)]
    //pub events: String, //TODO
}

impl User {
    pub fn new(
        user_id: u32,
        username: String,
        join_date: DateTime<Utc>,
        count300: u32,
        count100: u32,
        count50: u32,
        playcount: u32,
        ranked_score: u64,
        total_score: u64,
        pp_rank: u32,
        level: f64,
        pp_raw: f64,
        accuracy: f64,
        count_ssh: u32,
        count_ss: u32,
        count_sh: u32,
        count_s: u32,
        count_a: u32,
        country: String,
        total_seconds_played: u32,
        pp_country_rank: u32,
        //events: String,
    ) -> Self {
        Self {
            user_id,
            username,
            join_date,
            count300,
            count100,
            count50,
            playcount,
            ranked_score,
            total_score,
            pp_rank,
            level,
            pp_raw,
            accuracy,
            count_ssh,
            count_ss,
            count_sh,
            count_s,
            count_a,
            country,
            total_seconds_played,
            pp_country_rank,
            //events,
        }
    }

    pub fn default() -> Self {
        Self {
            user_id: 0,
            username: "".to_owned(),
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
            country: "".to_owned(),
            total_seconds_played: 0,
            pp_country_rank: 0,
            //events: "".to_owned(),
        }
    }
}
