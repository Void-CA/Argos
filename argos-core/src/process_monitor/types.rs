use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub timestamp: f64,      // tiempo relativo desde inicio
    pub cpu_usage: f32,      // %
    pub memory: u64,         // KB
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: String,
    pub cpu_usage: f32,
    pub memory_mb: f64,
    pub start_time: u64,
    pub parent_pid: Option<u32>,
}

#[derive(Debug)]
pub struct Snapshot {
    pub timestamp: f64,
    pub cpu_usage: f32,
    pub memory_kb: u64,
    pub read_bytes: Option<u64>,
    pub write_bytes: Option<u64>,
    pub net_in_bytes: Option<u64>,
    pub net_out_bytes: Option<u64>,
}

#[derive(Debug)]
pub struct LogSession {
    pub id: String,
    pub process_pid: u32,
    pub process_start_time: u64,
    pub start_datetime: DateTime<Utc>,
    pub duration_secs: f64,
    pub interval_ms: u64,
    pub num_samples: usize,
    pub snapshots: Vec<Snapshot>,
}

#[derive(Debug)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub cmdline: Option<String>,
    pub parent_pid: Option<u32>,
    pub created_at: u64,
}

#[derive(Debug)]
pub enum CoreError {
    ProcessEnded,
    Other(String)
    // Add other error variants as needed
}