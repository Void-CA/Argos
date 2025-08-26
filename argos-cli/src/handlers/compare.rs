use argos_core::commands::compare::{by_file, by_pid};

pub fn handle_compare(pid: Option<i32>, file: Option<&str>, format: &str, output: Option<&str>) -> CliResult<()> {
    // Validar entrada
    if pid.is_none() && file.is_none() {
        return Err(CliError::invalid_input("Debe proporcionar un PID o una ruta de archivo para comparar."));
    }
    if pid.is_some() && file.is_some() {
        return Err(CliError::invalid_input("No puede proporcionar tanto un PID como una ruta de archivo. Elija uno."));
    }

    // Llamar al core
    let comparison = if let Some(pid) = pid {
        by_pid(pid).map_err(CliError::core_error)?
    } else if let Some(path) = file {
        by_file(path).map_err(CliError::core_error)?
    } else {
        unreachable!(); // Ya validamos que uno de los dos es Some
    };

    // Formatear salida
    let formatter = OutputFormatter::new();
    let output = formatter.format_comparison(&comparison, format)?;

    // Guardar en archivo o imprimir en stdout
    if let Some(path) = output {
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