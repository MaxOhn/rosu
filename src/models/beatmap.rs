use crate::{
    backend::{
        deserialize::*,
        requests::{OsuArgs, UserArgs},
        Osu,
    },
    models::{ApprovalStatus, GameMode, Genre, Language, User},
    OsuError,
};
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use std::fmt;

/// Beatmap struct retrieved from the `/api/get_beatmaps` endpoint.
/// Some fields are returned as `null` from the api in some cases,
/// hence they're in an `Option`
#[derive(Debug, Clone, Deserialize)]
pub struct Beatmap {
    #[serde(rename = "approved", deserialize_with = "str_to_approved")]
    pub approval_status: ApprovalStatus,
    #[serde(deserialize_with = "str_to_date")]
    pub submit_date: DateTime<Utc>,
    #[serde(deserialize_with = "str_to_maybe_date")]
    pub approved_date: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "str_to_date")]
    pub last_update: DateTime<Utc>,
    pub artist: String,
    pub title: String,
    pub version: String,
    #[serde(deserialize_with = "str_to_u32")]
    pub beatmap_id: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub beatmapset_id: u32,
    #[serde(deserialize_with = "str_to_f32")]
    pub bpm: f32,
    pub creator: String,
    #[serde(deserialize_with = "str_to_u32")]
    pub creator_id: u32,
    #[serde(rename = "difficultyrating", deserialize_with = "str_to_f32")]
    pub stars: f32,
    #[serde(rename = "diff_aim", deserialize_with = "str_to_maybe_f32")]
    pub stars_aim: Option<f32>,
    #[serde(rename = "diff_speed", deserialize_with = "str_to_maybe_f32")]
    pub stars_speed: Option<f32>,
    #[serde(rename = "diff_size", deserialize_with = "str_to_f32")]
    pub diff_cs: f32,
    #[serde(rename = "diff_overall", deserialize_with = "str_to_f32")]
    pub diff_od: f32,
    #[serde(rename = "diff_approach", deserialize_with = "str_to_f32")]
    pub diff_ar: f32,
    #[serde(rename = "diff_drain", deserialize_with = "str_to_f32")]
    pub diff_hp: f32,
    #[serde(rename = "hit_length", deserialize_with = "str_to_u32")]
    pub seconds_drain: u32,
    #[serde(rename = "total_length", deserialize_with = "str_to_u32")]
    pub seconds_total: u32,
    pub source: String,
    #[serde(rename = "genre_id", deserialize_with = "str_to_genre")]
    pub genre: Genre,
    #[serde(rename = "language_id", deserialize_with = "str_to_language")]
    pub language: Language,
    #[serde(deserialize_with = "str_to_mode")]
    pub mode: GameMode,
    pub tags: String,
    #[serde(deserialize_with = "str_to_u32")]
    pub favourite_count: u32,
    #[serde(deserialize_with = "str_to_f32")]
    pub rating: f32,
    #[serde(deserialize_with = "str_to_u32")]
    pub playcount: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub passcount: u32,
    #[serde(rename = "count_normal", deserialize_with = "str_to_u32")]
    pub count_circle: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub count_slider: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub count_spinner: u32,
    #[serde(deserialize_with = "str_to_maybe_u32")]
    pub max_combo: Option<u32>,
    #[serde(deserialize_with = "str_to_bool")]
    pub download_unavailable: bool,
    #[serde(deserialize_with = "str_to_bool")]
    pub audio_unavailable: bool,
    pub file_md5: String,
}

impl fmt::Display for Beatmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} [{}]", self.artist, self.title, self.version)
    }
}

impl Default for Beatmap {
    fn default() -> Self {
        Self {
            beatmap_id: 0,
            beatmapset_id: 0,
            artist: String::default(),
            title: String::default(),
            version: String::default(),
            mode: GameMode::default(),
            creator: String::default(),
            creator_id: 0,
            seconds_drain: 0,
            seconds_total: 0,
            bpm: 0.0,
            stars: 0.0,
            stars_aim: None,
            stars_speed: None,
            diff_cs: 0.0,
            diff_od: 0.0,
            diff_ar: 0.0,
            diff_hp: 0.0,
            playcount: 0,
            passcount: 0,
            count_circle: 0,
            count_slider: 0,
            count_spinner: 0,
            max_combo: None,
            source: String::default(),
            genre: Genre::default(),
            language: Language::default(),
            tags: String::default(),
            favourite_count: 0,
            rating: 0.0,
            download_unavailable: true,
            audio_unavailable: true,
            file_md5: String::default(),
            approval_status: ApprovalStatus::WIP,
            submit_date: Utc::now(),
            approved_date: None,
            last_update: Utc::now(),
        }
    }
}

impl Beatmap {
    /// Retrieve the creator of the beatmap
    pub async fn get_creator(&self, osu: &Osu) -> Result<User, OsuError> {
        let args = OsuArgs::Users(UserArgs::with_user_id(self.creator_id));
        let mut users: Vec<User> = osu.create_request(args).queue().await?;
        users.pop().ok_or_else(|| {
            OsuError::Other(format!("No user with id {} was found", self.creator_id))
        })
    }

    /// Count all circles, sliders, and spinners of the beatmap
    pub fn count_objects(&self) -> u32 {
        self.count_circle + self.count_slider + self.count_spinner
    }
}

impl PartialEq for Beatmap {
    fn eq(&self, other: &Self) -> bool {
        self.beatmap_id == other.beatmap_id
    }
}

impl Eq for Beatmap {}
