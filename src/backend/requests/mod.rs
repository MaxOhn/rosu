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

use std::collections::HashMap;

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

pub(crate) trait Request {
    /// Combining all arguments of a request into a `HashMap`,
    /// pairing their url tags with values.
    fn prepare_args<'s>(&self) -> HashMap<&'s str, String>;

    /// Constructing the initial url for a request.
    /// Resulting url does not contain the API key yet.
    fn get_url(&self, endpoint: &str) -> String {
        let args = self.prepare_args();
        let mut url = format!("{}{}?", API_BASE, endpoint);
        let query: String = args
            .iter()
            .map(|(tag, val)| format!("{}={}", tag, val))
            .collect::<Vec<String>>()
            .join("&");
        url.push_str(&query);
        url
    }
}
