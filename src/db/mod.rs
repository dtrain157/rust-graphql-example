use super::config;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub mod models;
pub mod schema;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(config::DATABASE_URL());
    Pool::new(manager).expect("Failed to create DB Pool")
}
