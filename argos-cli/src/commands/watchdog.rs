use crate::{error::CliResult, output::OutputFormatter};

pub fn handle_watchdog(formatter: &OutputFormatter, pid: u32, cpu_over: u8, memory_over: u8, on_exceed: Option<String>, interval: u64) -> CliResult<()> {
    // Implementar la lógica para el watchdog
    Ok(())
}