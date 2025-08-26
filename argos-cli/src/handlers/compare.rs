use std::fs;

use argos_core::commands::compare::{by_file::compare_by_file, by_pid::compare_by_pid};

use crate::{error::{CliError, CliResult}, output::OutputFormatter};

pub fn handle_compare(pid: Option<u32>, file: Option<&str>, format: &str, output: Option<&str>) -> CliResult<()> {
    // Validar entrada
    if pid.is_none() && file.is_none() {
        return Err(CliError::io_error("Debe proporcionar un PID o una ruta de archivo para comparar."));
    }
    if pid.is_some() && file.is_some() {
        return Err(CliError::io_error("No puede proporcionar tanto un PID como una ruta de archivo. Elija uno."));
    }

    // Llamar al core
    let comparison = if let Some(pid) = pid {
        compare_by_pid(&[pid]).map_err(CliError::core_error)?
    } else if let Some(path) = file {
        use std::path::PathBuf;
        let path_buf = PathBuf::from(path);
        compare_by_file(&[path_buf]).map_err(CliError::core_error)?
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