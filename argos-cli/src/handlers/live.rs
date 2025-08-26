use crate::{error::{CliError, CliResult}, output::OutputFormatter};
use argos_core::commands::live::monitor_live_by_pid;

pub fn handle_live(pid: u32) -> CliResult<()> {
    let formatter = OutputFormatter::new();

    // Closure que se ejecuta en cada iteración
    let callback = |process: &argos_core::process::model::ProcessRow| {
        match formatter.format_process_info(process, "text") {
            Ok(output) => println!("{}", output),
            Err(e) => eprintln!("Error al formatear: {}", e),
        }
    };

    // Llamar al core con el callback
    monitor_live_by_pid(pid, callback).map_err(CliError::core_error)
}