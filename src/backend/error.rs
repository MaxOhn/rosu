use std::{error::Error, fmt};

#[derive(Debug)]
pub enum OsuError {
    InvalidUrl(String),
    FetchError(reqwest::Error),
    Json(serde_json::Error),
    ParseError(String),
    Other(&'static str),
}

impl From<serde_json::Error> for OsuError {
    fn from(err: serde_json::Error) -> Self {
        OsuError::Json(err)
    }
}

impl From<reqwest::Error> for OsuError {
    fn from(err: reqwest::Error) -> Self {
        OsuError::FetchError(err)
    }
}

impl fmt::Display for OsuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(e) => write!(f, "Received unexpected JSON from osu!api: {}", e),
            Self::ParseError(e) => write!(f, "{}", e),
            Self::Other(e) => write!(f, "{}", e),
            Self::FetchError(e) => write!(f, "Error while fetching: {}", e),
            Self::InvalidUrl(url) => write!(f, "Could not parse \"{}\" into url", url),
        }
    }
}

impl Error for OsuError {}
