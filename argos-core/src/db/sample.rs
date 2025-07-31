use diesel::prelude::*;
use crate::db::schema::*; // or specify the correct table/module if different
use crate::models::Sample; // Adjust the path according to your project structure

pub fn insert_sample(conn: &mut SqliteConnection, sample: &Sample) -> QueryResult<usize> {
    diesel::insert_into(samples::table)
        .values(sample)
        .execute(conn)
}

pub fn get_samples_by_log(conn: &mut SqliteConnection, log_id_val: &str) -> QueryResult<Vec<Sample>> {
    samples::table
        .filter(samples::log_id.eq(log_id_val))
        .load(conn)
}

// Más funciones si necesitas
