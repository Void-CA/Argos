use crate::{
    errors::CoreError,
    process::reader::ProcessReader,
    process::model::ProcessRow
};

pub fn compare_by_pid(pids: &[u32]) -> Result<Vec<ProcessRow>, CoreError> {
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
