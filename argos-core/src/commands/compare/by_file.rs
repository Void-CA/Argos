use std::path::PathBuf;
use crate::process::model::ProcessRow;
use crate::errors::CoreError;

pub fn compare_by_file(files: &[PathBuf]) -> Result<Vec<ProcessRow>, CoreError> {
    let mut rows = Vec::new();
    for file in files {
        let data = std::fs::read_to_string(file)
            .map_err(|e| CoreError::Io(e))?;
        // Aquí parsea el archivo según formato (ejemplo: JSON)
        let mut file_rows: Vec<ProcessRow> = serde_json::from_str(&data)
            .map_err(|e| CoreError::Parse(e))?;
        rows.append(&mut file_rows);
    }
    Ok(rows)
}