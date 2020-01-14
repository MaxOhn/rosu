use crate::models::GameMode;

#[derive(Default)]
pub struct ScoresReq {
    pub user_id: Option<u16>,
    pub username: Option<String>,
    pub map_id: Option<u16>,
    pub mode: Option<GameMode>,
    pub mods: Option<u32>,
    pub limit: Option<u32>,
}

impl ScoresReq {
    pub fn new() -> Self {
        Self::default()
    }
}
