use sysinfo::{System,};
use crate::process_monitor::types::ProcessInfo;

pub fn monitor_process(pid: u32) -> Option<ProcessInfo> {
    let mut system = System::new_all();
    system.refresh_process(sysinfo::Pid::from_u32(pid));

    if let Some(process) = system.process(sysinfo::Pid::from_u32(pid)) {
        Some(ProcessInfo {
            pid,
            name: process.name().to_string(),
            state: format!("{:?}", process.status()),
            cpu_usage: process.cpu_usage() as f32,
            memory_mb: process.memory() as f64 / 1024.0,
            start_time: process.start_time(),
            parent_pid: process.parent().map(|p| p.as_u32()),
        })
    } else {
        None
    }
}
