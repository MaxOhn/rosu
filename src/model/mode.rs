use std::fmt;

#[cfg(feature = "serialize")]
use serde_repr::Serialize_repr;

/// Enum for the four game modes osu!standard, osu!taiko, Catch the beat, and osu!mania
#[derive(Debug, Clone, Hash, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize_repr))]
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u8> for GameMode {
    fn from(m: u8) -> Self {
        match m {
            1 => Self::TKO,
            2 => Self::CTB,
            3 => Self::MNA,
            _ => Self::STD,
        }
    }
}
