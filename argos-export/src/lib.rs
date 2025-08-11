// En argos-export/lib.rs

pub mod error;
pub mod process;
pub mod samples;

pub use error::ExportError;
pub use process::{format_process_list, format_process_info};
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

pub fn format_to_text<T>(
    data: &[T],
    extract: fn(&T) -> Vec<String>,
    headers: &[&str]
) -> String {
    let mut lines = vec![headers.join("\t")];
    for item in data {
        lines.push(extract(item).join("\t"));
    }
    lines.join("\n")
}