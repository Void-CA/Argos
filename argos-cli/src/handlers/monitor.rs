use argos_core::commands::monitor::monitor_by_pid;
pub fn handle_monitor(pid: i32, format: &str, save: bool) -> CliResult<()> {
    // Llamar al core
    let process = monitor_by_pid(pid).map_err(CliError::core_error)?;

    // Formatear salida
    let formatter = OutputFormatter::new();
    let output = formatter.format_process(&process, format)?;

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
