use reqwest::StatusCode;
use serde::Deserialize;
use std::{error::Error, fmt};

/// `Result<_, OsuError>`
pub type OsuResult<T> = Result<T, OsuError>;

#[derive(Debug)]
/// Main error enum
pub enum OsuError {
    /// Failed to parse an i8 to an [`ApprovalStatus`](crate::model::ApprovalStatus).
    ApprovalStatusParsing(i8),
    /// Reqwest failed to build its client.
    BuildingClient(reqwest::Error),
    /// Error while handling response from the api
    ChunkingResponse(reqwest::Error),
    /// Failed to parse a `&str` to a [`Grade`].
    ///
    /// [`Grade`]: enum.Grade.html
    GradeParsing,
    /// The api response indicates that either the given `match_id`
    /// was invalid or that the corresponding [`Match`] was private.
    ///
    /// [`Match`]: struct.Match.html
    InvalidMultiplayerMatch,
    ModParsing(ModError),
    Parsing {
        body: String,
        source: serde_json::Error,
    },
    RequestError(reqwest::Error),
    Response {
        body: String,
        error: APIError,
        status: StatusCode,
    },
    ServiceUnavailable(Option<String>),
}

impl fmt::Display for OsuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApprovalStatusParsing(n) => {
                write!(f, "could not parse i8 `{}` into ApprovalStatus", n)
            }
            Self::BuildingClient(_) => f.write_str("error while building reqwest client"),
            Self::ChunkingResponse(_) => f.write_str("failed to chunk the response"),
            Self::GradeParsing => f.write_str("error while parsing Grade"),
            Self::InvalidMultiplayerMatch => f.write_str(
                "either the specified multiplayer match id was invalid or the match was private",
            ),
            Self::ModParsing(_) => f.write_str("error while parsing GameMods"),
            Self::Parsing { body, .. } => write!(f, "could not deserialize response: {}", body),
            Self::RequestError(_) => f.write_str("error while requesting data"),
            Self::Response { status, .. } => write!(f, "response error, status {}", status),
            Self::ServiceUnavailable(body) => write!(
                f,
                "api may be temporarily unavailable (received 503): {}",
                body.as_deref().unwrap_or("error while parsing body")
            ),
        }
    }
}

impl Error for OsuError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ApprovalStatusParsing(_) => None,
            Self::BuildingClient(e) => Some(e),
            Self::ChunkingResponse(e) => Some(e),
            Self::GradeParsing => None,
            Self::InvalidMultiplayerMatch => None,
            Self::ModParsing(e) => Some(e),
            Self::Parsing { source: e, .. } => Some(e),
            Self::RequestError(e) => Some(e),
            Self::Response { error: e, .. } => Some(e),
            Self::ServiceUnavailable(_) => None,
        }
    }
}

#[derive(Debug, Deserialize)]
/// The api response was of the form `{ error: "..." }`
pub struct APIError {
    error: String,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.error)
    }
}

impl Error for APIError {}

#[derive(Debug)]
/// Failed to parse [`GameMods`] either from `u32` or `&str`.
///
/// [`GameMods`]: struct.GameMods.html
pub enum ModError {
    U32(u32),
    Str,
}

impl fmt::Display for ModError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::U32(n) => write!(f, "can not parse u32 `{}` into GameMods", n),
            Self::Str => f.write_str("error while parsing string into GameMods"),
        }
    }
}

impl Error for ModError {}
