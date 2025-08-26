use crate::{
    errors::{CoreError, CoreResult},
    process::{model::ProcessRow, reader::ProcessReader},
};


/// Obtiene información de procesos filtrados por PID
pub fn monitor_by_pids(pids: &[u32]) -> CoreResult<Vec<ProcessRow>> {
    if pids.is_empty() {
        return Err(CoreError::ValidationError("No se proporcionaron PIDs".into()));
    }

    let mut reader = ProcessReader::new();
    let rows = reader.get_by_pids(pids);

    if rows.is_empty() {
        return Err(CoreError::ProcessNotFoundList(pids.to_vec()));
    }

    Ok(rows)
}
