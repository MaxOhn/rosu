mod serde_approval_status;
mod serde_bool;
mod serde_date_mod;
mod serde_f32;
mod serde_genre;
mod serde_grade;
mod serde_language;
mod serde_mode;
mod serde_mods;
mod serde_scoring_type;
mod serde_team;
mod serde_team_type;
mod serde_u32;
mod serde_u64;

pub(crate) use serde_bool::*;
pub(crate) use serde_date_mod::{serde_date, serde_maybe_date};
pub(crate) use serde_f32::*;
pub(crate) use serde_mods::*;
pub(crate) use serde_u32::*;
pub(crate) use serde_u64::*;

#[cfg(feature = "serialize")]
pub(crate) fn default_u32(n: &u32) -> bool {
    n == &0
}

#[cfg(feature = "serialize")]
pub(crate) fn default_bool(b: &bool) -> bool {
    !b
}

#[cfg(feature = "serialize")]
pub(crate) fn default_vec<T>(v: &Vec<T>) -> bool {
    v.is_empty()
}
