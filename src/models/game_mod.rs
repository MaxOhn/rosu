use crate::{backend::OsuError, models::GameMode};
use std::{
    collections::{
        btree_set::{IntoIter as IntoIterBTS, Iter as IterBTS},
        BTreeSet,
    },
    convert::{AsMut, AsRef, Into, TryFrom},
    fmt,
    iter::{DoubleEndedIterator, FromIterator, FusedIterator},
    ops::{Deref, DerefMut},
};

/// Enum for all game modifications
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum GameMod {
    NoMod = 0,
    NoFail = 1,
    Easy = 2,
    TouchDevice = 4,
    Hidden = 8,
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

impl GameMod {
    /// Method that checks whether a [`GameMod`] is one of osu!mania's key mods.
    ///
    /// [`GameMod`]: enum.GameMod.html
    ///
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

    /// Calculate the multiplier of the mod which will
    /// influence a [`Score`]'s playscore
    ///
    /// [`Score`]: struct.Score.html
    pub fn score_multiplier(self, mode: GameMode) -> f32 {
        match mode {
            GameMode::STD => match self {
                GameMod::HalfTime => 0.3,
                GameMod::Easy | GameMod::NoFail => 0.5,
                GameMod::SpunOut => 0.9,
                GameMod::HardRock | GameMod::Hidden => 1.06,
                GameMod::DoubleTime | GameMod::NightCore | GameMod::Flashlight => 1.12,
                _ => 1.0,
            },
            GameMode::TKO => match self {
                GameMod::HalfTime => 0.3,
                GameMod::Easy | GameMod::NoFail => 0.5,
                GameMod::HardRock | GameMod::Hidden => 1.06,
                GameMod::DoubleTime | GameMod::NightCore | GameMod::Flashlight => 1.12,
                _ => 1.0,
            },
            GameMode::CTB => match self {
                GameMod::HalfTime => 0.3,
                GameMod::Easy | GameMod::NoFail => 0.5,
                GameMod::DoubleTime | GameMod::NightCore | GameMod::Hidden => 1.06,
                GameMod::HardRock | GameMod::Flashlight => 1.12,
                _ => 1.0,
            },
            GameMode::MNA => match self {
                GameMod::Easy | GameMod::NoFail | GameMod::HalfTime => 0.5,
                _ => 1.0,
            },
        }
    }

    /// Check if a [`Score`]'s playscore will be increased
    ///
    /// [`Score`]: struct.Score.html
    pub fn increases_score(self, mode: GameMode) -> bool {
        self.score_multiplier(mode) > 1.0
    }

    /// Check if a [`Score`]'s playscore will be decreased
    ///
    /// [`Score`]: struct.Score.html
    pub fn decreases_score(self, mode: GameMode) -> bool {
        self.score_multiplier(mode) < 1.0
    }

    /// Check if a [`Beatmap`]'s star rating will be influenced
    ///
    /// [`Beatmap`]: struct.Beatmap.html
    pub fn changes_stars(self, mode: GameMode) -> bool {
        match self {
            GameMod::DoubleTime | GameMod::NightCore | GameMod::HalfTime => true,
            GameMod::HardRock | GameMod::Easy => mode == GameMode::STD || mode == GameMode::CTB,
            _ => false,
        }
    }

    /// Convert to a `u32`. Identical to `as u32`.
    pub fn as_bit(self) -> u32 {
        self as u32
    }
}

impl fmt::Display for GameMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GameMod::*;
        let abbrev = match self {
            NoMod => "NM",
            NoFail => "NF",
            Easy => "EZ",
            TouchDevice => "TD",
            Hidden => "HD",
            HardRock => "HR",
            SuddenDeath => "SD",
            DoubleTime => "DT",
            Relax => "RX",
            HalfTime => "HT",
            NightCore => "NC",
            Flashlight => "FL",
            Autoplay => "", // no abbrev
            SpunOut => "SO",
            Autopilot => "AP",
            Perfect => "PF",
            FadeIn => "FI",
            Random => "RD",
            Cinema => "", // no abbrev
            Target => "TP",
            KeyCoop => "", // no abbrev
            ScoreV2 => "V2",
            Mirror => "MR",
            Key1 => "1K",
            Key2 => "2K",
            Key3 => "3K",
            Key4 => "4K",
            Key5 => "5K",
            Key6 => "6K",
            Key7 => "7K",
            Key8 => "8K",
            Key9 => "9K",
        };
        write!(f, "{}", abbrev)
    }
}

impl Into<u32> for GameMod {
    fn into(self) -> u32 {
        self as u32
    }
}

impl TryFrom<u32> for GameMod {
    type Error = OsuError;

