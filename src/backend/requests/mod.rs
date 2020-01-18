mod maps;
mod osu_request;
mod scores;
mod user_best;
mod user_recent;
mod users;

pub use maps::BeatmapRequest;
pub use osu_request::OsuRequest;
pub use scores::ScoreRequest;
pub use user_best::UserBestRequest;
pub use user_recent::UserRecentRequest;
pub use users::UserRequest;

use std::{collections::HashMap, fmt::Debug};

pub(crate) const API_BASE: &str = "https://osu.ppy.sh/api/";

pub(crate) const USER_TAG: char = 'u';
pub(crate) const MODE_TAG: char = 'm';
pub(crate) const SET_TAG: char = 's';
pub(crate) const MAP_TAG: char = 'b';
pub(crate) const SINCE_TAG: &str = "since";
pub(crate) const CONV_TAG: char = 'a';
pub(crate) const HASH_TAG: char = 'h';
pub(crate) const LIMIT_TAG: &str = "limit";
pub(crate) const MODS_TAG: &str = "mods";
pub(crate) const EVENT_DAYS_TAG: &str = "event_days";

pub(crate) const USER_ENDPOINT: &str = "get_user";
pub(crate) const BEATMAP_ENDPOINT: &str = "get_beatmaps";
pub(crate) const SCORE_ENDPOINT: &str = "get_scores";
pub(crate) const USER_BEST_ENDPOINT: &str = "get_user_best";
pub(crate) const USER_RECENT_ENDPOINT: &str = "get_user_recent";

/// Helper trait to allow arbitrary requests as parameter for `Osu`'s `prepare_request` method.
pub trait Request {
    /// Artifact from the public `Request` trait. This method has no use outside of this library.
    fn add_args(self, args: &mut HashMap<String, String>) -> RequestType;
}

/// Artifact from the public `Request` struct. Helps to differentiate internally between requests of a generic type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequestType {
    User,
    Beatmap,
    Score,
    UserBest,
    UserRecent,
}

impl RequestType {
    pub(crate) fn get_endpoint(self) -> &'static str {
        match self {
            RequestType::User => USER_ENDPOINT,
            RequestType::Beatmap => BEATMAP_ENDPOINT,
            RequestType::Score => SCORE_ENDPOINT,
            RequestType::UserBest => USER_BEST_ENDPOINT,
            RequestType::UserRecent => USER_RECENT_ENDPOINT,
        }
    }
}
