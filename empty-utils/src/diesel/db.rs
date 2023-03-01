use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

pub type DbConnection = PooledConnection<PgConnection>;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get() -> DbPool {
    // set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let db_pool = r2d2::Pool::builder()
        .max_size(2)
        .build(manager)
        .expect("Failed to create pool.");
    if let Some((_, database)) = database_url.split_once('@') {
        log::debug!("Connection Pg pool database: {database}");
    } else {
        log::debug!("Connection Pg pool database_url: {database_url}");
    }
    db_pool
}
