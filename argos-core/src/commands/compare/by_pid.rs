use crate::{
    errors::CoreError,
    process::reader::ProcessReader,
    process::model::{ProcessRow, ProcessDelta},
};
use std::time::Duration;

/// Obtiene un sample único de los procesos indicados
pub fn sample_process(pids: &[u32]) -> Result<Vec<ProcessRow>, CoreError> {
    if pids.is_empty() {
        return Err(CoreError::ComparisonError("No se proporcionaron PIDs".into()));
    }

    let mut reader = ProcessReader::new();
    let rows = reader.get_by_pids(pids);

    if rows.is_empty() {
        return Err(CoreError::ComparisonError("No se encontraron procesos con esos PIDs".into()));
    }

    Ok(rows)
}

/// Compara dos samples consecutivos de procesos
pub fn compare_samples(old: &[ProcessRow], new: &[ProcessRow]) -> Vec<ProcessDelta> {
    let mut diffs = Vec::new();

    for new_row in new {
        if let Some(old_row) = old.iter().find(|p| p.pid == new_row.pid) {
            diffs.push(ProcessDelta {
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
            // Proceso nuevo
            diffs.push(ProcessDelta {
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

    // Procesos que desaparecieron
    for old_row in old {
        if !new.iter().any(|p| p.pid == old_row.pid) {
            diffs.push(ProcessDelta {
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

    diffs
}

/// Ejemplo de uso en un loop de monitorización
pub fn compare_live(pids: &[u32], interval_ms: u64) -> Result<Vec<ProcessDelta>, CoreError> {
    let old_sample = sample_process(pids)?;
    std::thread::sleep(Duration::from_millis(interval_ms));
    let new_sample = sample_process(pids)?;
    Ok(compare_samples(&old_sample, &new_sample))
}
