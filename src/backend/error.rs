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
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Error,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(d: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`error`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "error" => Ok(Field::Error),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                d.deserialize_identifier(FieldVisitor)
            }
        }
        struct APIErrorVisitor;

        impl<'de> Visitor<'de> for APIErrorVisitor {
            type Value = APIError;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct APIError")
            }

            fn visit_map<V>(self, mut map: V) -> Result<APIError, V::Error>
            where
                V: MapAccess<'de>,
            {
                match map.next_key()? {
                    Some(Field::Error) => Ok(APIError(map.next_value()?)),
                    _ => Err(de::Error::missing_field("error")),
                }
            }
        }
        const FIELDS: &'static [&'static str] = &["error"];
        deserializer.deserialize_struct("APIError", FIELDS, APIErrorVisitor)
    }
}
