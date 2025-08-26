use crate::{
    errors::{CoreError, CoreResult},
    process::{model::ProcessRow, reader::ProcessReader},
};

/// Lista todos los procesos actuales usando ProcessReader
pub fn list_processes() -> CoreResult<Vec<ProcessRow>> {
    let reader = ProcessReader::new();
    let rows = reader.get_all();

    if rows.is_empty() {
        return Err(CoreError::ProcessNotFoundList(vec![]));
    }

    Ok(rows)
}
