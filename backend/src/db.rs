use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::Config;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool(config: &Config) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    Pool::builder().max_size(10).build(manager).unwrap()
}