#![allow(non_upper_case_globals)]

use crate::{error::ModError, model::GameMode, OsuError};

use std::{
    convert::{Into, TryFrom},
    fmt,
    str::FromStr,
};

bitflags! {
    /// Enum for all game modifications.
    /// Implemented as [bitflags](https://crates.io/crates/bitflags).
    ///
    /// # Example
    /// ```
    /// use rosu::model::GameMods;
    /// use std::str::FromStr;
    ///
    /// let nomod = GameMods::default();
    /// assert_eq!(nomod, GameMods::NoMod);
    ///
    /// // Bitwise creating, or from u32
    /// let hdhr_1 = GameMods::HardRock | GameMods::Hidden;
    /// let hdhr_2 = GameMods::from_bits(8 + 16).unwrap();
    /// assert_eq!(hdhr_1, hdhr_2);
    ///
    /// // contains, intersects, and a few more methods from bitflags
    /// let ezhdpf = GameMods::Easy | GameMods::Hidden | GameMods::Perfect;
    /// assert!(!ezhdpf.contains(GameMods::HardRock));
    /// let hdpf = GameMods::Hidden | GameMods::Perfect;
    /// assert!(ezhdpf.intersects(hdpf));
    ///
    /// // Try converting from &str
    /// let hdhrdt = GameMods::from_str("dthdhr").unwrap();
    /// assert_eq!(hdhrdt.bits(), 8 + 16 + 64);
    /// // Implements fmt::Display
    /// assert_eq!(hdhrdt.to_string(), "HDHRDT".to_string());
    ///
    /// // Iterator
    /// let mut mod_iter = GameMods::from_bits(536871512).unwrap().iter();
    /// assert_eq!(mod_iter.next(), Some(GameMods::Hidden));
    /// assert_eq!(mod_iter.next(), Some(GameMods::HardRock));
    /// assert_eq!(mod_iter.next(), Some(GameMods::NightCore));
    /// assert_eq!(mod_iter.next(), Some(GameMods::ScoreV2));
    /// assert_eq!(mod_iter.next(), None);
    /// ```
    #[derive(Default)]
    pub struct GameMods: u32 {
        const NoMod = 0;
        const NoFail = 1;
        const Easy = 2;
        const TouchDevice = 4;
        const Hidden = 8;
        const HardRock = 16;
        const SuddenDeath = 32;
        const DoubleTime = 64;
        const Relax = 128;
        const HalfTime = 256;
        const NightCore = 512 | Self::DoubleTime.bits;
        const Flashlight = 1024;
        const SpunOut = 4096;
        const Perfect = 16_384 | Self::SuddenDeath.bits;
        const FadeIn = 1_048_576;
        const ScoreV2 = 536_870_912;
        const Mirror = 1_073_741_824;

        const Key1 = 67_108_864;
        const Key2 = 268_435_456;
        const Key3 = 134_217_728;
        const Key4 = 32_768;
        const Key5 = 65_536;
        const Key6 = 131_072;
        const Key7 = 262_144;
        const Key8 = 524_288;
        const Key9 = 16_777_216;
        const KeyCoop = 33_554_432;

        const Autoplay = 2048;
        const Autopilot = 8192;
        const Cinema = 4_194_304;
        const Random = 2_097_152;
        const Target = 8_388_608;
    }
}

