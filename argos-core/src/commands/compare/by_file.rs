use std::path::PathBuf;
use crate::process::model::{ProcessRow, ProcessDelta};
use crate::errors::CoreError;

pub fn compare_by_file(files: &[PathBuf]) -> Result<Vec<ProcessDelta>, CoreError> {
    if files.len() < 2 {
        return Err(CoreError::ComparisonError("Se necesitan al menos dos archivos para comparar".into()));
    }

    // Leer todos los snapshots
    let mut snapshots: Vec<Vec<ProcessRow>> = Vec::new();
    for file in files {
        let data = std::fs::read_to_string(file).map_err(CoreError::Io)?;
        let file_rows: Vec<ProcessRow> = serde_json::from_str(&data).map_err(CoreError::Parse)?;
        snapshots.push(file_rows);
    }

    // Comparar los dos últimos snapshots
    let old = &snapshots[snapshots.len() - 2];
    let new = &snapshots[snapshots.len() - 1];

    let mut deltas: Vec<ProcessDelta> = Vec::new();

    for new_row in new {
        if let Some(old_row) = old.iter().find(|p| p.pid == new_row.pid) {
            deltas.push(ProcessDelta {
                pid: new_row.pid,
                name: new_row.name.clone(),
                cpu_before: old_row.cpu_usage,
                cpu_after: new_row.cpu_usage,
                cpu_delta: (new_row.cpu_usage - old_row.cpu_usage),
                mem_before: old_row.memory_mb,
                mem_after: new_row.memory_mb,
                mem_delta: (new_row.memory_mb - old_row.memory_mb),
            });
        } else {
            // proceso nuevo
            deltas.push(ProcessDelta {
                pid: new_row.pid,
                name: new_row.name.clone(),
                cpu_before: 0.0,
                cpu_after: new_row.cpu_usage,
                cpu_delta: new_row.cpu_usage,
                mem_before: 0.0,
                mem_after: new_row.memory_mb,
                mem_delta: new_row.memory_mb,
            });
        }
    }

    // procesos que desaparecieron
    for old_row in old {
        if !new.iter().any(|p| p.pid == old_row.pid) {
            deltas.push(ProcessDelta {
                pid: old_row.pid,
                name: old_row.name.clone(),
                cpu_before: old_row.cpu_usage,
                cpu_after: 0.0,
                cpu_delta: -(old_row.cpu_usage),
                mem_before: old_row.memory_mb,
                mem_after: 0.0,
                mem_delta: -(old_row.memory_mb),
            });
        }
    }

    Ok(deltas)
}
