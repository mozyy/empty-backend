use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

use crate::errors::{ServiceResult};

// pub type DbConnection = PooledConnection<PgConnection>;

#[derive(Clone)]
pub struct DbPool(r2d2::Pool<ConnectionManager<PgConnection>>);

impl DbPool {
    fn new() -> Self {
        // set up database connection pool
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        log::debug!("Connection Pg pool database:111 {database_url}");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        let db_pool = r2d2::Pool::builder()
            .max_size(5)
            .build(manager)
            .expect("Failed to create pool.");
        if let Some((_, database)) = database_url.split_once('@') {
            log::debug!("Connection Pg pool database: {database}");
        } else {
            log::debug!("Connection Pg pool database_url: {database_url}");
        }

        Self(db_pool)
    }

    pub fn get_conn(&self) -> ServiceResult<PooledConnection<ConnectionManager<PgConnection>>> {
        let conn = self.0.get()?;
        Ok(conn)
    }
}

impl Default for DbPool {
    fn default() -> Self {
        Self::new()
    }
}
