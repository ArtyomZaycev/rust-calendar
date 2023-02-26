use chrono::{Duration, NaiveDateTime, Utc};
use rand::RngCore;

#[derive(Debug, Clone, diesel::Queryable)]
pub struct DbSession {
    pub id: i32,
    pub user_id: i32,
    pub key: Vec<u8>,
    pub access_level: i32,
    pub edit_right: bool,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub valid: bool,
}

#[derive(Debug, Clone, diesel::Insertable)]
#[diesel(table_name = crate::db::schema::sessions)]
pub struct DbNewSession {
    pub user_id: i32,
    pub key: Vec<u8>,
    pub access_level: i32,
    pub edit_right: bool,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl DbNewSession {
    fn generate_key() -> Vec<u8> {
        let mut key = [0u8; 64];
        rand::rngs::OsRng.fill_bytes(&mut key);
        key.map(|v| v % 128).to_vec()
    }

    pub fn new(user_id: i32, access_level: i32, edit_right: bool) -> Self {
        let key = DbNewSession::generate_key();
        let start = Utc::now().naive_utc();
        let end = start
            .checked_add_signed(Duration::days(1))
            .unwrap_or_default();

        DbNewSession {
            user_id,
            key,
            access_level,
            edit_right,
            start,
            end,
        }
    }
}
