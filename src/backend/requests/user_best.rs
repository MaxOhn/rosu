use crate::models::GameMode;

pub struct UserBestReq {
    pub user_id: Option<u16>,
    pub username: Option<String>,
    pub mode: Option<GameMode>,
    pub limit: Option<u32>,
}

impl UserBestReq {
    pub fn new() -> Self {
        Self {
            user_id: None,
            username: None,
            mode: None,
            limit: None,
        }
    }
}
