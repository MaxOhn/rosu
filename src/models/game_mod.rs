use crate::backend::OsuError;
use num_traits::FromPrimitive as FP;
use std::convert::Into;

/// Enum for all game modifications
///
/// As it derives `FromPrimitive`, one can use `GameMod::from_u32` to convert from `u32` to `GameMod`
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum GameMod {
    NoMod = 0,
    NoFail = 0x1,
    Easy = 0x2,
    TouchDevice = 0x4,
    Hidden = 0x8,
    HardRock = 0x10,
    SuddenDeath = 0x20,
    DoubleTime = 0x40,
    Relax = 0x80,
    HalfTime = 0x100,
    NightCore = 0x240,
    Flashlight = 0x400,
    Autoplay = 0x800,
    SpunOut = 0x1000,
    Autopilot = 0x2000,
    Perfect = 0x4020,
    Key4 = 0x8000,
    Key5 = 0x10_000,
    Key6 = 0x20_000,
    Key7 = 0x40_000,
    Key8 = 0x80_000,
    FadeIn = 0x100_000,
    Random = 0x200_000,
    Cinema = 0x400_000,
    Target = 0x800_000,
    Key9 = 0x1_000_000,
    KeyCoop = 0x2_000_000,
    Key1 = 0x4_000_000,
    Key2 = 0x8_000_000,
    Key3 = 0x10_000_000,
    ScoreV2 = 0x20_000_000,
    Mirror = 0x40_000_000,
}

impl Into<u32> for GameMod {
    fn into(self) -> u32 {
        self as u32
    }
}

impl GameMod {
    /// Static function to convert `u32` to `Vec<GameMod>`.
    /// Returns an error if it fails.
    /// # Examples
    /// ```
    /// # use rosu::backend::OsuError;
    /// use rosu::models::GameMod;
    ///
    /// # fn main() -> Result<(), OsuError> {
    /// let mods: Vec<GameMod> = GameMod::try_from(128)?;
    /// assert_eq!(mods.len(), 1);
    /// assert_eq!(*mods.get(0).unwrap(), GameMod::Relax);
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_from(m: u32) -> Result<Vec<Self>, OsuError> {
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

    /// Function to convert `u32` to `Vec<GameMod>`.
    /// Panics if it fails.
    /// # Examples
    /// ```
    /// use rosu::models::GameMod;
    ///
    /// let mods: Vec<GameMod> = GameMod::from(128);
    /// assert_eq!(mods.len(), 1);
    /// assert_eq!(*mods.get(0).unwrap(), GameMod::Relax);
    /// // Will panic
    /// # #[should_panic]
    /// let mods: Vec<GameMod> = GameMod::from(3);
    /// ```
    pub fn from(m: u32) -> Vec<Self> {
        Self::try_from(m).unwrap_or_else(|_| panic!("Can not parse {} into Vec<GameMod>", m))
    }

    /// Function to convert `&[GameMod]` to `u32`.
    /// # Examples
    /// ```
    /// use rosu::models::GameMod;
    ///
    /// let mods = vec![GameMod::Hidden, GameMod::HardRock];
    /// let bits = GameMod::slice_to_u32(&mods);
    /// assert_eq!(bits, 8 + 16);
    /// ```
    pub fn slice_to_u32(mods: &[GameMod]) -> u32 {
        mods.iter().map(|m| *m as u32).sum()
    }

    /// Method that checks whether a game mod is one of osu!mania's key mods.
    /// # Examples
    /// ```
    /// use rosu::models::GameMod;
    ///
    /// assert!(GameMod::Key4.is_key_mod());
    /// assert!(!GameMod::Hidden.is_key_mod());
    /// ```
    pub fn is_key_mod(self) -> bool {
        use GameMod::{Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9};
        match self {
            Key1 | Key2 | Key3 | Key4 | Key5 | Key6 | Key7 | Key8 | Key9 => true,
            _ => false,
        }
    }

    /// Method that checks whether a game mood increases the play score
    /// # Examples
    /// ```
    /// use rosu::models::GameMod;
    ///
    /// assert!(GameMod::HardRock.increases_score());
    /// assert!(!GameMod::HalfTime.increases_score());
    /// ```
    pub fn increases_score(self) -> bool {
        use GameMod::{DoubleTime, FadeIn, Flashlight, HardRock, Hidden};
        match self {
            Hidden | HardRock | DoubleTime | Flashlight | FadeIn => true,
            _ => false,
        }
    }
}
