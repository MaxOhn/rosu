use crate::backend::OsuError;
use std::convert::TryFrom;

/// Basic enum to describe a beatmap's approval status
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(i8)]
pub enum ApprovalStatus {
    Loved = 4,
    Qualified = 3,
    Approved = 2,
    Ranked = 1,
    Pending = 0,
    WIP = -1,
    Graveyard = -2,
}

impl TryFrom<i8> for ApprovalStatus {
    type Error = OsuError;
    fn try_from(m: i8) -> Result<Self, Self::Error> {
        match m {
            4 => Ok(Self::Loved),
            3 => Ok(Self::Qualified),
            2 => Ok(Self::Approved),
            1 => Ok(Self::Ranked),
            0 => Ok(Self::Pending),
            -1 => Ok(Self::WIP),
            -2 => Ok(Self::Graveyard),
            _ => Err(OsuError::Other(format!(
                "Can not parse {} into ApprovalStatus",
                m
            ))),
        }
    }
}
