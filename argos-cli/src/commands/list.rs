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
    top: Option<usize>,
) -> CliResult<()> {
    let mut system = sysinfo::System::new_all();
    system.refresh_processes();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL); // Espera breve
    system.refresh_processes();
    let mut processes: Vec<_> = system.processes().values().collect();

    // Filtrar por nombre
    if let Some(name) = name_filter {
        processes.retain(|p| p.name().contains(name));
    }
    // Filtrar por usuario
    if let Some(user) = user_filter {
        processes.retain(|p| {
            p.user_id()
                .and_then(|uid| get_user_by_id(uid.clone()))
                .map(|u| u.name.contains(user))
                .unwrap_or(false)
        });
    }
    // Ordenar
    sort_by_key(&mut processes, sort_by);

    // Convertir procesos a ProcessRow
    let process_rows: Vec<ProcessRow> = processes.iter().map(|p| process_to_row(p)).collect();
    
    // Limitar resultados si se solicita
    let rows = top_n_processes(&process_rows, top.unwrap_or(process_rows.len()));
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

fn sort_by_key(processes: &mut Vec<&sysinfo::Process>, sort_by: &str) {
    match sort_by {
        "cpu" => processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap()),
        "memory" => processes.sort_by(|a, b| b.memory().cmp(&a.memory())),
        "name" => processes.sort_by(|a, b| a.name().cmp(b.name())),
        "pid" => processes.sort_by(|a, b| a.pid().cmp(&b.pid())),
        _ => {}
    }
}

fn top_n_processes(processes: &[ProcessRow], n: usize) -> Vec<ProcessRow> {
    let mut sorted_processes = processes.to_vec();
    sorted_processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
    sorted_processes.into_iter().take(n).collect()
}