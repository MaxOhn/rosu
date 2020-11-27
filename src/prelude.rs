#[cfg(feature = "cache")]
pub use crate::OsuCached;

pub use crate::{
    error::{APIError, ModError},
    model::*,
    Osu, OsuError, OsuResult,
};

#[cfg(feature = "cache")]
pub use darkredis::ConnectionPool;

pub use reqwest::ClientBuilder;

#[cfg(feature = "metrics")]
pub use prometheus::IntCounterVec;
