use crate::error::{CliResult, CliError};
use crate::output::OutputFormatter;
use argos_export::ProcessRow;
use argos_core::users::utils::get_user_by_id;
use std::fs;

pub fn handle_list(
    formatter: &OutputFormatter,
    name_filter: Option<&str>,
    user_filter: Option<&str>,
    sort_by: &str,
    format: &str,
    output_file: Option<&str>,
) -> CliResult<()> {
    let mut system = sysinfo::System::new_all();
    system.refresh_processes();
    let mut processes: Vec<_> = system.processes().values().collect();
    if let Some(name) = name_filter {
        processes.retain(|p| p.name().contains(name));
    }
    if let Some(user) = user_filter {
        processes.retain(|p| {
            p.user_id()
                .and_then(|uid| get_user_by_id(uid.clone()))
                .map(|u| u.name.contains(user))
                .unwrap_or(false)
        });
    }
    match sort_by {
        "cpu" => processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap()),
        "memory" => processes.sort_by(|a, b| b.memory().cmp(&a.memory())),
        "name" => processes.sort_by(|a, b| a.name().cmp(b.name())),
        "pid" => processes.sort_by(|a, b| a.pid().cmp(&b.pid())),
        _ => {}
    }
    let rows: Vec<ProcessRow> = processes.iter().map(|p| process_to_row(p)).collect();
    let output = formatter.format_process_list(&rows, format)?;
    if let Some(file_path) = output_file {
        fs::write(file_path, &output)
            .map_err(|e| CliError::io_error(format!("Error al escribir archivo: {}", e)))?;
        if format == "text" {
            println!("✅ Resultados guardados en: {}", file_path);
        }
    } else {
        println!("{}", output);
    }
    Ok(())
}

fn process_to_row(p: &sysinfo::Process) -> ProcessRow {
    let myuser = p.user_id().and_then(|uid| get_user_by_id(uid.clone()));
    let user_name = myuser.as_ref().map(|u| u.name.as_str()).unwrap_or("-").to_string();
    let groups = myuser.as_ref().map(|u| u.groups.join(",")).unwrap_or_else(|| "-".to_string());
    let state = format!("{:?}", p.status());
    let start_time = p.start_time();
    let parent_pid = p.parent().map(|pp| pp.as_u32());
    let virtual_memory_mb = p.virtual_memory() as f64 / 1024.0;
    let read_disk_usage = p.disk_usage().total_read_bytes as f64 / 1024.0;
    ProcessRow {
        pid: p.pid().as_u32(),
        name: p.name().to_string(),
        cpu_usage: p.cpu_usage() as f64,
        memory_mb: p.memory() as f64 / 1024.0,
        user: user_name,
        groups,
        state,
        start_time,
        parent_pid,
        virtual_memory_mb,
        read_disk_usage,
    }
}
