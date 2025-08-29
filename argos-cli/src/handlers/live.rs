use crate::{error::{CliError, CliResult}, output::OutputFormatter};
use argos_core::commands::live::monitor_live_by_pid;
use std::fs::OpenOptions;
use std::io::Write;

pub fn handle_live(pid: u32, output_file: Option<&str>, format: Option<&str>) -> CliResult<()> {
    let formatter = OutputFormatter::new();
    let format = format.unwrap_or("text");

    // Si se pasa un archivo, lo abrimos en modo append
    let mut file = if let Some(path) = output_file {
        Some(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| CliError::io_error(format!("No se pudo abrir {}: {}", path, e)))?
        )
    } else {
        None
    };

    // Closure que se ejecuta en cada iteración
    let mut callback = |process: &argos_core::process::model::ProcessRow| {
        match formatter.format_process_info(process, format) {
            Ok(mut output) => {
                // Elimina saltos de línea al final
                while output.ends_with('\n') || output.ends_with(' ') {
                    output.pop();
                }
                if file.is_none() {
                    println!("{}", output);
                }
                if let Some(file) = file.as_mut() {
                    if let Err(e) = writeln!(file, "{}", output) {
                        eprintln!("Error al escribir en archivo: {}", e);
                    }
                }
            }
            Err(e) => eprintln!("Error al formatear: {}", e),
        }
    };

    // Llamar al core con el callback
    monitor_live_by_pid(pid, &mut callback).map_err(CliError::core_error)
}
