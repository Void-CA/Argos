use diesel::prelude::*;
use crate::db::schema::log_sessions;
use crate::models::{LogSession, NewLogSession};

pub fn insert_log_session(conn: &mut SqliteConnection, new_log: &NewLogSession) -> QueryResult<usize> {
    diesel::insert_into(log_sessions::table)
        .values(new_log)
        .execute(conn)
}


pub fn get_log_session_by_id(conn: &mut SqliteConnection, id_val: &str) -> QueryResult<LogSession> {
    log_sessions::table
        .filter(log_sessions::id.eq(id_val))
        .first(conn)
}

// Funciones adicionales para update o delete si es necesario
