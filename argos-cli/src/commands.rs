use argos_core::process_monitor::{monitor_process, monitor_during_execution};
use argos_core::db::process::insert_process;
use argos_core::db::manager::establish_connection;
use crate::cli::{Commands, ConfigAction};
use crate::output::OutputFormatter;
use crate::error::{CliResult, CliError};
use crate::config::Config;
use std::fs;

pub struct CommandHandler {
    formatter: OutputFormatter,
    config: Config,
}

impl CommandHandler {
    pub fn new() -> Self {
        let config = Config::load().unwrap_or_default();
        Self {
            formatter: OutputFormatter::new(),
            config,
        }
    }

    pub fn handle_command(&mut self, command: Commands) -> CliResult<()> {
        match command {
            Commands::Monitor { pid, format, save } => {
                self.handle_monitor(pid, &format, save)
            }
            Commands::Sample { pid, iterations, interval_ms, format, output } => {
                self.handle_sample(pid, iterations, interval_ms, &format, output.as_deref())
            }
            Commands::History { pid, limit, format } => {
                self.handle_history(pid, limit, &format)
            }
            Commands::List { name, user, sort_by, format } => {
                self.handle_list(name.as_deref(), user.as_deref(), &sort_by, &format)
            }
            Commands::Config { action } => {
                self.handle_config(action)
            }
        }
    }

    fn handle_monitor(&self, pid: u32, format: &str, save: bool) -> CliResult<()> {
        let info = monitor_process(pid)
            .ok_or_else(|| CliError::process_not_found(pid))?;

        if save || self.config.auto_save {
            let mut conn = establish_connection();
            let process = info.clone().into();
            insert_process(&mut conn, &process)
                .map_err(|e| CliError::database_error(format!("Error al insertar en la base de datos: {}", e)))?;
            
            if format == "text" {
                println!("✅ Proceso guardado en la base de datos");
            }
        }

        let output = self.formatter.format_process_info(&info, format)?;
        println!("{}", output);
        
        Ok(())
    }

    fn handle_sample(&self, pid: u32, iterations: usize, interval_ms: u64, format: &str, output_file: Option<&str>) -> CliResult<()> {
        if format == "text" {
            println!(
                "🔍 Muestreo del PID {} por {} iteraciones ({} ms c/u):\n",
                pid, iterations, interval_ms
            );
        }

        let samples = monitor_during_execution(pid, iterations, interval_ms);
        
        if samples.is_empty() {
            return Err(CliError::process_not_found(pid));
        }

        let output = self.formatter.format_samples(&samples, format)?;
        
        match output_file {
            Some(file_path) => {
                fs::write(file_path, &output)
                    .map_err(|e| CliError::io_error(format!("Error al escribir archivo: {}", e)))?;
                
                if format == "text" {
                    println!("✅ Resultados guardados en: {}", file_path);
                }
            }
            None => println!("{}", output),
        }
        
        Ok(())
    }

    fn handle_history(&self, _pid: Option<u32>, _limit: usize, format: &str) -> CliResult<()> {
        // TODO: Implementar consulta a la base de datos
        match format {
            "text" => println!("📦 Historial de procesos (por implementar)"),
            "json" => println!("{{\"message\": \"Historial por implementar\"}}"),
            "csv" => println!("message\nHistorial por implementar"),
            _ => return Err(CliError::format_error(format!("Formato no soportado: {}", format))),
        }
        Ok(())
    }

    fn handle_list(&self, _name_filter: Option<&str>, _user_filter: Option<&str>, _sort_by: &str, format: &str) -> CliResult<()> {
        // TODO: Implementar listado de procesos
        match format {
            "text" => println!("📋 Lista de procesos (por implementar)"),
            "json" => println!("{{\"message\": \"Lista por implementar\"}}"),
            "csv" => println!("message\nLista por implementar"),
            _ => return Err(CliError::format_error(format!("Formato no soportado: {}", format))),
        }
        Ok(())
    }

    fn handle_config(&mut self, action: ConfigAction) -> CliResult<()> {
        match action {
            ConfigAction::Show => {
                println!("{}", self.config.display());
            }
            ConfigAction::Set { key, value } => {
                self.config.set_value(&key, &value)?;
                self.config.save()?;
                println!("⚙️  Configuración actualizada: {} = {}", key, value);
            }
            ConfigAction::Reset => {
                self.config = Config::default();
                self.config.save()?;
                println!("⚙️  Configuración reseteada a valores por defecto");
            }
        }
        Ok(())
    }
}
