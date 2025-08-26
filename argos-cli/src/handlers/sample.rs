use std::fs;

use argos_core::commands::sampling::sample_process;

use crate::{error::{CliError, CliResult}, output::OutputFormatter};

pub fn handle_sample(pid: u32, iterations: u32, interval_ms: u64, format: &str, output: Option<&str>) -> CliResult<()> {
    // Llamar al core
    let samples = sample_process(pid, iterations as usize, interval_ms as u64).map_err(CliError::core_error)?;

    // Formatear salida
    let formatter = OutputFormatter::new();
    let formatted_output = formatter.format_samples(&samples, format)?;

    // Guardar en archivo o imprimir en stdout
    if let Some(path) = output {
        fs::write(path, &formatted_output)
            .map_err(|e| CliError::io_error(format!("Error al escribir archivo: {}", e)))?;
        if format == "text" {
            println!("✅ Resultados guardados en: {}", path);
        }
    } else {
        println!("{}", formatted_output);
    }

    Ok(())
}