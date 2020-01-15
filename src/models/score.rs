use crate::{backend::deserialize::*, models::GameMod};
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Score {
    #[serde(deserialize_with = "str_to_u32")]
    pub score_id: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub score: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub user_id: u32,
    #[serde(default)]
    pub username: String,
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
    #[serde(deserialize_with = "str_to_f64")]
    pub pp: f64,
    #[serde(deserialize_with = "str_to_bool")]
    pub replay_available: bool,
}

impl Score {
    pub fn default() -> Self {
        Self {
            score_id: 0,
            score: 0,
            user_id: 0,
            username: String::default(),
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
            pp: 0.0,
            replay_available: false,
        }
    }
}
