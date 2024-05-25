use crate::error::Error;
use diesel::prelude::*;

mod sql {
    use diesel::sql_function;
    sql_function!(fn last_insert_id() -> Unsigned<Bigint>);
}

pub fn last_insert_id(connection: &mut MysqlConnection) -> Result<i32, Error> {
    diesel::select(sql::last_insert_id())
        .get_result::<u64>(connection)
        .map(|v| v as i32)
        .map_err(|e| Error::DieselError(e))
}
