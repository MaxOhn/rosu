use crate::{
    backend::{deserialize::*, LazilyLoaded, OsuApi, requests::{Request, UserArgs, BeatmapArgs}},
    models::{Beatmap, GameMod, GameMode, Grade, HasLazies, User},
};
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use std::sync::{Arc, RwLock};

/// Score struct retrieved from `/api/get_scores`, `/api/get_user_best`,
/// and `/api/get_user_recent` endpoints.
/// Although the `/api/get_scores` endpoint fills most fields, the other
/// two endpoints do not. Hence, some fields are within an `Option`
#[derive(Debug, Clone, Deserialize)]
pub struct Score {
    #[serde(default, deserialize_with = "str_to_maybe_u32")]
    pub beatmap_id: Option<u32>,
    #[serde(skip)]
    pub beatmap: Option<LazilyLoaded<Beatmap>>,
    #[serde(default, deserialize_with = "str_to_maybe_u32")]
    pub score_id: Option<u32>,
    #[serde(deserialize_with = "str_to_u32")]
    pub score: u32,
    #[serde(deserialize_with = "str_to_u32")]
    pub user_id: u32,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(skip)]
    pub user: LazilyLoaded<User>,
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
    #[serde(rename = "rank", deserialize_with = "str_to_grade")]
    pub grade: Grade,
    #[serde(default, deserialize_with = "str_to_maybe_f64")]
    pub pp: Option<f64>,
    #[serde(default, deserialize_with = "str_to_maybe_bool")]
    pub replay_available: Option<bool>,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            beatmap_id: None,
            beatmap: None,
            score_id: None,
            score: 0,
            user_id: 0,
            user: LazilyLoaded::default(),
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
            grade: Grade::F,
            pp: None,
            replay_available: None,
        }
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.beatmap_id == other.beatmap_id
            && self.user_id == other.user_id
            && self.score == other.score
    }
}

impl Eq for Score {}

impl HasLazies for Score {
    fn prepare_lazies(&mut self, osu: Arc<RwLock<OsuApi>>) {
        if let Some(id) = self.beatmap_id {
            let args = BeatmapArgs::new().map_id(id);
            let request = Request::Beatmaps(args);
            self.beatmap = Some(LazilyLoaded::new(osu.clone(), id, request));
        }
        let args = UserArgs::with_user_id(self.user_id);
        let request = Request::Users(args);
        self.user = LazilyLoaded::new(osu, self.user_id, request);
    }
}

impl Score {
    /// Provided the `GameMode`, calculate the accuracy of the score
    /// i.e. 0 <= accuracy <= 100.
    pub fn get_accuracy(&self, mode: GameMode) -> f64 {
        let mut amount_objects = self.count300 + self.count100 + self.count_miss;
        if mode != GameMode::TKO {
            amount_objects += self.count50;
            if mode != GameMode::STD {
                amount_objects += self.count_katu;
                if mode != GameMode::CTB {
                    amount_objects += self.count_geki;
                }
            }
        }
        let (numerator, denumerator) = {
            let mut n: f64 = 0.0;
            let mut d: f64 = amount_objects as f64;
            match mode {
                GameMode::TKO => n = 0.5 * self.count100 as f64 + self.count300 as f64,
                GameMode::CTB => n = (self.count300 + self.count100 + self.count50) as f64,
                GameMode::STD | GameMode::MNA => {
                    if mode == GameMode::MNA {
                        n += (self.count_katu * 200 + self.count_geki * 300) as f64;
                    }
                    n += (self.count50 * 50 + self.count100 * 100 + self.count300 * 300) as f64;
                    d *= 300.0;
                }
            }
            (n, d)
        };
        (10_000.0 * numerator / denumerator).round() / 100.0
    }

