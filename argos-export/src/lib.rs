// En argos-export/lib.rs

pub mod error;
pub mod process;
pub mod samples;
use argos_core::process::model::ProcessRow;
pub use error::ExportError;
pub use process::{format_process_list, format_process_info, format_process_tree, format_comparison};
pub use samples::format_samples_list;

use serde::Serialize;

pub fn format_to_json<T: ?Sized + Serialize>(value: &T) -> Result<String, ExportError> {
    serde_json::to_string_pretty(value).map_err(|e| ExportError::from(e))
}

pub fn format_to_csv<T: serde::Serialize>(data: &[T]) -> Result<String, ExportError> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    for item in data {
        wtr.serialize(item)?;
    }
    // get csv:Error before processing
    let data = wtr.into_inner()
        .map_err(|e| ExportError::Io(e.into_error()))?;
    Ok(String::from_utf8(data)?)
}

pub fn format_to_text<F>(
    rows: &[ProcessRow],
    row_mapper: F,
    headers: &[&str],
) -> String
where
    F: Fn(&ProcessRow) -> Vec<String>,
{
    // 1. Mapear filas a vectores de strings
    let mapped_rows: Vec<Vec<String>> = rows.iter().map(|p| row_mapper(p)).collect();

    // 2. Calcular el ancho máximo de cada columna (incluye headers)
    let mut col_widths: Vec<usize> = headers
        .iter()
        .map(|h| h.len())
        .collect();

    for row in &mapped_rows {
        for (i, cell) in row.iter().enumerate() {
            if cell.len() > col_widths[i] {
                col_widths[i] = cell.len();
            }
        }
    }

    // 3. Construir la tabla
    let mut output = String::new();

    // Encabezados
    for (i, header) in headers.iter().enumerate() {
        output.push_str(&format!("{:<width$} ", header, width = col_widths[i]));
    }
    output.push('\n');

    // Separador
    for width in &col_widths {
        output.push_str(&format!("{:-<width$} ", "-", width = *width));
    }
    output.push('\n');

    // Filas
    for row in mapped_rows {
        for (i, cell) in row.iter().enumerate() {
            output.push_str(&format!("{:<width$} ", cell, width = col_widths[i]));
        }
        output.push('\n');
    }

    output
}
