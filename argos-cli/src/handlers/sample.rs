use argos_core::commands::sampling::sample_process;

pub fn sample_handler(pid: i32, iterations: u32, interval_ms: u32, format: &str, output: Option<&str>) -> CliResult<()> {
    // Llamar al core
    let samples = sample_process(pid, iterations, interval_ms).map_err(CliError::core_error)?;

    // Formatear salida
    let formatter = OutputFormatter::new();
    let output = formatter.format_process_samples(&samples, format)?;

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