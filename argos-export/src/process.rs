use crate::error::ExportError;
use argos_core::utils::process::{ProcessRow};


pub fn format_process_list(rows: &[ProcessRow], format: &str) -> Result<String, ExportError> {
    match format {
        "text" => {
            let headers = [
                "PID", "Nombre", "CPU %", "RAM MB", "Usuario", "Grupos", "Estado", "Inicio", "Padre", "VMEM MB", "Lectura KB", "Escritura KB"
            ];
            // Construir matriz de strings
            let data: Vec<Vec<String>> = rows.iter().map(|p| vec![
                p.pid.to_string(),
                p.name.clone(),
                format!("{:.2}", p.cpu_usage),
                format!("{:.2}", p.memory_mb),
                p.user.clone(),
                p.groups.clone(),
                p.state.clone(),
                p.start_time_human.clone(),
                p.parent_pid.map_or("-".to_string(), |pp| pp.to_string()),
                format!("{:.2}", p.virtual_memory_mb),
                format!("{:.2}", p.read_disk_usage),
                format!("{:.2}", p.write_disk_usage),
            ]).collect();

            // Calcular anchos máximos
            let mut col_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
            for row in &data {
                for (i, cell) in row.iter().enumerate() {
                    if cell.len() > col_widths[i] {
                        col_widths[i] = cell.len();
                    }
                }
            }

            // Formatear header
            let mut out = String::new();
            for (i, h) in headers.iter().enumerate() {
                out.push_str(&format!("{:<width$} ", h, width = col_widths[i]));
            }
            out.push('\n');

            // Formatear filas
            for row in &data {
                for (i, cell) in row.iter().enumerate() {
                    out.push_str(&format!("{:<width$} ", cell, width = col_widths[i]));
                }
                out.push('\n');
            }
            Ok(out)
        }
        "json" => crate::format_to_json(rows),
        "csv" => crate::format_to_csv(rows),
        _ => Err(ExportError::UnsupportedFormat(format.to_string())),
    }
}

// Opcional: función para formatear un solo proceso (puede usarse para detalles)
use argos_core::process_monitor::types::ProcessInfo;
pub fn format_process_info(info: &ProcessInfo, format: &str) -> Result<String, ExportError> {
    match format {
        "json" => crate::format_to_json(info),
        "csv" => crate::format_to_csv(&[info]),
        "text" => Ok(format!(
            "📊 Información del Proceso\n\
             ┌─────────────────────────────────────────┐\n\
             │ Nombre: {:<31} │\n\
             │ PID: {:<34} │\n\
             │ Estado: {:<30} │\n\
             │ CPU: {:<33.2}% │\n\
             │ RAM: {:<31.2} MB │\n\
             │ Inicio: {:<29} │\n\
             │ PID Padre: {:<26} │\n\
             └─────────────────────────────────────────┘",
            info.name,
            info.pid,
            info.state,
            info.cpu_usage,
            info.memory_mb,
            info.start_time,
            info.parent_pid.map_or("N/A".to_string(), |p| p.to_string()),
        )),
        _ => Err(ExportError::UnsupportedFormat(format.to_string())),
    }
}
