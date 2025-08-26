use std::time::{Duration, Instant};
use crate::commands::types::{Condition, Action};
use crate::errors::CoreError;
use sysinfo::System;

pub struct WatchdogReport {
    pub pid: u32,
    pub triggered: Vec<(Condition, Action)>, // historial de disparos
    pub duration: Duration,                  // cuánto tiempo estuvo corriendo
}

pub fn watchdog(
    pid: u32,
    interval: Duration,
    conditions: Vec<Condition>,
    actions: Vec<Action>,
) -> Result<WatchdogReport, CoreError> {
    let mut system = System::new_all();
    let start = Instant::now();

    let mut triggered_log: Vec<(Condition, Action)> = Vec::new();

    loop {
        system.refresh_process(sysinfo::Pid::from(pid as usize));

        if let Some(process) = system.process(sysinfo::Pid::from(pid as usize)) {
            let cpu_usage = process.cpu_usage();
            let memory = process.memory();

            for condition in &conditions {
                if condition.is_triggered(cpu_usage, memory) {
                    for action in &actions {
                        // Ejecutar acción
                        action.execute(process).map_err(|e| CoreError::WatchdogError(format!("Action execution failed: {}", e)))?;

                        // Guardar en historial
                        triggered_log.push((condition.clone(), action.clone()));
                    }
                }
            }
        } else {
            println!("Proceso {} no encontrado", pid);
            break; // termina el watchdog
        }

        std::thread::sleep(interval);
    }

    Ok(WatchdogReport {
        pid,
        triggered: triggered_log,
        duration: start.elapsed(),
    })
}