    fn try_from(m: u32) -> Result<Self, Self::Error> {
        use GameMod::*;
        let m = match m {
            0 => NoMod,
            1 => NoFail,
            2 => Easy,
            4 => TouchDevice,
            8 => Hidden,
            0x10 => HardRock,
            0x20 => SuddenDeath,
            0x40 => DoubleTime,
            0x80 => Relax,
            0x100 => HalfTime,
            0x240 => NightCore,
            0x400 => Flashlight,
            0x800 => Autoplay,
            0x1000 => SpunOut,
            0x2000 => Autopilot,
            0x4020 => Perfect,
            0x8000 => Key4,
            0x10_000 => Key5,
            0x20_000 => Key6,
            0x40_000 => Key7,
            0x80_000 => Key8,
            0x100_000 => FadeIn,
            0x200_000 => Random,
            0x400_000 => Cinema,
            0x800_000 => Target,
            0x1_000_000 => Key9,
            0x2_000_000 => KeyCoop,
            0x4_000_000 => Key1,
            0x8_000_000 => Key2,
            0x10_000_000 => Key3,
            0x20_000_000 => ScoreV2,
            0x40_000_000 => Mirror,
            _ => {
                return Err(OsuError::ParseError(format!(
                    "Can not parse {} into GameMod",
                    m
                )));
            }
        };
        Ok(m)
    }
}

impl TryFrom<&str> for GameMod {
    type Error = OsuError;

    fn try_from(m: &str) -> Result<Self, Self::Error> {
        use GameMod::*;
        let m = match m {
            "NM" => NoMod,
            "NF" => NoFail,
            "EZ" => Easy,
            "TD" => TouchDevice,
            "HD" => Hidden,
            "HR" => HardRock,
            "SD" => SuddenDeath,
            "DT" => DoubleTime,
            "RX" | "RL" => Relax,
            "HT" => HalfTime,
            "NC" => NightCore,
            "FL" => Flashlight,
            "SO" => SpunOut,
            "AP" => Autopilot,
            "PF" => Perfect,
            "FI" => FadeIn,
            "RD" => Random,
            "TP" => Target,
            "V2" => ScoreV2,
            "MR" => Mirror,
            "1K" => Key1,
            "2K" => Key2,
            "3K" => Key3,
            "4K" => Key4,
            "5K" => Key5,
            "6K" => Key6,
            "7K" => Key7,
            "8K" => Key8,
            "9K" => Key9,
            _ => {
                return Err(OsuError::ParseError(format!(
                    "Could not parse &str \"{}\" into GameMod",
                    m
                )))
            }
        };
        Ok(m)
    }
}

/// Collection struct containing multiple [`GameMod`]s
///
/// [`GameMod`]: enum.GameMod.html
#[derive(Default, Debug, Clone, Eq, Hash, PartialEq)]
pub struct GameMods(BTreeSet<GameMod>);

impl GameMods {
    pub fn new(mods: Vec<GameMod>) -> Self {
        Self(BTreeSet::from_iter(mods.into_iter()))
    }

    /// Check if a [`Beatmap`]'s star rating for the given [`GameMode`] will be influenced.
    ///
    /// [`Beatmap`]: struct.Beatmap.html
    /// [`GameMode`]: struct.GameMode.html
    ///
    /// # Example
    /// ```rust
    /// use rosu::models::{GameMode, GameMod, GameMods};
    /// use std::convert::TryFrom;
    ///
    /// let hdhr = GameMods::try_from(24).unwrap();
    /// assert!(hdhr.changes_stars(GameMode::STD));
    /// assert!(!hdhr.changes_stars(GameMode::MNA));
    /// let nc = GameMods::new(vec![GameMod::NightCore]);
    /// assert!(nc.changes_stars(GameMode::MNA));
    /// ```
    pub fn changes_stars(&self, mode: GameMode) -> bool {
        self.0.iter().any(|m| m.changes_stars(mode))
    }

    /// Calculate the multiplier of the mods which will
    /// influence a [`Score`]'s playscore
    ///
    /// [`Score`]: struct.Score.html
    ///
    /// # Example
    /// ```rust
    /// use rosu::models::{GameMod, GameMods, GameMode};
    /// use std::convert::TryFrom;
    ///
    /// let ezhd = GameMods::try_from(2 + 8).unwrap();
    /// assert_eq!(ezhd.score_multiplier(GameMode::STD), 0.53);
    /// assert_eq!(ezhd.score_multiplier(GameMode::MNA), 0.5);
    /// ```
    pub fn score_multiplier(&self, mode: GameMode) -> f32 {
        self.0.iter().map(|m| m.score_multiplier(mode)).product()
    }

