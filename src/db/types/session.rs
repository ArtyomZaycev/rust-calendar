use chrono::{Duration, NaiveDateTime, Utc};
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbSession {
    pub id: i32,
    pub password_id: i32,
    pub key: Vec<u8>,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub valid: bool,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::sessions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbNewSession {
    pub user_id: i32,
    pub key: Vec<u8>,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl DbNewSession {
    fn generate_key() -> Vec<u8> {
        let mut key = [0u8; 64];
        rand::rngs::OsRng.fill_bytes(&mut key);
        key.map(|v| v % 128).to_vec()
    }

    pub fn new(user_id: i32) -> Self {
        let key = DbNewSession::generate_key();
        let start = Utc::now().naive_utc();
        let end = start
            .checked_add_signed(Duration::try_days(1).unwrap())
            .unwrap_or_default();

        DbNewSession {
            user_id,
            key,
            start,
            end,
        }
    }
}
