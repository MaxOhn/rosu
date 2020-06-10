use std::{
    error::Error,
    fmt::{self, Debug},
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum OsuError {
    BadResponse(String),
    InvalidUrl(String),
    FetchError(reqwest::Error),
    FromUtf8(FromUtf8Error),
    Json(::serde_json::Error),
    ParseError(String),
    Other(&'static str),
    ReqBuilder(String),
    NoResults(String),
}

impl From<serde_json::Error> for OsuError {
    fn from(err: serde_json::Error) -> Self {
        OsuError::Json(err)
    }
}

impl From<FromUtf8Error> for OsuError {
    fn from(err: FromUtf8Error) -> Self {
        OsuError::FromUtf8(err)
    }
}

impl fmt::Display for OsuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReqBuilder(e) => write!(f, "{}", e),
            Self::Json(e) => write!(f, "{}", e),
            Self::FromUtf8(e) => write!(f, "{}", e),
            Self::ParseError(e) => write!(f, "{}", e),
            Self::BadResponse(e) => write!(f, "{}", e),
            Self::Other(e) => write!(f, "{}", e),
            Self::FetchError(e) => write!(f, "Error while fetching: {}", e),
            Self::InvalidUrl(url) => write!(f, "Could not parse \"{}\" into url", url),
            Self::NoResults(e) => write!(f, "API response contained no {} elements", e),
        }
    }
}

impl Error for OsuError {}
