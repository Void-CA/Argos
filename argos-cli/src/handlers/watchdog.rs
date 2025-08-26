use argos_core::commands::watchdog::watchdog;
use argos_core::commands::types::{Condition, Action};
use crate::error::{CliError, CliResult};
use std::time::Duration;

pub fn handle_watchdog(
    pid: u32,
    cpu_over: Option<f32>,
    memory_over: Option<u64>,
    on_exceed: Option<String>, // <- Opción CLI
    interval: u64,
) -> CliResult<()> {
    let action: Option<Action> = on_exceed
        .map(|s| s.parse())
        .transpose()
        .map_err(|_| CliError::io_error("Invalid action".to_string()))?;

    let mut conditions = Vec::new();
    if let Some(cpu) = cpu_over {
        conditions.push(Condition::CpuAbove(cpu));
    }
    if let Some(mem) = memory_over {
        conditions.push(Condition::MemAbove(mem));
    }

    let duration = Duration::from_secs(interval);

    let report = watchdog(
        pid,
        duration,
        conditions,
        action.into_iter().collect(), // convierte Option<Action> a Vec<Action>
    ).map_err(CliError::core_error)?;

    println!("Watchdog finished for PID {} after {:?}", report.pid, report.duration);
    for (cond, act) in report.triggered {
        println!("Triggered {:?} -> {:?}", cond, act);
    }

    Ok(())
}

