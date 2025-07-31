use diesel::prelude::*;
use crate::db::schema::{processes, log_sessions, samples};
use serde::{Serialize, Deserialize};

// Model para la tabla processes
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = processes)]
pub struct Process {
    pub pid: i32,                     // Ya es PRIMARY KEY, no debería ser Option
    pub name: String,
    pub state: String,
    pub memory_mb: Option<f32>,
    pub start_time: Option<i32>,      // Cambiar a i32 para Unix timestamp (Diesel Integer)
    pub parent_pid: Option<i32>,
}


#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = log_sessions)]
pub struct LogSession {
    pub id: String,                                       // PRIMARY KEY manual
    pub process_pid: i32,
    pub started_at: Option<chrono::NaiveDateTime>,        // Diesel usa NaiveDateTime
    pub duration_secs: Option<i32>,
    pub iterations: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = log_sessions)]
pub struct NewLogSession<'a> {
    pub id: &'a str,
    pub process_pid: i32,
    pub duration_secs: Option<i32>,
    pub iterations: Option<i32>,
}


// Model para la tabla samples
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = samples)]
pub struct Sample {
    pub id: Option<i32>,
    pub log_id: String,
    pub timestamp: f32,
    pub cpu_usage: f32,
    pub memory: i32,
}
