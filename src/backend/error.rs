use serde_json::Error as SerdeError;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum OsuError {
    ParseUrl,
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
            Self::FetchError(e) => write!(f, "error while fetching: {}", e),
            Self::Other(e) => f.write_str(e),
            Self::ParseUrl => f.write_str("could not parse request into url"),
            Self::Serde(e, text) => write!(
                f,
                "error while deserializing api response: {}, response: {}",
                e, text
            ),
        }
    }
}

impl Error for OsuError {}
