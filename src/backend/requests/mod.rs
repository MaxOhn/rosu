mod maps;
mod r#match;
mod osu_args;
mod osu_request;
mod scores;
mod user_best;
mod user_recent;
mod users;

pub use maps::BeatmapArgs;
pub use osu_args::OsuArgs;
pub use osu_request::OsuRequest;
pub use r#match::MatchArgs;
pub use scores::ScoreArgs;
pub use user_best::UserBestArgs;
pub use user_recent::UserRecentArgs;
pub use users::UserArgs;

pub(crate) const API_BASE: &str = "https://osu.ppy.sh/api/";