    #[allow(clippy::cognitive_complexity)]
    /// Provided the `GameMode` and optionally the accuracy of the score,
    /// recalculate the grade of the score and return the result.
    /// The accuracy is only required for non-`GameMode::STD` scores and is
    /// calculated if not already provided. This method assumes the score to
    /// be a pass i.e. the amount of passed objects is equal to the beatmaps
    /// total amount of objects. Otherwise, it may produce an incorrect grade.
    pub fn recalculate_grade(&mut self, mode: GameMode, accuracy: Option<f64>) -> Grade {
        let mut amount_objects = self.count300 + self.count100 + self.count_miss;
        if mode != GameMode::TKO {
            amount_objects += self.count50;
            if mode != GameMode::STD {
                amount_objects += self.count_katu;
                if mode != GameMode::CTB {
                    amount_objects += self.count_geki;
                }
            }
        }
        self.grade = match mode {
            GameMode::STD => {
                if self.count300 == amount_objects {
                    self.grade = if self.enabled_mods.contains(&GameMod::Hidden) {
                        Grade::XH
                    } else {
                        Grade::X
                    };
                    return self.grade;
                }
                let ratio300 = self.count300 as f64 / amount_objects as f64;
                let ratio50 = self.count50 as f64 / amount_objects as f64;
                if ratio300 > 0.9 && ratio50 < 0.01 && self.count_miss == 0 {
                    if self.enabled_mods.contains(&GameMod::Hidden) {
                        Grade::SH
                    } else {
                        Grade::S
                    }
                } else if ratio300 > 0.9 || (ratio300 > 0.8 && self.count_miss == 0) {
                    Grade::A
                } else if ratio300 > 0.8 || (ratio300 > 0.7 && self.count_miss == 0) {
                    Grade::B
                } else if ratio300 > 0.6 {
                    Grade::C
                } else {
                    Grade::D
                }
            }
            GameMode::MNA => {
                if self.count_geki == amount_objects {
                    self.grade = if self.enabled_mods.contains(&GameMod::Hidden) {
                        Grade::XH
                    } else {
                        Grade::X
                    };
                    return self.grade;
                }
                let accuracy = accuracy.unwrap_or_else(|| self.get_accuracy(mode));
                if accuracy > 95.0 {
                    if self.enabled_mods.contains(&GameMod::Hidden) {
                        Grade::SH
                    } else {
                        Grade::S
                    }
                } else if accuracy > 90.0 {
                    Grade::A
                } else if accuracy > 80.0 {
                    Grade::B
                } else if accuracy > 70.0 {
                    Grade::C
                } else {
                    Grade::D
                }
            }
            GameMode::TKO => {
                if self.count300 == amount_objects {
                    self.grade = if self.enabled_mods.contains(&GameMod::Hidden) {
                        Grade::XH
                    } else {
                        Grade::X
                    };
                    return self.grade;
                }
                let accuracy = accuracy.unwrap_or_else(|| self.get_accuracy(mode));
                if accuracy > 95.0 {
                    if self.enabled_mods.contains(&GameMod::Hidden) {
                        Grade::SH
                    } else {
                        Grade::S
                    }
                } else if accuracy > 90.0 {
                    Grade::A
                } else if accuracy > 80.0 {
                    Grade::B
                } else {
                    Grade::C
                }
            }
            GameMode::CTB => {
                let accuracy = accuracy.unwrap_or_else(|| self.get_accuracy(mode));
                if (100.0 - accuracy).abs() <= std::f64::EPSILON {
                    if self.enabled_mods.contains(&GameMod::Hidden) {
                        Grade::XH
                    } else {
                        Grade::X
                    }
                } else if accuracy > 98.0 {
                    if self.enabled_mods.contains(&GameMod::Hidden) {
                        Grade::SH
                    } else {
                        Grade::S
                    }
                } else if accuracy > 94.0 {
                    Grade::A
                } else if accuracy > 90.0 {
                    Grade::B
                } else if accuracy > 85.0 {
                    Grade::C
                } else {
                    Grade::D
                }
            }
        };
        self.grade
    }
}
