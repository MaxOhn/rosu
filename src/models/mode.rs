use crate::backend::OsuError;
use std::{convert::TryFrom, fmt};

/// Enum for the four game modes osu!standard, osu!taiko, Catch the beat, and osu!mania
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum GameMode {
    STD = 0,
    TKO = 1,
    CTB = 2,
    MNA = 3,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::STD
    }
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<u8> for GameMode {
    type Error = OsuError;
    fn try_from(m: u8) -> Result<Self, Self::Error> {
        match m {
            0 => Ok(Self::STD),
            1 => Ok(Self::TKO),
            2 => Ok(Self::CTB),
            3 => Ok(Self::MNA),
            _ => Err(OsuError::Other(format!(
                "Can not parse {} into GameMode",
                m
            ))),
        }
    }
}
