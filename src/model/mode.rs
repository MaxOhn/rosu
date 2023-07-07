use std::fmt::{Display, Formatter, Result as FmtResult};

#[cfg(feature = "serialize")]
use serde_repr::Serialize_repr;

/// Enum for the four game modes osu!standard, osu!taiko, Catch the beat, and osu!mania
#[derive(Debug, Clone, Hash, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize_repr))]
#[repr(u8)]
pub enum GameMode {
    Osu = 0,
    Taiko = 1,
    Catch = 2,
    Mania = 3,
}

impl Default for GameMode {
    #[inline]
    fn default() -> Self {
        Self::Osu
    }
}

impl Display for GameMode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mode = match self {
            GameMode::Osu => "osu",
            GameMode::Taiko => "taiko",
            GameMode::Catch => "fruits",
            GameMode::Mania => "mania",
        };

        f.write_str(mode)
    }
}

impl From<u8> for GameMode {
    #[inline]
    fn from(m: u8) -> Self {
        match m {
            1 => Self::Taiko,
            2 => Self::Catch,
            3 => Self::Mania,
            _ => Self::Osu,
        }
    }
}
