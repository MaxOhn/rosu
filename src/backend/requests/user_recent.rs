use crate::models::GameMode;

pub struct UserRecentReq {
    pub user_id: Option<u16>,
    pub username: Option<String>,
    pub mode: Option<GameMode>,
    pub limit: Option<u32>,
}

impl UserRecentReq {
    pub fn new() -> Self {
        Self {
            user_id: None,
            username: None,
            mode: None,
            limit: None,
        }
    }

    pub fn user_id<'a>(&'a mut self, id: u16) -> &'a mut Self {
        self.user_id = Some(id);
        self
    }

    pub fn username<'a>(&'a mut self, name: String) -> &'a mut Self {
        self.username = Some(name);
        self
    }
}
