mod api;
pub(crate) mod deserialize;
mod error;
pub mod requests;

pub use api::Osu;
pub use error::OsuError;
pub use requests::{
    BeatmapRequest, BestRequest, MatchRequest, RecentRequest, ScoreRequest, UserRequest,
};

pub type OsuResult<T> = Result<T, OsuError>;