    /// Check if a [`Score`]'s playscore will be increased
    ///
    /// [`Score`]: struct.Score.html
    ///
    /// # Example
    /// ```rust
    /// use rosu::models::{GameMod, GameMods, GameMode};
    /// use std::convert::TryFrom;
    ///
    /// let hrso = GameMods::try_from(16 + 2 << 16).unwrap();
    /// assert!(!hrso.increases_score(GameMode::STD));
    /// assert!(GameMods::from(GameMod::DoubleTime).increases_score(GameMode::TKO));
    /// ```
    pub fn increases_score(self, mode: GameMode) -> bool {
        self.score_multiplier(mode) > 1.0
    }

    /// Check if a [`Score`]'s playscore will be decreased
    ///
    /// [`Score`]: struct.Score.html
    ///
    /// # Example
    /// ```rust
    /// use rosu::models::{GameMod, GameMods, GameMode};
    /// use std::convert::TryFrom;
    ///
    /// let hrso = GameMods::try_from(16 + 0x1000).unwrap();
    /// assert!(hrso.decreases_score(GameMode::STD));
    /// assert!(!GameMods::from(GameMod::DoubleTime).decreases_score(GameMode::TKO));
    /// ```
    pub fn decreases_score(self, mode: GameMode) -> bool {
        self.score_multiplier(mode) < 1.0
    }

    /// Accumulate the bits of all contained [`GameMod`]s into a `u32`.
    ///
    /// [`GameMod`]: enum.GameMod.html
    ///
    /// # Example
    /// ```
    /// use rosu::models::{GameMod, GameMods};
    ///
    /// let mods = GameMods::new(vec![GameMod::Hidden, GameMod::HardRock]);
    /// let bits = mods.as_bits();
    /// assert_eq!(bits, 8 + 16);
    /// ```
    pub fn as_bits(&self) -> u32 {
        self.0.iter().map(|m| *m as u32).sum()
    }

    /// Returns an iterator over [`GameMod`] references
    ///
    /// [`GameMod`]: enum.GameMod.html
    ///
    /// # Example
    /// ```
    /// use rosu::models::{GameMod, GameMods};
    /// use std::convert::TryFrom;
    ///
    /// let mods = GameMods::try_from(8 + 16 + 64 + 128).unwrap();
    /// let mut mod_iter = mods.iter();
    /// assert_eq!(mod_iter.next(), Some(&GameMod::Hidden));
    /// assert_eq!(mod_iter.next(), Some(&GameMod::HardRock));
    /// assert_eq!(mod_iter.next(), Some(&GameMod::DoubleTime));
    /// assert_eq!(mod_iter.next(), Some(&GameMod::Relax));
    /// assert_eq!(mod_iter.next(), None);
    /// ```
    pub fn iter(&'_ self) -> Iter<'_> {
        Iter {
            iter: self.0.iter(),
        }
    }

    /// Check if a given [`GameMod`] is contained in the collection
    ///
    /// [`GameMod`]: enum.GameMod.html
    ///
    /// # Example
    /// ```
    /// use rosu::models::{GameMod, GameMods};
    /// use std::convert::TryFrom;
    ///
    /// let hdhrdtrx = GameMods::try_from(8 + 16 + 64 + 128).unwrap();
    /// assert!(hdhrdtrx.contains(&GameMod::Hidden));
    /// assert!(!hdhrdtrx.contains(&GameMod::NightCore));
    /// ```
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn contains(&self, m: &GameMod) -> bool {
        self.0.contains(m)
    }
}

impl fmt::Display for GameMods {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.len() {
            0 => write!(f, "NM"),
            _ => {
                let mut result = String::with_capacity(self.0.len() * 2);
                for m in self.0.iter() {
                    result.push_str(&m.to_string());
                }
                write!(f, "{}", result)
            }
        }
    }
}

impl FromIterator<GameMod> for GameMods {
    fn from_iter<I: IntoIterator<Item = GameMod>>(iter: I) -> Self {
        Self(BTreeSet::from_iter(iter.into_iter()))
    }
}

impl From<GameMod> for GameMods {
    fn from(m: GameMod) -> Self {
        let mut map = BTreeSet::new();
        map.insert(m);
        Self(map)
    }
}

impl From<Vec<GameMod>> for GameMods {
    fn from(mods: Vec<GameMod>) -> Self {
        Self(BTreeSet::from_iter(mods.into_iter()))
    }
}

impl From<&[GameMod]> for GameMods {
    fn from(mods: &[GameMod]) -> Self {
        Self(BTreeSet::from_iter(mods.iter().copied()))
    }
}

