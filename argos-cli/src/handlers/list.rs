use argos_core::commands::list::list_processes;
use crate::error::{CliResult, CliError};
use crate::output::OutputFormatter;
use std::fs;

pub fn handle_list(format: &str, output_file: Option<&str>) -> CliResult<()> {
    // Llamar al core
    let rows = list_processes().map_err(CliError::core_error)?;

    // Formatear salida
    let formatter = OutputFormatter::new();
    let output = formatter.format_process_list(&rows, format)?;

    // Guardar en archivo o imprimir en stdout
    if let Some(path) = output_file {
        fs::write(path, &output)
            .map_err(|e| CliError::io_error(format!("Error al escribir archivo: {}", e)))?;
        if format == "text" {
            println!("✅ Resultados guardados en: {}", path);
        }
    } else {
        println!("{}", output);
    }

    Ok(())
}
