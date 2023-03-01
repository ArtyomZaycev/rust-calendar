use diesel::{MysqlConnection, RunQueryDsl};

use crate::error::Error;

mod sql {
    use diesel::sql_function;
    sql_function!(fn last_insert_id() -> Unsigned<Bigint>);
}

pub fn last_insert_id(connection: &mut MysqlConnection) -> Result<usize, Error> {
    diesel::select(sql::last_insert_id())
        .execute(connection)
        .map_err(|e| Error::DieselError(e))
}
