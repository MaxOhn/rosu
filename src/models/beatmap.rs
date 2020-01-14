use crate::{
    backend::deserialize::*,
    models::{ApprovalStatus, GameMode, Genre, Language},
};
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Beatmap {
    #[serde(rename = "approved", deserialize_with = "str_to_approved")]
    approval_status: ApprovalStatus,
    #[serde(deserialize_with = "str_to_date")]
    submit_date: DateTime<Utc>,
    #[serde(deserialize_with = "str_to_maybe_date")]
    approved_date: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "str_to_date")]
    last_update: DateTime<Utc>,
    artist: String,
    title: String,
    version: String,
    #[serde(deserialize_with = "str_to_u32")]
    beatmap_id: u32,
    #[serde(deserialize_with = "str_to_u32")]
    beatmapset_id: u32,
    #[serde(deserialize_with = "str_to_u32")]
    bpm: u32, // keep an eye on
    creator: String,
    #[serde(deserialize_with = "str_to_u32")]
    creator_id: u32,
    #[serde(rename = "difficultyrating", deserialize_with = "str_to_f64")]
    stars: f64,
    #[serde(rename = "diff_aim", deserialize_with = "str_to_f64")]
    stars_aim: f64,
    #[serde(rename = "diff_speed", deserialize_with = "str_to_f64")]
    stars_speed: f64,
    #[serde(rename = "diff_size", deserialize_with = "str_to_f64")]
    diff_cs: f64,
    #[serde(rename = "diff_overall", deserialize_with = "str_to_f64")]
    diff_od: f64,
    #[serde(rename = "diff_approach", deserialize_with = "str_to_f64")]
    diff_ar: f64,
    #[serde(rename = "diff_drain", deserialize_with = "str_to_f64")]
    diff_hp: f64,
    #[serde(rename = "hit_length", deserialize_with = "str_to_u32")]
    seconds_drain: u32,
    #[serde(rename = "total_length", deserialize_with = "str_to_u32")]
    seconds_total: u32,
    source: String,
    #[serde(rename = "genre_id", deserialize_with = "str_to_genre")]
    genre: Genre,
    #[serde(rename = "language_id", deserialize_with = "str_to_language")]
    language: Language,
    #[serde(deserialize_with = "str_to_mode")]
    mode: GameMode,
    tags: String,
    #[serde(deserialize_with = "str_to_u32")]
    favourite_count: u32,
    #[serde(deserialize_with = "str_to_f64")]
    rating: f64,
    #[serde(deserialize_with = "str_to_u32")]
    playcount: u32,
    #[serde(deserialize_with = "str_to_u32")]
    passcount: u32,
    #[serde(rename = "count_normal", deserialize_with = "str_to_u32")]
    count_circle: u32,
    #[serde(deserialize_with = "str_to_u32")]
    count_slider: u32,
    #[serde(deserialize_with = "str_to_u32")]
    count_spinner: u32,
    #[serde(deserialize_with = "str_to_u32")]
    max_combo: u32,
    #[serde(deserialize_with = "str_to_bool")]
    download_unavailable: bool,
    #[serde(deserialize_with = "str_to_bool")]
    audio_unavailable: bool,
    file_md5: String,
}

impl Beatmap {
    pub fn default(approval_status: ApprovalStatus, submit_date: DateTime<Utc>) -> Self {
        Self {
            approval_status,
            submit_date,
            approved_date: None,
            last_update: Utc::now(),
            artist: String::default(),
            title: String::default(),
            version: String::default(),
            beatmap_id: 0,
            beatmapset_id: 0,
            bpm: 0,
            creator: String::default(),
            creator_id: 0,
            stars: 0.0,
            stars_aim: 0.0,
            stars_speed: 0.0,
            diff_cs: 0.0,
            diff_od: 0.0,
            diff_ar: 0.0,
            diff_hp: 0.0,
            seconds_drain: 0,
            seconds_total: 0,
            source: String::default(),
            genre: Genre::default(),
            language: Language::default(),
            mode: GameMode::default(),
            tags: String::default(),
            favourite_count: 0,
            rating: 0.0,
            playcount: 0,
            passcount: 0,
            count_circle: 0,
            count_slider: 0,
            count_spinner: 0,
            max_combo: 0,
            download_unavailable: true,
            audio_unavailable: true,
            file_md5: String::default(),
        }
    }
}
