use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use diesel::MysqlConnection;

pub struct AppState {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl AppState {
    pub fn new(pool: Pool<ConnectionManager<MysqlConnection>>) -> Self {
        Self { pool }
    }

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<MysqlConnection>> {
        self.pool.get().unwrap()
    }
}

pub struct WorkerState {}

impl WorkerState {
    pub fn new() -> Self {
        Self {}
    }
}
