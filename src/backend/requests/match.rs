#[derive(Clone, Eq, PartialEq)]
/// Request struct to retrieve users. An instance __must__ contains either a user id or a username
pub struct MatchArgs {
    pub(crate) match_id: u32,
}

impl MatchArgs {
    /// Construct a `MatchArgs` via match id
    pub fn with_match_id(id: u32) -> Self {
        Self { match_id: id }
    }
}
