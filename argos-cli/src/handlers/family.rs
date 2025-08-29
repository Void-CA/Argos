use argos_core::commands::family::get_family;
use crate::error::{CliError, CliResult};
use crate::output::OutputFormatter;

pub fn handle_family(pid: u32, format: &str) -> CliResult<()> {
    let family = get_family(pid).map_err(|e| CliError::io_error(e.to_string()))?;
    let formatter = OutputFormatter::new();
    let output = formatter.format_process_tree(pid, &family, format)?;
    
    if family.is_empty() {
        println!("No se encontraron procesos relacionados con el PID {}", pid);
    }

    println!("{}", output); // <- mostrar el resultado
    Ok(())
}
