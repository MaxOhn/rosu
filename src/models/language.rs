use crate::backend::OsuError;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Language {
    Any = 0,
    Other = 1,
    English = 2,
    Japanese = 3,
    Chinese = 4,
    Instrumental = 5,
    Korean = 6,
    French = 7,
    German = 8,
    Swedish = 9,
    Spanish = 10,
    Italian = 11,
}

impl Default for Language {
    fn default() -> Self {
        Self::Any
    }
}

impl TryFrom<u8> for Language {
    type Error = OsuError;
    fn try_from(l: u8) -> Result<Self, Self::Error> {
        match l {
            0 => Ok(Self::Any),
            1 => Ok(Self::Other),
            2 => Ok(Self::English),
            3 => Ok(Self::Japanese),
            4 => Ok(Self::Chinese),
            5 => Ok(Self::Instrumental),
            6 => Ok(Self::Korean),
            7 => Ok(Self::French),
            8 => Ok(Self::German),
            9 => Ok(Self::Swedish),
            10 => Ok(Self::Spanish),
            11 => Ok(Self::Italian),
            _ => Err(OsuError::Other(format!(
                "Can not parse {} into Language",
                l
            ))),
        }
    }
}
