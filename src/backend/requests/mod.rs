mod maps;
mod r#match;
mod scores;
mod user_best;
mod user_recent;
mod users;

pub use maps::BeatmapRequest;
pub use r#match::MatchRequest;
pub use scores::ScoreRequest;
pub use user_best::BestRequest;
pub use user_recent::RecentRequest;
pub use users::UserRequest;

use std::{collections::HashMap, fmt::Write};

pub(crate) const API_BASE: &str = "https://osu.ppy.sh/api/";

pub(crate) const USER_TAG: &str = "u";
pub(crate) const MODE_TAG: &str = "m";
pub(crate) const SET_TAG: &str = "s";
pub(crate) const MAP_TAG: &str = "b";
pub(crate) const SINCE_TAG: &str = "since";
pub(crate) const TYPE_TAG: &str = "type";
pub(crate) const CONV_TAG: &str = "a";
pub(crate) const HASH_TAG: &str = "h";
pub(crate) const LIMIT_TAG: &str = "limit";
pub(crate) const MODS_TAG: &str = "mods";
pub(crate) const EVENT_DAYS_TAG: &str = "event_days";
pub(crate) const MP_TAG: &str = "mp";

pub(crate) const USER_ENDPOINT: &str = "get_user";
pub(crate) const BEATMAP_ENDPOINT: &str = "get_beatmaps";
pub(crate) const SCORE_ENDPOINT: &str = "get_scores";
pub(crate) const USER_BEST_ENDPOINT: &str = "get_user_best";
pub(crate) const USER_RECENT_ENDPOINT: &str = "get_user_recent";
pub(crate) const MATCH_ENDPOINT: &str = "get_match";

pub(crate) struct Request;

impl Request {
    pub(crate) fn create_url(endpoint: &str, args: HashMap<&str, String>) -> String {
        let len = API_BASE.len()
            + endpoint.len()
            + 44
            + args
                .iter()
                .fold(0, |sum, (tag, value)| sum + tag.len() + value.len() + 2);
        let mut url = String::with_capacity(len);
        let _ = write!(url, "{}{}?", API_BASE, endpoint);
        for (tag, val) in args {
            let _ = write!(url, "{}={}&", tag, val);
        }
        url
    }
}
