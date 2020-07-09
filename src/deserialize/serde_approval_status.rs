use crate::models::ApprovalStatus;
use serde::{
    de::{self, Visitor},
    Deserializer,
};
use std::fmt;

struct ApprovalStatusVisitor;

impl<'de> Visitor<'de> for ApprovalStatusVisitor {
    type Value = Option<ApprovalStatus>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an i8, a string, or null")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let status = match v {
            "4" | "loved" => Some(ApprovalStatus::Loved),
            "3" | "qualified" => Some(ApprovalStatus::Qualified),
            "2" | "approved" => Some(ApprovalStatus::Approved),
            "1" | "ranked" => Some(ApprovalStatus::Ranked),
            "0" | "pending" => Some(ApprovalStatus::Pending),
            "-1" | "wip" => Some(ApprovalStatus::WIP),
            "-2" | "graveyard" => Some(ApprovalStatus::Graveyard),
            _ => None,
        };
        Ok(status)
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(ApprovalStatus::from(v)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub(crate) fn to_maybe_approval_status<'de, D>(d: D) -> Result<Option<ApprovalStatus>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(ApprovalStatusVisitor)
}

pub(crate) fn to_approval_status<'de, D>(d: D) -> Result<ApprovalStatus, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(d.deserialize_any(ApprovalStatusVisitor)?
        .expect("Could not unwrap approval status"))
}
