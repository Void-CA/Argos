use crate::{error::{CliError, CliResult}, output::OutputFormatter};


pub fn handle_history(formatter: &OutputFormatter, _pid: Option<u32>, _limit: usize, format: &str) -> CliResult<()> {
        // TODO: Implementar consulta a la base de datos
        match format {
            "text" => println!("📦 Historial de procesos (por implementar)"),
            "json" => println!("{{\"message\": \"Historial por implementar\"}}"),
            "csv" => println!("message\nHistorial por implementar"),
            _ => return Err(CliError::format_error(format!("Formato no soportado: {}", format))),
        }
        Ok(())
    }