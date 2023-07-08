use crate::model::Grade;
use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer,
};
use std::{
    fmt::{Formatter, Result as FmtResult},
    str::FromStr,
};

struct GradeVisitor;

impl<'de> Visitor<'de> for GradeVisitor {
    type Value = Grade;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Grade::from_str(v).map_err(Error::custom)
    }
}

impl<'de> Deserialize<'de> for Grade {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(GradeVisitor)
    }
}