#[allow(clippy::len_without_is_empty)]
impl GameMods {
    /// Method that checks whether [`GameMods`] contains one of osu!mania's key mods.
    ///
    /// [`GameMods`]: struct.GameMods.html
    ///
    /// # Examples
    /// ```
    /// use rosu::model::GameMods;
    ///
    /// let mods = GameMods::Hidden | GameMods::Key4;
    /// assert_eq!(mods.has_key_mod(), Some(GameMods::Key4));
    /// assert_eq!(GameMods::Hidden.has_key_mod(), None);
    /// ```
    pub fn has_key_mod(self) -> Option<GameMods> {
        if self.contains(GameMods::Key1) {
            Some(GameMods::Key1)
        } else if self.contains(GameMods::Key2) {
            Some(GameMods::Key2)
        } else if self.contains(GameMods::Key3) {
            Some(GameMods::Key3)
        } else if self.contains(GameMods::Key4) {
            Some(GameMods::Key4)
        } else if self.contains(GameMods::Key5) {
            Some(GameMods::Key5)
        } else if self.contains(GameMods::Key6) {
            Some(GameMods::Key6)
        } else if self.contains(GameMods::Key7) {
            Some(GameMods::Key7)
        } else if self.contains(GameMods::Key8) {
            Some(GameMods::Key8)
        } else if self.contains(GameMods::Key9) {
            Some(GameMods::Key9)
        } else {
            None
        }
    }

    /// Calculate the multiplier of the mods which will
    /// influence a [`Score`]'s playscore
    ///
    /// [`Score`]: struct.Score.html
    ///
    /// # Example
    /// ```rust
    /// use rosu::model::{GameMods, GameMode};
    ///
    /// let ezhd = GameMods::from_bits(2 + 8).unwrap();
    /// assert_eq!(ezhd.score_multiplier(GameMode::STD), 0.53);
    /// assert_eq!(ezhd.score_multiplier(GameMode::MNA), 0.5);
    /// ```
    pub fn score_multiplier(self, mode: GameMode) -> f32 {
        self.into_iter()
            .map(|m| match mode {
                GameMode::STD => match m {
                    GameMods::HalfTime => 0.3,
                    GameMods::Easy | GameMods::NoFail => 0.5,
                    GameMods::SpunOut => 0.9,
                    GameMods::HardRock | GameMods::Hidden => 1.06,
                    GameMods::DoubleTime | GameMods::NightCore | GameMods::Flashlight => 1.12,
                    _ => 1.0,
                },
                GameMode::TKO => match m {
                    GameMods::HalfTime => 0.3,
                    GameMods::Easy | GameMods::NoFail => 0.5,
                    GameMods::HardRock | GameMods::Hidden => 1.06,
                    GameMods::DoubleTime | GameMods::NightCore | GameMods::Flashlight => 1.12,
                    _ => 1.0,
                },
                GameMode::CTB => match m {
                    GameMods::HalfTime => 0.3,
                    GameMods::Easy | GameMods::NoFail => 0.5,
                    GameMods::DoubleTime | GameMods::NightCore | GameMods::Hidden => 1.06,
                    GameMods::HardRock | GameMods::Flashlight => 1.12,
                    _ => 1.0,
                },
                GameMode::MNA => match m {
                    GameMods::Easy | GameMods::NoFail | GameMods::HalfTime => 0.5,
                    _ => 1.0,
                },
            })
            .product()
    }

    /// Check if a [`Score`]'s playscore will be increased
    ///
    /// [`Score`]: struct.Score.html
    ///
    /// # Example
    /// ```rust
    /// use rosu::model::{GameMods, GameMode};
    ///
    /// let hrso = GameMods::HardRock | GameMods::SpunOut;
    /// assert!(!hrso.increases_score(GameMode::STD));
    /// assert!(GameMods::DoubleTime.increases_score(GameMode::TKO));
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
    /// use rosu::model::{GameMods, GameMode};
    ///
    /// let hrso = GameMods::HardRock | GameMods::SpunOut;
    /// assert!(hrso.decreases_score(GameMode::STD));
    /// assert!(!GameMods::DoubleTime.decreases_score(GameMode::TKO));
    /// ```
    pub fn decreases_score(self, mode: GameMode) -> bool {
        self.score_multiplier(mode) < 1.0
    }

