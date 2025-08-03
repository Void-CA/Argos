use argos_core::process_monitor::Sample;
use serde::Serialize;

#[derive(Serialize)]
pub struct SampleRow {
    pub timestamp: String,
    pub cpu_usage: f32,
    pub memory: u64,
    // ...otros campos...
}

pub fn format_samples_list(
    samples: &[SampleRow],
    format: &str
) -> Result<String, crate::ExportError> {
    match format {
        "json" => crate::format_to_json(samples),
        "csv" => crate::format_to_csv(samples),
        "text" => Ok(crate::format_to_text(
            samples,
            |s| vec![s.timestamp.clone(), s.cpu_usage.to_string(), s.memory.to_string()],
            &["Timestamp", "CPU Usage", "Memory"]
        )),
        _ => Err(crate::ExportError::UnsupportedFormat(format.to_string())),
    }
}

pub trait IntoSampleRow {
    fn into(self) -> SampleRow;
}

impl IntoSampleRow for Sample {
    fn into(self) -> SampleRow {
        SampleRow {
            timestamp: self.timestamp.to_string(),
            cpu_usage: self.cpu_usage,
            memory: self.memory,
        }
    }
}