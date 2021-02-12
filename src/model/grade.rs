use crate::OsuError;

use std::{fmt, str::FromStr};

#[cfg(feature = "serialize")]
use serde::Serialize;

/// Enum for a [`Score`](crate::model::Score)'s grade (sometimes called rank)
#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub enum Grade {
    F,
    D,
    C,
    B,
    A,
    S,
    SH,
    X,
    XH,
}

impl Grade {
    /// Check two grades for equality, ignoring silver-/regular-S difference
    ///
    /// # Example
    /// ```
    /// use rosu::model::Grade;
    ///
    /// assert!(Grade::S.eq_letter(Grade::SH));
    /// assert!(!Grade::X.eq_letter(Grade::SH));
    /// ```
    #[inline]
    pub fn eq_letter(self, other: Grade) -> bool {
        match self {
            Grade::XH | Grade::X => other == Grade::XH || other == Grade::X,
            Grade::SH | Grade::S => other == Grade::SH || other == Grade::S,
            _ => self == other,
        }
    }
}

impl FromStr for Grade {
    type Err = OsuError;

    fn from_str(grade: &str) -> Result<Self, Self::Err> {
        let grade = match grade.to_uppercase().as_str() {
            "XH" | "SSH" => Self::XH,
            "X" | "SS" => Self::X,
            "SH" => Self::SH,
            "S" => Self::S,
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "F" => Self::F,
            _ => return Err(OsuError::GradeParsing),
        };

        Ok(grade)
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grade_eq() {
        assert!(Grade::SH.eq_letter(Grade::S));
    }
}
