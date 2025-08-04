use crate::error::CliResult;
use crate::output::OutputFormatter;
use crate::config::Config;
use argos_core::process_monitor::monitor_process;
use argos_core::db::process::insert_process;
use argos_core::db::manager::establish_connection;
use crate::error::CliError;

pub fn handle_monitor(formatter: &OutputFormatter, config: &Config, pid: u32, format: &str, save: bool) -> CliResult<()> {
    let info = monitor_process(pid)
        .ok_or_else(|| CliError::process_not_found(pid))?;

    if save || config.auto_save {
        let mut conn = establish_connection();
        let process = info.clone().into();
        insert_process(&mut conn, &process)
            .map_err(|e| CliError::database_error(format!("Error al insertar en la base de datos: {}", e)))?;
        if format == "text" {
            println!("✅ Proceso guardado en la base de datos");
        }
    }

    let output = formatter.format_process_info(&info, format)?;
    println!("{}", output);
    Ok(())
}
