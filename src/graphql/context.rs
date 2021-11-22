use crate::db::DbPool;

#[derive(Clone)]
pub struct Context {
    pub db_pool: DbPool,
}

impl juniper::Context for Context {}

impl AsRef<Self> for Context {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}
