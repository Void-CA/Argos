use crate::process_monitor::types::ProcessInfo;
use crate::models::Process;

impl From<ProcessInfo> for Process {
    fn from(info: ProcessInfo) -> Self {
        Process {
            pid: info.pid as i32,
            name: info.name,
            state: info.state,
            memory_mb: Some(info.memory_mb as f32),
            start_time: Some(info.start_time as i32),
            parent_pid: info.parent_pid.map(|p| p as i32),
        }
    }
}
