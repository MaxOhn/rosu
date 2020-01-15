use crate::backend::OsuError;
use num_traits::FromPrimitive as FP;

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum GameMod {
    NM = 0,
    NF = 1,
    EZ = 1 << 1,
    TD = 1 << 2,
    HD = 1 << 3,
    HR = 1 << 4,
    SD = 1 << 5,
    DT = 1 << 6,
    RX = 1 << 7,
    HT = 1 << 8,
    NC = 1 << 9,
    FL = 1 << 10,
    AL = 1 << 11,
    SO = 1 << 12,
    AP = 1 << 13,
    PF = 1 << 14,
    K4 = 1 << 15,
    K5 = 1 << 16,
    K6 = 1 << 17,
    K7 = 1 << 18,
    K8 = 1 << 19,
    FI = 1 << 20,
    RN = 1 << 21,
    CM = 1 << 22,
    TG = 1 << 23,
    K9 = 1 << 24,
    KC = 1 << 25,
    K1 = 1 << 26,
    K2 = 1 << 27,
    K3 = 1 << 28,
    V2 = 1 << 29,
    LM = 1 << 30,
}

impl GameMod {
    pub fn mods_from_u32(m: u32) -> Result<Vec<Self>, OsuError> {
        let mut mods = Vec::new();
        if m == 0 {
            return Ok(mods);
        }
        let mut s = 0;
        while s < 32 {
            let curr = (1 << s) & m;
            if curr > 0 {
                if let Some(game_mod) = GameMod::from_u32(curr) {
                    mods.push(game_mod);
                }
            }
            s += 1;
        }
        if m > 0 && mods.is_empty() {
            Err(OsuError::Other(format!(
                "Can not parse {} into Vec<GameMod>",
                m
            )))
        } else {
            Ok(mods)
        }
    }

    pub fn from(m: u32) -> Vec<Self> {
        Self::mods_from_u32(m).unwrap_or_else(|_| panic!("Can not parse {} into Vec<GameMod>", m))
    }

    pub fn slice_to_u32(mods: &[GameMod]) -> u32 {
        mods.iter().map(|m| *m as u32).sum()
    }
}
