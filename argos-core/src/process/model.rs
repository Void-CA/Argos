use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProcessRow {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f64,
    pub memory_mb: f64,
    pub user: String,
    pub groups: String,
    pub state: String,
    pub start_time: u64,
    pub start_time_human: String,
    pub parent_pid: Option<u32>,
    pub virtual_memory_mb: f64,
    pub read_disk_usage: f64,
    pub write_disk_usage: f64,
    pub exe: String,
    pub cmd: String,
    // ...otros campos si es necesario
}

