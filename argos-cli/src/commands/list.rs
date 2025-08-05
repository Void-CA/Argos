use std::fs;
use argos_core::utils::process::{process_to_row, ProcessRow};
use argos_core::users::utils::get_user_by_id;
use crate::output::OutputFormatter;
use crate::error::{CliResult, CliError};

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
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL); // Espera breve
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