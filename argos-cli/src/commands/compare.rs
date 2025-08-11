use std::path::PathBuf;
use argos_core::process::{model::ProcessRow, transform::process_to_row};
use argos_core::
use sysinfo::Pid;

use crate::{error::{CliError, CliResult}, output::OutputFormatter};



pub fn handle_compare(
    formatter: &OutputFormatter,
    pids: Option<Vec<u32>>,
    files: Option<Vec<PathBuf>>,
    format: &str,
    output: Option<&str>,
) -> CliResult<()> {
    validate_comparison_inputs(&pids, &files)?;
    validate_pids(&pids)?;
    validate_files(&files)?;

    // Delegación de lógica según el modo
    match (pids, files) {
        (Some(pids), None) => handle_compare_by_pid(&pids, formatter, format, output),
        (None, Some(files)) => handle_compare_by_file(&files, formatter, format, output),
        _ => unreachable!("La validación previa garantiza que esto nunca ocurra"),
    }

    Ok(())
}

fn handle_compare_by_file(files: &[PathBuf], formatter: &OutputFormatter, format: &str, output: Option<&str>) {
    todo!()
}

fn handle_compare_by_pid(pids: &[u32], formatter: &OutputFormatter, format: &str, output: Option<&str>) {
    let mut system = sysinfo::System::new_all();
    system.refresh_processes();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL); // Espera breve
    system.refresh_processes(); // Segunda actualización

    let processes: Vec<_> = system.processes().values().filter(|p| pids.contains(&p.pid().as_u32())).collect();
    let rows: Vec<ProcessRow> = processes.iter().map(|p| process_to_row(p)).collect();
    let formatted_output = formatter.format_process_list(&rows, format);
    match formatted_output {
        Ok(output_str) => {
            if let Some(file_path) = output {
                if let Err(e) = std::fs::write(file_path, &output_str) {
                    eprintln!("Error al escribir archivo: {}", e);
                } else if format == "text" {
                    println!("✅ Resultados guardados en: {}", file_path);
                }
            } else {
                println!("{}", output_str);
            }
        }
        Err(e) => {
            eprintln!("Error al formatear la salida: {}", e);
        }
    }


}



