use serde_json::Error as SerdeError;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum OsuError {
    InvalidUrl(String),
    FetchError(reqwest::Error),
    Serde(SerdeError, String),
    Other(String),
}

impl From<reqwest::Error> for OsuError {
    fn from(err: reqwest::Error) -> Self {
        OsuError::FetchError(err)
    }
}

impl fmt::Display for OsuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(e) => f.write_str(e),
            Self::FetchError(e) => write!(f, "error while fetching: {}", e),
            Self::InvalidUrl(url) => write!(f, "could not parse `{}` into url", url),
            Self::Serde(e, text) => write!(
                f,
                "error while deserializing api response: {}, response: {}",
                e, text
            ),
        }
    }
}

impl Error for OsuError {}
