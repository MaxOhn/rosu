mod approval;
mod beatmap;
mod game_mod;
mod genre;
mod grade;
mod language;
mod r#match;
mod mode;
mod score;
mod user;

pub use approval::ApprovalStatus;
pub use beatmap::Beatmap;
pub use game_mod::{GameMod, GameMods};
pub use genre::Genre;
pub use grade::Grade;
pub use language::Language;
pub use mode::GameMode;
pub use r#match::{GameScore, Match, MatchGame, ScoringType, Team, TeamType};
pub use score::Score;
pub use user::{Event, User};

use crate::backend::OsuApi;
use std::sync::{Arc, RwLock};

/// Helper trait to provide a way to set the LazilyLoaded fields of a struct after deserializing it.
pub trait HasLazies {
    /// Artifact from the public `HasLazies` trait. This method has no use outside of this library.
    fn prepare_lazies(&mut self, osu: Arc<RwLock<OsuApi>>);
}
