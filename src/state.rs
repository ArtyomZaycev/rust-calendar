use diesel::r2d2::{ConnectionManager, PooledConnection};
use std::sync::{Arc, Mutex};

use diesel::MysqlConnection;

pub struct AppState {
    pub pool: Arc<Mutex<PooledConnection<ConnectionManager<MysqlConnection>>>>,
}

impl AppState {
    pub fn new(pool: PooledConnection<ConnectionManager<MysqlConnection>>) -> Self {
        Self {
            pool: Arc::new(Mutex::new(pool)),
        }
    }
}

pub struct WorkerState {}

impl WorkerState {
    pub fn new() -> Self {
        Self {}
    }
}
