use std::fs;

use argos_core::commands::monitor::monitor_by_pids;

use crate::{error::{CliError, CliResult}, output::OutputFormatter};

pub fn handle_monitor(pid: u32, format: &str, save: bool) -> CliResult<()> {
    // Llamar al core
    let process = monitor_by_pids(&[pid]).map_err(CliError::core_error)?;

    // Formatear salida
    let formatter = OutputFormatter::new();
    let output = formatter.format_process_info(&process[0], format)?;

    // Guardar en archivo o imprimir en stdout
    if save {
        fs::write(format!("monitor_{}.txt", pid), &output)
            .map_err(|e| CliError::io_error(format!("Error al escribir archivo: {}", e)))?;
        println!("✅ Resultados guardados en: monitor_{}.txt", pid);
    } else {
        println!("{}", output);
    }

    Ok(())
}
