use reqwest::{Error as ReqwestError, StatusCode};
use serde::Deserialize;
use serde_json::Error as JsonError;
use thiserror::Error as ThisError;

/// `Result<_, OsuError>`
pub type OsuResult<T> = Result<T, OsuError>;

#[derive(Debug, ThisError)]
/// Main error enum
pub enum OsuError {
    #[error("Could not parse i8 `{0}` into ApprovalStatus")]
    ApprovalStatusParsing(i8),
    #[error("Failed to build reqwest client")]
    BuildingClient(#[source] ReqwestError),
    #[error("Failed to chunk a response")]
    ChunkingResponse(#[source] ReqwestError),
    #[error("Failed to parse grade")]
    GradeParsing,
    #[error("Either the specified multiplayer match id was invalid or the match is private")]
    InvalidMultiplayerMatch,
    #[error("Failed to parse mods")]
    ModParsing(#[source] ModError),
    #[error("Failed to deserialize a response")]
    Parsing {
        body: String,
        #[source]
        source: JsonError,
    },
    #[error("Failed to send a request")]
    RequestError(#[source] ReqwestError),
    #[error("The response contained an error code={status}")]
    Response {
        body: String,
        #[source]
        error: ApiError,
        status: StatusCode,
    },
    #[error("The API may be temporarily unavailable (received 503)")]
    ServiceUnavailable(Option<String>),
}

#[derive(Debug, Deserialize, ThisError)]
#[error("{error}")]
pub struct ApiError {
    error: String,
}

#[derive(Debug, ThisError)]
pub enum ModError {
    #[error("Failed to parse `{0}`")]
    U32(u32),
    #[error("Failed to parse string")]
    Str,
}
