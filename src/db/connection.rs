use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenvy::dotenv;
use std::env;

pub fn establish_pooled_connection() -> PooledConnection<ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Pool::builder()
        .build(ConnectionManager::<MysqlConnection>::new(&database_url))
        .expect("Failed to create pool.")
        .clone()
        .get()
        .unwrap()
}