impl AsRef<BTreeSet<GameMod>> for GameMods {
    fn as_ref(&self) -> &BTreeSet<GameMod> {
        &self.0
    }
}

impl AsMut<BTreeSet<GameMod>> for GameMods {
    fn as_mut(&mut self) -> &mut BTreeSet<GameMod> {
        &mut self.0
    }
}

impl Deref for GameMods {
    type Target = BTreeSet<GameMod>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GameMods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<u32> for GameMods {
    fn into(self) -> u32 {
        self.as_bits()
    }
}

impl TryFrom<&str> for GameMods {
    type Error = OsuError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut mods = Vec::new();
        for m in util::cut(s, 2) {
            mods.push(GameMod::try_from(m.to_uppercase().as_ref())?);
        }
        Ok(Self::new(mods))
    }
}

impl TryFrom<u32> for GameMods {
    type Error = OsuError;

    fn try_from(m: u32) -> Result<Self, Self::Error> {
        if m == 0 {
            return Ok(Self::default());
        }
        let mut mods = Vec::new();
        let mut curr = m;
        let mut bit = 1 << 31;
        while curr > 0 && bit > 0 {
            if (curr & bit) > 0 {
                if let Ok(game_mod) = GameMod::try_from(bit) {
                    mods.push(game_mod);
                    curr -= bit;
                } else if bit == 0x200 && (curr & 0x40) > 0 {
                    mods.push(GameMod::NightCore);
                    curr -= 0x240;
                } else if bit == 0x4000 && (curr & 0x20) > 0 {
                    mods.push(GameMod::Perfect);
                    curr -= 0x4020;
                }
            }
            bit >>= 1;
        }
        if curr > 0 {
            return Err(OsuError::ParseError(format!(
                "Can not parse u32 {} into GameMods",
                m
            )));
        }
        mods.reverse();
        Ok(Self::new(mods))
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'k> {
    iter: IterBTS<'k, GameMod>,
}

impl<'k> Iterator for Iter<'k> {
    type Item = &'k GameMod;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl FusedIterator for Iter<'_> {}

impl<'k> DoubleEndedIterator for Iter<'k> {
    fn next_back(&mut self) -> Option<&'k GameMod> {
        self.iter.next_back()
    }
}

impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

#[derive(Debug)]
pub struct IntoIter {
    entries: IntoIterBTS<GameMod>,
}

impl Iterator for IntoIter {
    type Item = GameMod;

    fn next(&mut self) -> Option<Self::Item> {
        self.entries.next()
    }
}

impl FusedIterator for IntoIter {}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<GameMod> {
        self.entries.next_back()
    }
}

impl ExactSizeIterator for IntoIter {
    fn len(&self) -> usize {
        self.entries.len()
    }
}

impl IntoIterator for GameMods {
    type Item = GameMod;
    type IntoIter = IntoIter;

    fn into_iter(self) -> IntoIter {
        IntoIter {
            entries: self.0.into_iter(),
        }
    }
}

mod util {
    /// Provide an iterator over substring of the given length on the given source string
    pub(crate) fn cut(mut source: &str, n: usize) -> impl Iterator<Item = &str> {
        std::iter::from_fn(move || {
            if source.is_empty() {
                None
            } else {
                let end_idx = source
                    .char_indices()
                    .nth(n - 1)
                    .map_or_else(|| source.len(), |(idx, ch)| idx + ch.len_utf8());
                let (sub_str, rest) = source.split_at(end_idx);
                source = rest;
                Some(sub_str)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mods_try_from_u32() {
        assert_eq!(GameMod::try_from(8).unwrap(), GameMod::Hidden);
        let mods = GameMods::new(vec![GameMod::HardRock, GameMod::Hidden]);
        assert_eq!(GameMods::try_from(24).unwrap(), mods);
    }

    #[test]
    fn test_mods_try_from_str() {
        assert_eq!(GameMod::try_from("HD").unwrap(), GameMod::Hidden);
        let mods = GameMods::new(vec![GameMod::HardRock, GameMod::Hidden]);
        assert_eq!(GameMods::try_from("HRHD").unwrap(), mods);
    }

    #[test]
    fn test_mods_iter() {
        let mods = GameMods::new(vec![GameMod::HardRock, GameMod::Hidden]);
        let mut iter = mods.iter();
        assert_eq!(iter.next().unwrap(), &GameMod::Hidden);
        assert_eq!(iter.next().unwrap(), &GameMod::HardRock);
        assert_eq!(iter.next(), None);
        assert_eq!(mods.len(), 2);
    }
}
