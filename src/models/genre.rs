use crate::backend::OsuError;
use std::convert::TryFrom;

/// Basic enum to describe a beatmap's music genre
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Genre {
    Any = 0,
    Unspecified = 1,
    VideoGame = 2,
    Anime = 3,
    Rock = 4,
    Pop = 5,
    Other = 6,
    Novelty = 7,
    HipHop = 9,
    Electronic = 10,
}

impl Default for Genre {
    fn default() -> Self {
        Self::Any
    }
}

impl TryFrom<u8> for Genre {
    type Error = OsuError;
    fn try_from(g: u8) -> Result<Self, Self::Error> {
        match g {
            0 => Ok(Self::Any),
            1 => Ok(Self::Unspecified),
            2 => Ok(Self::VideoGame),
            3 => Ok(Self::Anime),
            4 => Ok(Self::Rock),
            5 => Ok(Self::Pop),
            6 => Ok(Self::Other),
            7 => Ok(Self::Novelty),
            9 => Ok(Self::HipHop),
            10 => Ok(Self::Electronic),
            _ => Err(OsuError::Other(format!("Can not parse {} into Genre", g))),
        }
    }
}
