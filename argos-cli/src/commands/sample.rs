use std::fs;
use argos_core::process_monitor::monitor_during_execution;
use argos_export::{IntoSampleRow, SampleRow};
use crate::output::OutputFormatter;
use crate::error::{CliError, CliResult};

pub fn handle_sample(
    formatter: &OutputFormatter,
    pid: u32,
    iterations: usize,
    interval_ms: u64,
    format: &str,
    output_file: Option<&str>,
) -> CliResult<()> {
    if format == "text" {
        println!(
            "🔍 Muestreo del PID {} por {} iteraciones ({} ms c/u):\n",
            pid, iterations, interval_ms
        );
    }

    let samples = monitor_during_execution(pid, iterations, interval_ms);
    if samples.is_empty() {
        return Err(CliError::process_not_found(pid));
    }
    let samples: Vec<SampleRow> = samples.into_iter().map(|s| IntoSampleRow::into(s)).collect();
    let output = formatter.format_samples(&samples, format)?;
    match output_file {
        Some(file_path) => {
            fs::write(file_path, &output)
                .map_err(|e| CliError::io_error(format!("Error al escribir archivo: {}", e)))?;
            if format == "text" {
                println!("✅ Resultados guardados en: {}", file_path);
            }
        }
        None => println!("{}", output),
    }
    Ok(())
}
