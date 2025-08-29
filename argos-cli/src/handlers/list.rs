use argos_core::commands::list::list_processes;
use crate::error::{CliResult, CliError};
use crate::output::OutputFormatter;
use std::fs;

pub fn handle_list(
    format: &str,
    output_file: Option<&str>,
    name: Option<String>,
    user: Option<String>,
    top: Option<usize>,
    sort_by: String,
) -> CliResult<()> {
    // Llamar al core
    let mut rows = list_processes().map_err(|e| CliError::io_error(e.to_string()))?;

    // === Limitar con top ===
    if let Some(limit) = top {
        rows.truncate(limit);
    }
    
    // === Aplicar filtros ===
    if let Some(n) = name {
        rows.retain(|p| p.name.contains(&n));
    }
    if let Some(u) = user {
        rows.retain(|p| p.user.contains(&u));
    }

    // === Ordenamiento dinámico ===
    match sort_by.as_str() {
        "cpu" => rows.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap()),
        "ram" | "memory" => rows.sort_by(|a, b| b.memory_mb.partial_cmp(&a.memory_mb).unwrap()),
        "name" => rows.sort_by(|a, b| a.name.cmp(&b.name)),
        "user" => rows.sort_by(|a, b| a.user.cmp(&b.user)),
        _ => {} // si no matchea nada, no se ordena
    }

    

    // === Formatear salida ===
    let formatter = OutputFormatter::new();
    let output = formatter.format_process_list(&rows, format)?;

    // === Guardar o mostrar ===
    if let Some(path) = output_file {
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
