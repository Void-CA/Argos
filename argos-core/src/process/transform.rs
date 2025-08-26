use crate::{process::model::ProcessRow, users::get_user_by_id};

// Conversión de sysinfo::Process a argos_export::ProcessRow
pub fn process_to_row(p: &sysinfo::Process) -> ProcessRow {
    let myuser = p.user_id().and_then(|uid| get_user_by_id(uid.clone()));
    let user_name = myuser.as_ref().map(|u| u.name.as_str()).unwrap_or("-").to_string();
    let groups = myuser.as_ref().map(|u| u.groups.join(",")).unwrap_or_else(|| "-".to_string());
    let state = format!("{:?}", p.status());
    let exe = p.exe().map(|path| path.display().to_string()).unwrap_or_else(|| "-".to_string());
    let cmd = p.cmd().join(" ");
    let start_time = p.start_time();
    let parent_pid = p.parent().map(|pp| pp.as_u32());
    let virtual_memory_mb = (p.virtual_memory() as f64 / 1024.0) / 1024.0;
    let read_disk_usage = p.disk_usage().total_read_bytes as f64 / 1024.0;
    let write_disk_usage = p.disk_usage().total_written_bytes as f64 / 1024.0;

    ProcessRow {
        pid: p.pid().as_u32(),
        name: p.name().to_string(),
        cpu_usage: p.cpu_usage() as f64,
        memory_mb: (p.memory() as f64 / 1024.0) / 1024.0,
        user: user_name,
        groups,
        state,
        exe,
        cmd,
        start_time,
        start_time_human: format_start_time(start_time),
        parent_pid,
        virtual_memory_mb,
        read_disk_usage,
        write_disk_usage,
    }
}

fn format_start_time(start_time: u64) -> String {
    use chrono::{DateTime};
    match DateTime::from_timestamp(start_time as i64, 0) {
        Some(datetime) => {
            datetime.format("%H:%M:%S").to_string()
        }
        None => "-".to_string(),
    }
}