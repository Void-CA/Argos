use std::{fs, time::Duration, path::PathBuf};
use std::thread::sleep;

use argos_core::commands::compare::{
    by_file::compare_by_file,
    by_pid::{sample_process, compare_samples},
};
use crate::{error::{CliError, CliResult}, output::OutputFormatter};

pub fn handle_compare(
    pids: Option<Vec<u32>>,
    files: Option<Vec<PathBuf>>,
    format: &str,
    output: Option<&str>,
    interval_ms: u64,
) -> CliResult<()> {
    // Validar entrada
    if pids.is_none() && files.is_none() {
        return Err(CliError::io_error(
            "Debe proporcionar un PID o una ruta de archivo para comparar.",
        ));
    }
    if pids.is_some() && files.is_some() {
        return Err(CliError::io_error(
            "No puede proporcionar tanto un PID como una ruta de archivo. Elija uno.",
        ));
    }

    // Obtener los datos
    let comparison = if let Some(pids) = pids {
        // Sampleo en vivo: dos muestras consecutivas para todos los PIDs
        let old_sample = sample_process(&pids).map_err(CliError::core_error)?;
        sleep(Duration::from_millis(interval_ms));
        let new_sample = sample_process(&pids).map_err(CliError::core_error)?;
        compare_samples(&old_sample, &new_sample)
    } else if let Some(files) = files {
        compare_by_file(&files).map_err(CliError::core_error)?
    } else {
        unreachable!(); // Ya validamos que uno de los dos es Some
    };

    // Formatear salida
    let formatter = OutputFormatter::new();
    let formatted_output = formatter.format_comparison(&comparison, format)?;

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