    /// Check if a [`Beatmap`]'s star rating for the given [`GameMode`] will be influenced.
    ///
    /// [`Beatmap`]: struct.Beatmap.html
    /// [`GameMode`]: struct.GameMode.html
    ///
    /// # Example
    /// ```rust
    /// use rosu::model::{GameMode, GameMods};
    ///
    /// let hdhr = GameMods::Hidden | GameMods::HardRock;
    /// assert!(hdhr.changes_stars(GameMode::STD));
    /// assert!(!hdhr.changes_stars(GameMode::MNA));
    /// let nc = GameMods::NightCore;
    /// assert!(nc.changes_stars(GameMode::MNA));
    /// ```
    pub fn changes_stars(self, mode: GameMode) -> bool {
        if self.intersects(GameMods::DoubleTime | GameMods::NightCore | GameMods::HalfTime) {
            true
        } else if self.intersects(GameMods::HardRock | GameMods::Easy) {
            mode == GameMode::STD || mode == GameMode::CTB
        } else {
            false
        }
    }

    /// Returns an iterator. Alias of `into_iter`.
    ///
    /// [`GameMods`]: struct.GameMods.html
    ///
    /// # Example
    /// ```
    /// use rosu::model::GameMods;
    ///
    /// let mods = GameMods::from_bits(8 + 16 + 64 + 128).unwrap();
    /// let mut mod_iter = mods.iter();
    /// assert_eq!(mod_iter.next(), Some(GameMods::Hidden));
    /// assert_eq!(mod_iter.next(), Some(GameMods::HardRock));
    /// assert_eq!(mod_iter.next(), Some(GameMods::DoubleTime));
    /// assert_eq!(mod_iter.next(), Some(GameMods::Relax));
    /// assert_eq!(mod_iter.next(), None);
    /// ```
    pub fn iter(self) -> IntoIter {
        self.into_iter()
    }

    /// Returns the amount of contained mods
    ///
    /// # Example
    /// ```
    /// use rosu::model::GameMods;
    ///
    /// assert_eq!(GameMods::NoMod.len(), 0);
    /// let mods = GameMods::from_bits(8 + 16 + 64 + 128).unwrap();
    /// assert_eq!(mods.len(), 4);
    /// ```
    pub fn len(self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.into_iter().count()
        }
    }
}

impl fmt::Display for GameMods {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for m in self.into_iter() {
            let abbrev = match m {
                GameMods::NoMod => "NM",
                GameMods::NoFail => "NF",
                GameMods::Easy => "EZ",
                GameMods::TouchDevice => "TD",
                GameMods::Hidden => "HD",
                GameMods::HardRock => "HR",
                GameMods::SuddenDeath => "SD",
                GameMods::DoubleTime => "DT",
                GameMods::Relax => "RX",
                GameMods::HalfTime => "HT",
                GameMods::NightCore => "NC",
                GameMods::Flashlight => "FL",
                GameMods::SpunOut => "SO",
                GameMods::Autopilot => "AP",
                GameMods::Perfect => "PF",
                GameMods::FadeIn => "FI",
                GameMods::Random => "RD",
                GameMods::Target => "TP",
                GameMods::ScoreV2 => "V2",
                GameMods::Mirror => "MR",
                GameMods::Key1 => "1K",
                GameMods::Key2 => "2K",
                GameMods::Key3 => "3K",
                GameMods::Key4 => "4K",
                GameMods::Key5 => "5K",
                GameMods::Key6 => "6K",
                GameMods::Key7 => "7K",
                GameMods::Key8 => "8K",
                GameMods::Key9 => "9K",
                GameMods::Autoplay => "",
                GameMods::Cinema => "",
                GameMods::KeyCoop => "",
                _ => unreachable!(),
            };
            write!(f, "{}", abbrev)?;
        }
        Ok(())
    }
}

impl Into<u32> for GameMods {
    fn into(self) -> u32 {
        self.bits
    }
}

impl TryFrom<u32> for GameMods {
    type Error = OsuError;
    fn try_from(m: u32) -> Result<Self, Self::Error> {
        GameMods::from_bits(m).ok_or(OsuError::ModParsing(ModError::U32(m)))
    }
}

