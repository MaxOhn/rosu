use crate::models::Genre;
use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::fmt;

struct GenreVisitor;

impl<'de> Visitor<'de> for GenreVisitor {
    type Value = Option<Genre>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an u8, a string, or null")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let genre = match v {
            "0" | "any" => Some(Genre::Any),
            "1" | "unspecified" => Some(Genre::Unspecified),
            "2" | "videogame" => Some(Genre::VideoGame),
            "3" | "anime" => Some(Genre::Anime),
            "4" | "rock" => Some(Genre::Rock),
            "5" | "Pop" => Some(Genre::Pop),
            "6" | "other" => Some(Genre::Other),
            "7" | "novelty" => Some(Genre::Novelty),
            "9" | "hiphip" => Some(Genre::HipHop),
            "10" | "electronic" => Some(Genre::Electronic),
            "11" | "metal" => Some(Genre::Metal),
            "12" | "classical" => Some(Genre::Classical),
            "13" | "folk" => Some(Genre::Folk),
            "14" | "jazz" => Some(Genre::Jazz),
            _ => None,
        };
        Ok(genre)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(Genre::from(v)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub(crate) fn to_maybe_genre<'de, D>(d: D) -> Result<Option<Genre>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(GenreVisitor)
}

pub(crate) fn to_genre<'de, D>(d: D) -> Result<Genre, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(GenreVisitor)?
        .expect("Could not unwrap genre"))
}
