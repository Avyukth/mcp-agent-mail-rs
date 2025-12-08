#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64,
}

impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }

    pub fn new(user_id: i64) -> Self {
        Ctx { user_id }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
