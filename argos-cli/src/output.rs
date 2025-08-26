use argos_core::process::model::ProcessRow;
use argos_export::{self};
use crate::error::{CliResult, CliError};

#[derive(Debug)]
pub struct OutputFormatter;

impl OutputFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format_process_list(&self, rows: &[ProcessRow], format: &str) -> CliResult<String> {
        argos_export::format_process_list(rows, format)
            .map_err(|e| CliError::format_error(format!("Error al exportar procesos: {}", e)))
    }

    pub fn format_process_info(&self, info: &ProcessRow, format: &str) -> CliResult<String> {
        argos_export::format_process_info(info, format)
            .map_err(|e| CliError::format_error(format!("Error al exportar proceso: {}", e)))
    }

    pub fn format_samples(&self, samples: &[ProcessRow], format: &str) -> CliResult<String> {
        argos_export::format_samples_list(samples, format)
            .map_err(|e| CliError::format_error(format!("Error al exportar muestras: {}", e)))
    }
    
    pub(crate) fn format_comparison(&self, comparison: &[ProcessRow], format: &str) -> CliResult<String> {
        argos_export::format_comparison(comparison, format)
            .map_err(|e| CliError::format_error(format!("Error al exportar comparación: {}", e)))
    }
}