impl FromStr for GameMods {
    type Err = OsuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = GameMods::default();
        let upper = s.to_uppercase();
        for m in util::cut(&upper, 2) {
            let m = match m {
                "NM" => GameMods::NoMod,
                "NF" => GameMods::NoFail,
                "EZ" => GameMods::Easy,
                "TD" => GameMods::TouchDevice,
                "HD" => GameMods::Hidden,
                "HR" => GameMods::HardRock,
                "SD" => GameMods::SuddenDeath,
                "DT" => GameMods::DoubleTime,
                "RX" | "RL" => GameMods::Relax,
                "HT" => GameMods::HalfTime,
                "NC" => GameMods::NightCore,
                "FL" => GameMods::Flashlight,
                "SO" => GameMods::SpunOut,
                "AP" => GameMods::Autopilot,
                "PF" => GameMods::Perfect,
                "FI" => GameMods::FadeIn,
                "RD" => GameMods::Random,
                "TP" => GameMods::Target,
                "V2" => GameMods::ScoreV2,
                "MR" => GameMods::Mirror,
                "1K" | "K1" => GameMods::Key1,
                "2K" | "K2" => GameMods::Key2,
                "3K" | "K3" => GameMods::Key3,
                "4K" | "K4" => GameMods::Key4,
                "5K" | "K5" => GameMods::Key5,
                "6K" | "K6" => GameMods::Key6,
                "7K" | "K7" => GameMods::Key7,
                "8K" | "K8" => GameMods::Key8,
                "9K" | "K9" => GameMods::Key9,
                "NO" if s == "NOMOD" => break,
                _ => return Err(OsuError::ModParsing(ModError::Str)),
            };
            res.insert(m);
        }
        Ok(res)
    }
}

pub struct IntoIter {
    mods: GameMods,
    shift: usize,
}

impl Iterator for IntoIter {
    type Item = GameMods;
    fn next(&mut self) -> Option<Self::Item> {
        if self.mods.is_empty() {
            if self.shift < 32 {
                self.shift = 32;
                Some(GameMods::NoMod)
            } else {
                None
            }
        } else {
            loop {
                if self.shift == 32 {
                    return None;
                }
                let mut bit = 1 << self.shift;
                self.shift += 1;
                if (bit == 32 && self.mods.contains(GameMods::Perfect))
                    || (bit == 64 && self.mods.contains(GameMods::NightCore))
                {
                    continue;
                } else if bit == 512 {
                    bit += GameMods::DoubleTime.bits
                } else if bit == 16_384 {
                    bit += GameMods::SuddenDeath.bits
                }
                if self.mods.bits & bit == bit {
                    return GameMods::from_bits(bit);
                }
            }
        }
    }
    fn count(self) -> usize {
        let (len, _) = self.size_hint();
        len
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.mods.bits().count_ones() as usize
            - (self.mods.contains(GameMods::NightCore) as usize)
            - (self.mods.contains(GameMods::Perfect) as usize);
        (len, Some(len))
    }
}

impl IntoIterator for GameMods {
    type Item = GameMods;
    type IntoIter = IntoIter;
    fn into_iter(self) -> IntoIter {
        IntoIter {
            mods: self,
            shift: 0,
        }
    }
}

mod util {
    /// Provide an iterator over substrings of the given length on the given source string
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
    fn test_mods_try_from_str() {
        assert_eq!(GameMods::from_str("NM").unwrap(), GameMods::NoMod);
        assert_eq!(GameMods::from_str("HD").unwrap(), GameMods::Hidden);
        let mods = GameMods::from_bits(24).unwrap();
        assert_eq!(GameMods::from_str("HRHD").unwrap(), mods);
        assert!(GameMods::from_str("HHDR").is_err());
    }

    #[test]
    fn test_mods_iter() {
        let mut iter = GameMods::default().iter();
        assert_eq!(iter.next().unwrap(), GameMods::NoMod);
        assert_eq!(iter.next(), None);
        let mut iter = GameMods::from_bits(24).unwrap().iter();
        assert_eq!(iter.next().unwrap(), GameMods::Hidden);
        assert_eq!(iter.next().unwrap(), GameMods::HardRock);
        assert_eq!(iter.next(), None);
    }
}
