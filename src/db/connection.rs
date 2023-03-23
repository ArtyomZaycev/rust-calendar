use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

pub fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    // Needed to load env::var
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Pool::builder()
        .max_size(4)
        .build(ConnectionManager::<MysqlConnection>::new(&database_url))
        .expect("Failed to create pool.")
}
