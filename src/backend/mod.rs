mod api;
mod error;
pub mod requests;

#[cfg(feature = "cache")]
pub use api::OsuCached;

pub use api::Osu;
pub use error::OsuError;
pub use requests::{
    BeatmapRequest, BestRequest, MatchRequest, RecentRequest, ScoreRequest, UserRequest,
};

pub type OsuResult<T> = Result<T, OsuError>;
