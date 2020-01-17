use crate::{backend::deserialize::*, models::GameMod};
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

/// Score struct retrieved from `/api/get_scores`, `/api/get_user_best`, and `/api/get_user_recent` endpoints
/// Although the `/api/get_scores` endpoint fills all fields, the other
/// two endpoints do not. Hence, some fields are within an `Option`
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Score {
    #[serde(default, deserialize_with = "str_to_maybe_u32")]
    pub score_id: Option<u32>,
    #[serde(deserialize_with = "str_to_u32")]
    pub score: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub user_id: u32,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(deserialize_with = "str_to_u32")]
    pub count300: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub count100: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub count50: u32,
    #[serde(rename = "countmiss", deserialize_with = "str_to_u32")]
    pub count_miss: u32,
    #[serde(rename = "countgeki", deserialize_with = "str_to_u32")]
    pub count_geki: u32,
    #[serde(rename = "countkatu", deserialize_with = "str_to_u32")]
    pub count_katu: u32,
    #[serde(rename = "maxcombo", deserialize_with = "str_to_u32")]
    pub max_combo: u32,
    #[serde(deserialize_with = "str_to_bool")]
    pub perfect: bool,
    #[serde(deserialize_with = "str_to_mods")]
    pub enabled_mods: Vec<GameMod>,
    #[serde(deserialize_with = "str_to_date")]
    pub date: DateTime<Utc>,
    #[serde(rename = "rank")]
    pub grade: String,
    #[serde(default, deserialize_with = "str_to_maybe_f64")]
    pub pp: Option<f64>,
    #[serde(default, deserialize_with = "str_to_maybe_bool")]
    pub replay_available: Option<bool>,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            score_id: None,
            score: 0,
            user_id: 0,
            username: None,
            count300: 0,
            count100: 0,
            count50: 0,
            count_geki: 0,
            count_katu: 0,
            count_miss: 0,
            max_combo: 0,
            perfect: false,
            enabled_mods: Vec::default(),
            date: Utc::now(),
            grade: String::default(),
            pp: None,
            replay_available: None,
        }
    }
}
