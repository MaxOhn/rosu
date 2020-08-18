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

use crate::{OsuError, OsuResult};

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
    pub(crate) fn create_url(
        endpoint: &str,
        args: Vec<(&'static str, String)>,
    ) -> OsuResult<String> {
        let base = format!("{}{}", API_BASE, endpoint);
        let url = reqwest::Url::parse_with_params(&base, args)
            .map_err(|_| OsuError::ParseUrl)?
            .into_string();
        Ok(url)
    }
}
