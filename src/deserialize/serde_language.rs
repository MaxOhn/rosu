use crate::models::Language;
use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::fmt;

struct LanguageVisitor;

impl<'de> Visitor<'de> for LanguageVisitor {
    type Value = Option<Language>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an i8, a string, or null")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let language = match v {
            "0" | "any" => Some(Language::Any),
            "1" | "other" => Some(Language::Other),
            "2" | "english" => Some(Language::English),
            "3" | "japanese" => Some(Language::Japanese),
            "4" | "chinese" => Some(Language::Chinese),
            "5" | "instrumental" => Some(Language::Instrumental),
            "6" | "korean" => Some(Language::Korean),
            "7" | "french" => Some(Language::French),
            "8" | "german" => Some(Language::German),
            "9" | "swedish" => Some(Language::Swedish),
            "10" | "spanish" => Some(Language::Spanish),
            "11" | "italian" => Some(Language::Italian),
            "12" | "russian" => Some(Language::Russian),
            "13" | "polish" => Some(Language::Polish),
            "14" | "unspecified" => Some(Language::Unspecified),
            _ => None,
        };
        Ok(language)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(Language::from(v)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub fn to_language<'de, D>(d: D) -> Result<Language, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(LanguageVisitor)?
        .expect("Could not unwrap language"))
}
