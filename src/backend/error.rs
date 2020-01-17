use hyper::http::uri::InvalidUri;
use std::{
    error::Error,
    fmt::{self, Debug},
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum OsuError {
    BadResponse(String),
    FromUtf8(FromUtf8Error),
    Hyper(::hyper::Error),
    Json(::serde_json::Error),
    Other(String),
    ReqBuilder(String),
    Uri(InvalidUri),
}

impl From<hyper::Error> for OsuError {
    fn from(err: hyper::Error) -> Self {
        OsuError::Hyper(err)
    }
}

impl From<serde_json::Error> for OsuError {
    fn from(err: serde_json::Error) -> Self {
        OsuError::Json(err)
    }
}

impl From<InvalidUri> for OsuError {
    fn from(err: InvalidUri) -> Self {
        OsuError::Uri(err)
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
            Self::Hyper(e) => write!(f, "{}", e),
            Self::Json(e) => write!(f, "{}", e),
            Self::Uri(e) => write!(f, "{}", e),
            Self::FromUtf8(e) => write!(f, "{}", e),
            Self::BadResponse(e) => write!(f, "{}", e),
            Self::Other(e) => write!(f, "{}", e),
        }
    }
}

impl Error for OsuError {}
