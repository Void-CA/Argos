use argos_core::process::model::ProcessRow;

pub fn format_samples_list(
    samples: &[ProcessRow],
    format: &str
) -> Result<String, crate::ExportError> {
    match format {
        "json" => crate::format_to_json(samples),
        "csv" => crate::format_to_csv(samples),
        "text" => Ok(crate::format_to_text(
            samples,
            |s| vec![s.start_time_human.clone(), s.cpu_usage.to_string(), s.memory_mb.to_string()],
            &["Timestamp", "CPU Usage", "Memory"]
        )),
        _ => Err(crate::ExportError::UnsupportedFormat(format.to_string())),
    }
}