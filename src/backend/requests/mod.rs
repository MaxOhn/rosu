mod maps;
mod osu_request;
mod scores;
mod user_best;
mod user_recent;
mod users;

pub use maps::BeatmapArgs;
pub use osu_request::OsuRequest;
pub use scores::ScoreArgs;
pub use user_best::UserBestArgs;
pub use user_recent::UserRecentArgs;
pub use users::UserArgs;

use std::collections::HashMap;

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

#[derive(Clone)]
/// Wrapper for the different kind of requests.
pub enum Request {
    Users(UserArgs),
    Beatmaps(BeatmapArgs),
    Scores(ScoreArgs),
    Recent(UserRecentArgs),
    Best(UserBestArgs),
}

impl Request {
    fn get_args(&self) -> HashMap<String, String> {
        let mut args = HashMap::new();
        match self {
            Request::Users(u) => {
                if let Some(id) = u.user_id {
                    args.insert(USER_TAG.to_string(), id.to_string());
                } else if let Some(name) = &u.username {
                    args.insert(USER_TAG.to_string(), name.replace(" ", "%"));
                }
                if let Some(mode) = u.mode {
                    args.insert(MODE_TAG.to_string(), (mode as u8).to_string());
                }
                if let Some(amount) = u.event_days {
                    args.insert(EVENT_DAYS_TAG.to_owned(), amount.to_string());
                }
            },
            Request::Beatmaps(m) => {
                if let Some(since) = m.since {
                    args.insert(SINCE_TAG.to_owned(), since.format("%F%%T").to_string());
                }
                if let Some(id) = m.map_id {
                    args.insert(MAP_TAG.to_string(), id.to_string());
                }
                if let Some(id) = m.mapset_id {
                    args.insert(SET_TAG.to_string(), id.to_string());
                }
                if let Some(id) = m.user_id {
                    args.insert(USER_TAG.to_string(), id.to_string());
                } else if let Some(name) = &m.username {
                    args.insert(USER_TAG.to_string(), name.replace(" ", "%"));
                }
                if let Some(mode) = m.mode {
                    args.insert(MODE_TAG.to_string(), (mode as u8).to_string());
                }
                if let Some(limit) = m.limit {
                    args.insert(LIMIT_TAG.to_owned(), limit.to_string());
                }
                if let Some(mods) = m.mods {
                    args.insert(MODS_TAG.to_owned(), mods.to_string());
                }
                if let Some(with_converted) = m.with_converted {
                    args.insert(CONV_TAG.to_string(), (with_converted as u8).to_string());
                }
                if let Some(hash) = &m.hash {
                    args.insert(HASH_TAG.to_string(), hash.to_owned());
                }
            },
            Request::Scores(s) => {
                if let Some(id) = s.map_id {
                    args.insert(MAP_TAG.to_string(), id.to_string());
                }
                if let Some(id) = s.user_id {
                    args.insert(USER_TAG.to_string(), id.to_string());
                } else if let Some(name) = &s.username {
                    args.insert(USER_TAG.to_string(), name.replace(" ", "%"));
                }
                if let Some(mode) = s.mode {
                    args.insert(MODE_TAG.to_string(), (mode as u8).to_string());
                }
                if let Some(mods) = s.mods {
                    args.insert(MODS_TAG.to_owned(), mods.to_string());
                }
                if let Some(limit) = s.limit {
                    args.insert(LIMIT_TAG.to_owned(), limit.to_string());
                }
            },
            Request::Best(b) => {
                if let Some(id) = b.user_id {
                    args.insert(USER_TAG.to_string(), id.to_string());
                } else if let Some(name) = &b.username {
                    args.insert(USER_TAG.to_string(), name.replace(" ", "%"));
                }
                if let Some(mode) = b.mode {
                    args.insert(MODE_TAG.to_string(), (mode as u8).to_string());
                }
                if let Some(limit) = b.limit {
                    args.insert(LIMIT_TAG.to_owned(), limit.to_string());
                }
            },
            Request::Recent(r) => {
                if let Some(id) = r.user_id {
                    args.insert(USER_TAG.to_string(), id.to_string());
                } else if let Some(name) = &r.username {
                    args.insert(USER_TAG.to_string(), name.replace(" ", "%"));
                }
                if let Some(mode) = r.mode {
                    args.insert(MODE_TAG.to_string(), (mode as u8).to_string());
                }
                if let Some(limit) = r.limit {
                    args.insert(LIMIT_TAG.to_owned(), limit.to_string());
                }
            },
        }
        args
    }

    pub(crate) fn get_endpoint(&self) -> &'static str {
        match self {
            &Request::Users(_) => USER_ENDPOINT,
            &Request::Beatmaps(_) => BEATMAP_ENDPOINT,
            &Request::Scores(_) => SCORE_ENDPOINT,
            &Request::Best(_) => USER_BEST_ENDPOINT,
            &Request::Recent(_) => USER_RECENT_ENDPOINT,
        }
    }
}
