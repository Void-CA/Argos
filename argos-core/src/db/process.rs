use diesel::prelude::*;
use crate::db::schema::processes;
use crate::models::Process;  // Asegúrate de tener tus structs en models.rs

pub fn insert_process(conn: &mut SqliteConnection, process: &Process) -> QueryResult<usize> {
    diesel::insert_into(processes::table)
        .values(process)
        .execute(conn)
}

pub fn get_process_by_pid(conn: &mut SqliteConnection, pid_val: i32) -> QueryResult<Process> {
    processes::table
        .filter(processes::pid.eq(pid_val))
        .first(conn)
}

// Más funciones: update_process, delete_process...
