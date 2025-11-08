pub mod config;
pub mod domain;
pub mod schema;
pub mod usecase;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> DbPool {
    let database_url = config::CONFIG.postgres.database_url();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
