use serde::Serialize;
use crate::error::ExportError;
use argos_core::process::model::ProcessRow;


pub fn format_process_list(rows: &[ProcessRow], format: &str) -> Result<String, ExportError> {
    match format {
        "json" => crate::format_to_json(rows),
        "csv" => crate::format_to_csv(rows),
        "text" => Ok(crate::format_to_text(
            rows,
            |p: &ProcessRow| vec![
                p.pid.to_string(),
                p.name.clone(),
                format!("{:.2}", p.cpu_usage),
                format!("{:.2}", p.memory_mb),
                p.user.clone(),
                p.groups.clone(),
                p.state.clone(),
                p.start_time.to_string(),
                p.parent_pid.map_or("-".to_string(), |pp| pp.to_string()),
                format!("{:.2}", p.virtual_memory_mb),
                format!("{:.2}", p.read_disk_usage),
            ],
            &["PID", "Nombre", "CPU %", "RAM MB", "Usuario", "Grupos", "Estado", "Inicio", "Padre", "VMEM", "Lectura"]
        )),
        _ => Err(ExportError::UnsupportedFormat(format.to_string())),
    }
}

pub fn format_process_info(info: &ProcessRow, format: &str) -> Result<String, ExportError> {
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
