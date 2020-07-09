use crate::OsuError;

use std::{convert::TryFrom, fmt};

#[cfg(feature = "serialize")]
use serde_derive::Serialize;

/// Enum for a [`Score`]'s grade (sometimes called rank)
///
/// [`Score`]: struct.Score.html
#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub enum Grade {
    XH,
    X,
    SH,
    S,
    A,
    B,
    C,
    D,
    F,
}

impl Grade {
    /// Check two grades for equality, ignoring silver-/regular-S difference
    /// # Example
    /// ```
    /// use rosu::models::Grade;
    ///
    /// assert!(Grade::S.eq_letter(Grade::SH));
    /// assert!(!Grade::X.eq_letter(Grade::SH));
    /// ```
    pub fn eq_letter(self, other: Grade) -> bool {
        match self {
            Grade::XH | Grade::X => other == Grade::XH || other == Grade::X,
            Grade::SH | Grade::S => other == Grade::SH || other == Grade::S,
            _ => self == other,
        }
    }
}

impl TryFrom<&str> for Grade {
    type Error = OsuError;

    fn try_from(grade: &str) -> Result<Self, Self::Error> {
        let grade = match grade.to_uppercase().as_ref() {
            "XH" | "SSH" => Self::XH,
            "X" | "SS" => Self::X,
            "SH" => Self::SH,
            "S" => Self::S,
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "F" => Self::F,
            _ => {
                return Err(OsuError::ParseError(format!(
                    "Cannot parse \"{}\" into a Grade",
                    grade
                )))
            }
        };
        Ok(grade)
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
