use crate::{
    backend::{
        deserialize::*,
        requests::{ScoreRequest, UserRequest},
        Osu,
    },
    models::{GameMode, Score, User},
    OsuError, OsuResult,
};

use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use std::fmt;

/// Beatmap struct retrieved from the `/api/get_beatmaps` endpoint.
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
    /// Retrieve the creator of the beatmap from the API
    pub async fn get_creator(&self, osu: &Osu, mode: GameMode) -> OsuResult<User> {
        UserRequest::with_user_id(self.creator_id)
            .mode(mode)
            .queue_single(osu)
            .await?
            .ok_or_else(|| OsuError::Other("Beatmap creator was not found"))
    }

    /// Retrieve the global top scores of the beatmap from the API (0 < amount <= 100)
    pub async fn get_global_leaderboard(&self, osu: &Osu, amount: u32) -> OsuResult<Vec<Score>> {
        ScoreRequest::with_map_id(self.beatmap_id)
            .limit(amount)
            .mode(self.mode)
            .queue(osu)
            .await
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

/// Basic enum to describe a [`Beatmap`]'s music genre
///
/// [`Beatmap`]: struct.Beatmap.html
#[derive(Debug, Clone, Hash, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Genre {
    Any = 0,
    Unspecified = 1,
    VideoGame = 2,
    Anime = 3,
    Rock = 4,
    Pop = 5,
    Other = 6,
    Novelty = 7,
    HipHop = 9,
    Electronic = 10,
    Metal = 11,
    Classical = 12,
    Folk = 13,
    Jazz = 14,
}

impl Default for Genre {
    fn default() -> Self {
        Self::Any
    }
}

impl From<u8> for Genre {
    fn from(g: u8) -> Self {
        match g {
            1 => Self::Unspecified,
            2 => Self::VideoGame,
            3 => Self::Anime,
            4 => Self::Rock,
            5 => Self::Pop,
            6 => Self::Other,
            7 => Self::Novelty,
            9 => Self::HipHop,
            10 => Self::Electronic,
            11 => Self::Metal,
            12 => Self::Classical,
            13 => Self::Folk,
            14 => Self::Jazz,
            _ => Self::Any,
        }
    }
}

/// Basic enum to describe a [`Beatmap`]'s music language
///
/// [`Beatmap`]: struct.Beatmap.html
#[derive(Debug, Clone, Hash, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Language {
    Any = 0,
    Other = 1,
    English = 2,
    Japanese = 3,
    Chinese = 4,
    Instrumental = 5,
    Korean = 6,
    French = 7,
    German = 8,
    Swedish = 9,
    Spanish = 10,
    Italian = 11,
    Russian = 12,
    Polish = 13,
    Unspecified = 14,
}

impl Default for Language {
    fn default() -> Self {
        Self::Any
    }
}

impl From<u8> for Language {
    fn from(language: u8) -> Self {
        match language {
            1 => Self::Other,
            2 => Self::English,
            3 => Self::Japanese,
            4 => Self::Chinese,
            5 => Self::Instrumental,
            6 => Self::Korean,
            7 => Self::French,
            8 => Self::German,
            9 => Self::Swedish,
            10 => Self::Spanish,
            11 => Self::Italian,
            12 => Self::Russian,
            13 => Self::Polish,
            14 => Self::Unspecified,
            _ => Self::Any,
        }
    }
}

/// Basic enum to describe a [`Beatmap`]'s approval status
///
/// [`Beatmap`]: struct.Beatmap.html
#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
#[repr(i8)]
pub enum ApprovalStatus {
    Loved = 4,
    Qualified = 3,
    Approved = 2,
    Ranked = 1,
    Pending = 0,
    WIP = -1,
    Graveyard = -2,
}

impl From<i8> for ApprovalStatus {
    fn from(m: i8) -> Self {
        match m {
            4 => Self::Loved,
            3 => Self::Qualified,
            2 => Self::Approved,
            1 => Self::Ranked,
            0 => Self::Pending,
            -1 => Self::WIP,
            -2 => Self::Graveyard,
            _ => panic!("Can not parse {} into ApprovalStatus", m),
        }
    }
}
