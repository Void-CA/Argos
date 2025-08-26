pub fn handle_live(pid: i32) -> CliResult<()> {
    // Llamar al core
    let process = monitor_by_pid(pid).map_err(CliError::core_error)?;

    // Formatear salida
    let formatter = OutputFormatter::new();
    let output = formatter.format_process(&process, "text")?;

    // Imprimir en stdout
    println!("{}", output);

    Ok(())
}