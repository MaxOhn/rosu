use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum OsuError {
    API(String),
    FetchError(reqwest::Error),
    ParseUrl(String),
    Serde(serde_json::Error, String),
    Other(String),
}

impl From<reqwest::Error> for OsuError {
    fn from(e: reqwest::Error) -> Self {
        Self::FetchError(e)
    }
}

impl From<APIError> for OsuError {
    fn from(e: APIError) -> Self {
        Self::API(e.0)
    }
}

impl fmt::Display for OsuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::API(e) => write!(f, "api error: {}", e),
            Self::FetchError(e) => write!(f, "error while fetching: {}", e),
            Self::Other(e) => f.write_str(e),
            Self::ParseUrl(e) => write!(f, "could not parse request into url: {}", e),
            Self::Serde(e, text) => write!(
                f,
                "error while deserializing api response: {}, response: {}",
                e, text
            ),
        }
    }
}

impl Error for OsuError {}

#[derive(Debug)]
pub(crate) struct APIError(pub(crate) String);

impl<'de> Deserialize<'de> for APIError {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct APIErrorVisitor;

        impl<'de> Visitor<'de> for APIErrorVisitor {
            type Value = APIError;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("error field")
            }

            fn visit_map<V>(self, mut map: V) -> Result<APIError, V::Error>
            where
                V: MapAccess<'de>,
            {
                match map.next_key()? {
                    Some("error") => Ok(APIError(map.next_value()?)),
                    _ => Err(de::Error::missing_field("error")),
                }
            }
        }
        d.deserialize_struct("APIError", &["error"], APIErrorVisitor)
    }
}
