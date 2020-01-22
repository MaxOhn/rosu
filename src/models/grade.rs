use std::{convert::From, string::ToString};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl From<String> for Grade {
    fn from(grade: String) -> Self {
        match grade.to_uppercase().as_ref() {
            "XH" | "SSH" => Self::XH,
            "X" | "SS" => Self::X,
            "SH" => Self::SH,
            "S" => Self::S,
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "F" => Self::F,
            _ => panic!("Cannot parse {} into a Grade", grade),
        }
    }
}

impl ToString for Grade {
    fn to_string(&self) -> String {
        match self {
            Self::XH => "XH",
            Self::X => "X",
            Self::SH => "SH",
            Self::S => "S",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::F => "F",
        }
        .to_owned()
    }
}
