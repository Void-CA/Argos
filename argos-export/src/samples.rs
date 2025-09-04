use argos_core::process::model::ProcessRow;
use crate::ExportError;

pub fn format_samples_list(samples: &[ProcessRow], format: &str) -> Result<String, ExportError> {
    match format {
        "json" => crate::format_to_json(samples),
        "csv" => crate::format_to_csv(samples),
        "text" => {
            use std::fmt::Write;
            let mut output = String::new();

            // Column widths calculadas dinámicamente
            let ts_width = "Timestamp".len().max(samples.iter().map(|s| s.start_time_human.len()).max().unwrap_or(0));
            let pid_width = "PID".len().max(samples.iter().map(|s| s.pid.to_string().len()).max().unwrap_or(0));
            let name_width = "Nombre".len().max(samples.iter().map(|s| s.name.len()).max().unwrap_or(0));
            let cpu_width = "CPU %".len().max(samples.iter().map(|s| format!("{:.2}", s.cpu_usage).len()).max().unwrap_or(0));
            let mem_width = "RAM MB".len().max(samples.iter().map(|s| format!("{:.2}", s.memory_mb).len()).max().unwrap_or(0));
            let state_width = "Estado".len().max(samples.iter().map(|s| s.state.len()).max().unwrap_or(0));

            // Header
            let ts_header = format!("{:<width$}", "Timestamp", width = ts_width);
            let pid_header = format!("{:>width$}", "PID", width = pid_width);
            let name_header = format!("{:<width$}", "Nombre", width = name_width);
            let cpu_header = format!("{:>width$}", "CPU %", width = cpu_width);
            let mem_header = format!("{:>width$}", "RAM MB", width = mem_width);
            let state_header = format!("{:<width$}", "Estado", width = state_width);

        writeln!(
            &mut output,
            "{} {} {} {} {} {}",
            ts_header, pid_header, name_header, cpu_header, mem_header, state_header
        ).unwrap();


            writeln!(
                &mut output,
                "{:-<ts$} {:-<pid$} {:-<name$} {:-<cpu$} {:-<mem$} {:-<state$}",
                "", "", "", "", "", "",
                ts = ts_width, pid = pid_width, name = name_width, cpu = cpu_width, mem = mem_width, state = state_width
            ).unwrap();

            // Rows
            for s in samples {
                let ts_col = format!("{:<width$}", s.start_time_human, width = ts_width);
                let pid_col = format!("{:>width$}", s.pid, width = pid_width);
                let name_col = format!("{:<width$}", s.name, width = name_width);
                let cpu_col = format!("{:>width$.2}", s.cpu_usage, width = cpu_width);
                let mem_col = format!("{:>width$.2}", s.memory_mb, width = mem_width);
                let state_col = format!("{:<width$}", s.state, width = state_width);

                writeln!(
                    &mut output,
                    "{} {} {} {} {} {}",
                    ts_col, pid_col, name_col, cpu_col, mem_col, state_col
                ).unwrap();
            }

            // Estadísticas
            if !samples.is_empty() {
                let cpu_values: Vec<f64> = samples.iter().map(|s| s.cpu_usage as f64).collect();
                let mem_values: Vec<f64> = samples.iter().map(|s| s.memory_mb as f64).collect();

                let avg = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
                let std = |v: &[f64], mean: f64| (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64).sqrt();

                let cpu_avg = avg(&cpu_values);
                let cpu_std = std(&cpu_values, cpu_avg);
                let cpu_min = cpu_values.iter().cloned().fold(f64::INFINITY, f64::min);
                let cpu_max = cpu_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

                let mem_avg = avg(&mem_values);
                let mem_std = std(&mem_values, mem_avg);
                let mem_min = mem_values.iter().cloned().fold(f64::INFINITY, f64::min);
                let mem_max = mem_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

                writeln!(&mut output, "\nEstadísticas:").unwrap();
                writeln!(&mut output, "CPU % -> Promedio: {:.2}, Min: {:.2}, Max: {:.2}, Desv.Est: {:.2}", cpu_avg, cpu_min, cpu_max, cpu_std).unwrap();
                writeln!(&mut output, "RAM MB -> Promedio: {:.2}, Min: {:.2}, Max: {:.2}, Desv.Est: {:.2}", mem_avg, mem_min, mem_max, mem_std).unwrap();
            }

            Ok(output)
        }
        _ => Err(ExportError::UnsupportedFormat(format.to_string())),
    }
}