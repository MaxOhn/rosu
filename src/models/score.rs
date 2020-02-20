use crate::{
    backend::{
        deserialize::*,
        requests::{BeatmapArgs, OsuArgs, UserArgs},
    },
    models::{Beatmap, GameMod, GameMode, GameMods, Grade, User},
    Osu, OsuError,
};
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

/// Score struct retrieved from `/api/get_scores`, `/api/get_user_best`,
/// and `/api/get_user_recent` endpoints.
/// Although the `/api/get_scores` endpoint fills most fields, the other
/// two endpoints do not. Hence, some fields are within an `Option`
#[derive(Debug, Clone, Deserialize)]
pub struct Score {
    #[serde(default, deserialize_with = "str_to_maybe_u32")]
    pub beatmap_id: Option<u32>,
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
    pub enabled_mods: GameMods,
    #[serde(deserialize_with = "str_to_date")]
    pub date: DateTime<Utc>,
    #[serde(rename = "rank", deserialize_with = "str_to_grade")]
    pub grade: Grade,
    #[serde(default, deserialize_with = "str_to_maybe_f32")]
    pub pp: Option<f32>,
    #[serde(default, deserialize_with = "str_to_maybe_bool")]
    pub replay_available: Option<bool>,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            beatmap_id: None,
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
            enabled_mods: GameMods::default(),
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

impl Score {
    /// Retrieve the beatmap of the score from the API
    pub async fn get_beatmap(&self, osu: &Osu) -> Result<Beatmap, OsuError> {
        if let Some(id) = self.beatmap_id {
            let args = OsuArgs::Beatmaps(BeatmapArgs::new().map_id(id));
            let mut maps: Vec<Beatmap> = osu.create_request(args).queue().await?;
            maps.pop()
                .ok_or_else(|| OsuError::Other(format!("No beatmap with id {} was found", id)))
        } else {
            Err(OsuError::Other(
                "Cannot retrieve beatmap of a score without beatmap id".to_string(),
            ))
        }
    }

    /// Retrieve the user of the score from the API
    pub async fn get_user(&self, osu: &Osu, mode: GameMode) -> Result<User, OsuError> {
        let args = OsuArgs::Users(UserArgs::with_user_id(self.user_id).mode(mode));
        let mut users: Vec<User> = osu.create_request(args).queue().await?;
        users
            .pop()
            .ok_or_else(|| OsuError::Other(format!("No user with id {} was found", self.user_id)))
    }

    /// Count all hitobjects of the score i.e. for `GameMode::STD` the amount 300s, 100s, 50s, and misses.
    pub fn get_amount_hits(&self, mode: GameMode) -> u32 {
        let mut amount = self.count300 + self.count100 + self.count_miss;
        if mode != GameMode::TKO {
            amount += self.count50;
            if mode != GameMode::STD {
                amount += self.count_katu;
                if mode != GameMode::CTB {
                    amount += self.count_geki;
                }
            }
        }
        amount
    }

    /// Provided the `GameMode`, calculate the accuracy of the score
    /// i.e. 0 <= accuracy <= 100.
    pub fn get_accuracy(&self, mode: GameMode) -> f32 {
        let amount_objects = self.get_amount_hits(mode) as f32;
        let (numerator, denumerator) = {
            match mode {
                GameMode::TKO => (
                    0.5 * self.count100 as f32 + self.count300 as f32,
                    amount_objects,
                ),
                GameMode::CTB => (
                    (self.count300 + self.count100 + self.count50) as f32,
                    amount_objects,
                ),
                GameMode::STD | GameMode::MNA => {
                    let mut n =
                        (self.count50 * 50 + self.count100 * 100 + self.count300 * 300) as f32;
                    if mode == GameMode::MNA {
                        n += (self.count_katu * 200 + self.count_geki * 300) as f32;
                    }
                    (n, amount_objects * 300.0)
                }
            }
        };
        (10_000.0 * numerator / denumerator).round() / 100.0
    }

    /// Provided the `GameMode` and optionally the accuracy of the score,
    /// recalculate the grade of the score and return the result.
    /// The accuracy is only required for non-`GameMode::STD` scores and is
    /// calculated if not already provided. This method assumes the score to
    /// be a pass i.e. the amount of passed objects is equal to the beatmaps
    /// total amount of objects. Otherwise, it may produce an incorrect grade.
    pub fn recalculate_grade(&mut self, mode: GameMode, accuracy: Option<f32>) -> Grade {
        let passed_objects = self.get_amount_hits(mode);
        self.grade = match mode {
            GameMode::STD => self.get_osu_grade(passed_objects),
            GameMode::MNA => self.get_mania_grade(passed_objects, accuracy),
            GameMode::TKO => self.get_taiko_grade(passed_objects, accuracy),
            GameMode::CTB => self.get_ctb_grade(accuracy),
        };
        self.grade
    }

    fn get_osu_grade(&self, passed_objects: u32) -> Grade {
        if self.count300 == passed_objects {
            return if self.enabled_mods.contains(&GameMod::Hidden) {
                Grade::XH
            } else {
                Grade::X
            };
        }
        let ratio300 = self.count300 as f32 / passed_objects as f32;
        let ratio50 = self.count50 as f32 / passed_objects as f32;
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

    fn get_mania_grade(&self, passed_objects: u32, accuracy: Option<f32>) -> Grade {
        if self.count_geki == passed_objects {
            return if self.enabled_mods.contains(&GameMod::Hidden) {
                Grade::XH
            } else {
                Grade::X
            };
        }
        let accuracy = accuracy.unwrap_or_else(|| self.get_accuracy(GameMode::MNA));
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

    fn get_taiko_grade(&self, passed_objects: u32, accuracy: Option<f32>) -> Grade {
        if self.count300 == passed_objects {
            return if self.enabled_mods.contains(&GameMod::Hidden) {
                Grade::XH
            } else {
                Grade::X
            };
        }
        let accuracy = accuracy.unwrap_or_else(|| self.get_accuracy(GameMode::TKO));
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

    fn get_ctb_grade(&self, accuracy: Option<f32>) -> Grade {
        let accuracy = accuracy.unwrap_or_else(|| self.get_accuracy(GameMode::CTB));
        if (100.0 - accuracy).abs() <= std::f32::EPSILON {
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
}
