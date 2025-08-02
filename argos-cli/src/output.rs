use argos_core::process_monitor::types::ProcessInfo;
use argos_core::process_monitor::types::Sample;
use serde_json;
use crate::error::{CliResult, CliError, ErrorKind};

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format_process_info(&self, info: &ProcessInfo, format: &str) -> CliResult<String> {
        match format {
            "text" => Ok(self.format_process_text(info)),
            "json" => self.format_process_json(info),
            "csv" => Ok(self.format_process_csv(info)),
            _ => Err(CliError::format_error(format!("Formato no soportado: {}", format))),
        }
    }

    pub fn format_samples(&self, samples: &[Sample], format: &str) -> CliResult<String> {
        match format {
            "text" => Ok(self.format_samples_text(samples)),
            "json" => self.format_samples_json(samples),
            "csv" => Ok(self.format_samples_csv(samples)),
            _ => Err(CliError::format_error(format!("Formato no soportado: {}", format))),
        }
    }

    fn format_process_text(&self, info: &ProcessInfo) -> String {
        format!(
            "📊 Información del Proceso\n\
             ┌─────────────────────────────────────────┐\n\
             │ Nombre: {:<31} │\n\
             │ PID: {:<34} │\n\
             │ Estado: {:<30} │\n\
             │ CPU: {:<33.2}% │\n\
             │ RAM: {:<31.2} MB │\n\
             │ Inicio: {:<29} │\n\
             │ PID Padre: {:<26} │\n\
             └─────────────────────────────────────────┘",
            info.name,
            info.pid,
            info.state,
            info.cpu_usage,
            info.memory_mb,
            info.start_time,
            info.parent_pid.map_or("N/A".to_string(), |p| p.to_string()),
        )
    }

    fn format_process_json(&self, info: &ProcessInfo) -> CliResult<String> {
        let json_value = serde_json::json!({
            "name": info.name,
            "pid": info.pid,
            "state": info.state,
            "cpu_usage": info.cpu_usage,
            "memory_mb": info.memory_mb,
            "start_time": info.start_time,
            "parent_pid": info.parent_pid,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        serde_json::to_string_pretty(&json_value)
            .map_err(|e| CliError::format_error(format!("Error al serializar JSON: {}", e)))
    }

    fn format_process_csv(&self, info: &ProcessInfo) -> String {
        format!(
            "name,pid,state,cpu_usage,memory_mb,start_time,parent_pid\n\
             {},{},{},{:.2},{:.2},{},{}",
            info.name,
            info.pid,
            info.state,
            info.cpu_usage,
            info.memory_mb,
            info.start_time,
            info.parent_pid.unwrap_or(0)
        )
    }

    fn format_samples_text(&self, samples: &[Sample]) -> String {
        let mut output = String::new();
        output.push_str("⏱️  Timestamp │   Memoria   │   CPU   \n");
        output.push_str("──────────────┼─────────────┼─────────\n");
        
        for sample in samples {
            output.push_str(&format!(
                " {:>10.2}s │ {:>8} KB │ {:>6.2}%\n",
                sample.timestamp, sample.memory, sample.cpu_usage
            ));
        }
        
        // Agregar estadísticas resumen
        if !samples.is_empty() {
            let avg_cpu: f32 = samples.iter().map(|s| s.cpu_usage).sum::<f32>() / samples.len() as f32;
            let avg_mem: f32 = samples.iter().map(|s| s.memory as f32).sum::<f32>() / samples.len() as f32;
            let max_cpu = samples.iter().map(|s| s.cpu_usage).fold(0.0, f32::max);
            let max_mem = samples.iter().map(|s| s.memory).max().unwrap_or(0);

            output.push_str("──────────────┼─────────────┼─────────\n");
            output.push_str(&format!(
                " {:>10} │ {:>8.0} KB │ {:>6.2}%  (Promedio)\n",
                "Promedio", avg_mem, avg_cpu
            ));
            output.push_str(&format!(
                " {:>10} │ {:>8} KB │ {:>6.2}%  (Máximo)\n",
                "Máximo", max_mem, max_cpu
            ));
        }
        
        output
    }

    fn format_samples_json(&self, samples: &[Sample]) -> CliResult<String> {
        let json_value = serde_json::json!({
            "samples": samples,
            "summary": {
                "count": samples.len(),
                "avg_cpu": if !samples.is_empty() {
                    samples.iter().map(|s| s.cpu_usage).sum::<f32>() / samples.len() as f32
                } else { 0.0 },
                "avg_memory": if !samples.is_empty() {
                    samples.iter().map(|s| s.memory as f32).sum::<f32>() / samples.len() as f32
                } else { 0.0 },
                "max_cpu": samples.iter().map(|s| s.cpu_usage).fold(0.0, f32::max),
                "max_memory": samples.iter().map(|s| s.memory).max().unwrap_or(0)
            },
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        serde_json::to_string_pretty(&json_value)
            .map_err(|e| CliError::format_error(format!("Error al serializar JSON: {}", e)))
    }

    fn format_samples_csv(&self, samples: &[Sample]) -> String {
        let mut output = String::from("timestamp,memory_kb,cpu_usage\n");
        
        for sample in samples {
            output.push_str(&format!(
                "{:.2},{},{:.2}\n",
                sample.timestamp, sample.memory, sample.cpu_usage
            ));
        }
        
        output
    }
}
