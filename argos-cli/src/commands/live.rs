use argos_core::process_monitor::monitor_live;
use argos_export::IntoSampleRow;

use crate::{error::{CliError, CliResult}, output::OutputFormatter};

pub fn handle_live(formatter: &OutputFormatter, pid: u32) -> CliResult<()> {

    monitor_live(pid, 200, |sample| {
        let row = IntoSampleRow::into(sample);
        let output = formatter.format_samples(std::slice::from_ref(&row), "csv").unwrap();
        print!("\x1B[2J\x1B[1;1H"); // clear screen
        println!("{}", output);
    }).map_err(|e| CliError::io_error(format!("Error al iniciar monitorización en vivo: {:?}", e)))?;

    Ok(())
}