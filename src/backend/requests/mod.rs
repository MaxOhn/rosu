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

const API_BASE: &str = "https://osu.ppy.sh";

const USER_TAG: &str = "u";
const MODE_TAG: &str = "m";
const MAP_TAG: &str = "b";
const TYPE_TAG: &str = "type";
const LIMIT_TAG: &str = "limit";
const MODS_TAG: &str = "mods";
