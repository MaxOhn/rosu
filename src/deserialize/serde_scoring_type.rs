use crate::models::ScoringType;
use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::fmt;

struct ScoringTypeVisitor;

impl<'de> Visitor<'de> for ScoringTypeVisitor {
    type Value = Option<ScoringType>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u8, null, or a stringified number")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let scoring_type = match v {
            "0" => Some(ScoringType::Score),
            "1" => Some(ScoringType::Accuracy),
            "2" => Some(ScoringType::Combo),
            "3" => Some(ScoringType::ScoreV2),
            _ => None,
        };
        Ok(scoring_type)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(ScoringType::from(v)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub fn to_scoring_type<'de, D>(d: D) -> Result<ScoringType, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(ScoringTypeVisitor)?
        .expect("Could not unwrap scoring type"))
}
