use crate::models::Grade;
use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::{convert::TryFrom, fmt};

struct GradeVisitor;

impl<'de> Visitor<'de> for GradeVisitor {
    type Value = Option<Grade>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or null")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Grade::try_from(v).map(Some).map_err(de::Error::custom)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub fn to_grade<'de, D>(d: D) -> Result<Grade, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(GradeVisitor)?
        .expect("Could not unwrap grade"))
}
