use std::collections::HashMap;

use crate::{error::ExportError, format_to_csv, format_to_json};
use ansi_term::Colour;
use argos_core::process::model::ProcessRow;


pub fn format_process_list(rows: &[ProcessRow], format: &str) -> Result<String, ExportError> {
    match format {
        "json" => crate::format_to_json(rows),
        "csv" => crate::format_to_csv(rows),
        "text" => Ok(crate::format_to_text(
            rows,
            |p: &ProcessRow| vec![
                p.pid.to_string(),
                p.name.clone(),
                format!("{:.2}", p.cpu_usage),
                format!("{:.2}", p.memory_mb),
                p.user.clone(),
                p.groups.clone(),
                p.state.clone(),
                p.start_time_human.clone(),
                p.parent_pid.map_or("-".to_string(), |pp| pp.to_string()),
                format!("{:.2}", p.virtual_memory_mb),
                format!("{:.2}", p.read_disk_usage),
                format!("{:.2}", p.write_disk_usage),
            ],
            &["PID", "Nombre", "CPU %", "RAM MB", "Usuario", "Grupos", "Estado", "Inicio", "Padre", "VMEM", "Lectura", "Escritura"]
        )),
        _ => Err(ExportError::UnsupportedFormat(format.to_string())),
    }
}

pub fn format_process_info(info: &ProcessRow, format: &str) -> Result<String, ExportError> {
    match format {
        "json" => crate::format_to_json(info),
        "csv" => crate::format_to_csv(&[info]),
        "text" => Ok(format!(
            "Información del Proceso\n\
            Nombre      : {:<30}\n\
            PID         : {:>6}\n\
            Estado      : {:<12}\n\
            CPU         : {:>6.2}%\n\
            RAM         : {:>8.2} MB\n\
            Inicio      : {:<20}\n\
            PID Padre   : {:>6}\n",
            info.name,
            info.pid,
            info.state,
            info.cpu_usage,
            info.memory_mb,
            info.start_time,
            info.parent_pid.map_or("N/A".to_string(), |p| p.to_string())
        )),
        _ => Err(ExportError::UnsupportedFormat(format.to_string())),
    }
}

pub fn format_process_tree(root: u32, rows: &[ProcessRow], format: &str) -> Result<String, ExportError> {
    match format {
        "json" => crate::format_to_json(rows),
        "csv" => crate::format_to_csv(rows),
        "text" => {
            let mut output = String::new();
            let mut map: HashMap<u32, Vec<&ProcessRow>> = HashMap::new();

            // Agrupar hijos por padre
            for p in rows {
                if let Some(ppid) = p.parent_pid {
                    map.entry(ppid).or_default().push(p);
                }
            }

            fn print_node(
                p: &ProcessRow,
                map: &HashMap<u32, Vec<&ProcessRow>>,
                indent: String,
                is_last: bool,
                output: &mut String,
            ) {
                // Símbolos de rama
                let branch = if indent.is_empty() {
                    String::new()
                } else if is_last {
                    format!("{}└─ ", indent)
                } else {
                    format!("{}├─ ", indent)
                };

                // Color según estado
                let name_colored = match p.state.as_str() {
                    "Zombie" => Colour::Red.paint(&p.name),
                    "Sleeping" => Colour::Yellow.paint(&p.name),
                    _ => Colour::Blue.paint(&p.name),
                };

                let line = format!(
                    "{}{} (PID {}, CPU {:.2}%, MEM {:.2} MB)\n",
                    branch, name_colored, p.pid, p.cpu_usage, p.memory_mb
                );
                output.push_str(&line);

                // Procesar hijos recursivamente
                if let Some(children) = map.get(&p.pid) {
                    let count = children.len();
                    for (i, child) in children.iter().enumerate() {
                        let new_indent = if indent.is_empty() {
                            String::new()
                        } else if is_last {
                            format!("{}   ", indent)
                        } else {
                            format!("{}│  ", indent)
                        };
                        print_node(child, map, new_indent, i == count - 1, output);
                    }
                }
            }

            // Buscar el proceso raíz por PID
            if let Some(root_process) = rows.iter().find(|p| p.pid == root) {
                print_node(root_process, &map, String::new(), true, &mut output);
            } else {
                output.push_str(&format!("No se encontró el proceso raíz con PID {}\n", root));
            }

            Ok(output)
        }
        _ => Err(ExportError::UnsupportedFormat(format.to_string())),
    }
}

pub fn format_comparison(
    comparison: &[argos_core::process::model::ProcessDelta],
    format: &str,
) -> Result<String, ExportError> {
    match format {
        "json" => format_to_json(comparison),
        "csv" => format_to_csv(comparison),
        "text" => {
            let mut output = String::new();
            output.push_str(&format!(
                "{:<6} {:<25} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}\n",
                "PID", "Name", "CPU Before", "CPU After", "CPU Δ",
                "Mem Before", "Mem After", "Mem Δ"
            ));
            output.push_str(&format!("{:-<6} {:-<25} {:-<10} {:-<10} {:-<10} {:-<10} {:-<10} {:-<10}\n",
                "", "", "", "", "", "", "", ""
            ));

            for delta in comparison {
                let cpu_delta = if delta.cpu_delta >= 0.0 {
                    Colour::Green.paint(format!("{:.2}", delta.cpu_delta))
                } else {
                    Colour::Red.paint(format!("{:.2}", delta.cpu_delta))
                };

                let mem_delta = if delta.mem_delta >= 0.0 {
                    Colour::Green.paint(format!("{:.2}", delta.mem_delta))
                } else {
                    Colour::Red.paint(format!("{:.2}", delta.mem_delta))
                };

                output.push_str(&format!(
                    "{:<6} {:<25} {:>10.2} {:>10.2} {:>10} {:>10.2} {:>10.2} {:>10}\n",
                    delta.pid,
                    delta.name,
                    delta.cpu_before,
                    delta.cpu_after,
                    cpu_delta,
                    delta.mem_before,
                    delta.mem_after,
                    mem_delta
                ));
            }

            Ok(output)
        }
        _ => Err(ExportError::UnsupportedFormat(format.to_string())),
    }
}