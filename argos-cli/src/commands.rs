use argos_core::process_monitor::{monitor_process, monitor_during_execution};
use argos_core::users::utils::get_user_by_id;
use argos_core::db::process::insert_process;
use argos_core::db::manager::establish_connection;
use crate::cli::{Commands, ConfigAction};
use crate::output::OutputFormatter;
use argos_export::{ProcessRow, SampleRow, IntoSampleRow};
use crate::error::{CliResult, CliError};
use crate::config::Config;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use ctrlc;

#[derive(Debug)]


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
            Commands::List { name, user, sort_by, format , output} => {
                self.handle_list(name.as_deref(), user.as_deref(), &sort_by, &format, output.as_deref())
            }
            Commands::Live {pid} => {
                self.handle_live(pid)
            }
            Commands::Compare {pids, files, format, output} => {
                self.handle_compare(pids, files, &format, output.as_deref())
            }
            Commands::Watchdog { pid, cpu_over, memory_over, on_exceed, interval } => {
                self.handle_watchdog(pid, cpu_over, memory_over, on_exceed, interval)
            }
            Commands::Tag { name, pid } => {
                self.handle_tag(&name, pid)
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

        // Convert Vec<Sample> to Vec<SampleRow>
        let samples = samples.into_iter().map(|s| IntoSampleRow::into(s))
            .collect::<Vec<SampleRow>>();
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

    fn handle_list(
        &self,
        name_filter: Option<&str>,
        user_filter: Option<&str>,
        sort_by: &str,
        format: &str,
        output_file: Option<&str>,
    ) -> CliResult<()> {
        let mut system = sysinfo::System::new_all();
        system.refresh_processes();

        let mut processes: Vec<_> = system.processes().values().collect();

        // Filtrar por nombre
        if let Some(name) = name_filter {
            processes.retain(|p| p.name().contains(name));
        }

        // Filtrar por usuario
        if let Some(user) = user_filter {
            processes.retain(|p| {
                p.user_id()
                    .and_then(|uid| get_user_by_id(uid.clone()))
                    .map(|u| u.name.contains(user))
                    .unwrap_or(false)
            });
        }

        // Ordenar
        match sort_by {
            "cpu" => processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap()),
            "memory" => processes.sort_by(|a, b| b.memory().cmp(&a.memory())),
            "name" => processes.sort_by(|a, b| a.name().cmp(b.name())),
            "pid" => processes.sort_by(|a, b| a.pid().cmp(&b.pid())),
            _ => {}
        }

        // Conversión a ProcessRow de argos_export
        let rows: Vec<ProcessRow> = processes.iter().map(|p| process_to_row(p)).collect();

        let output = self.formatter.format_process_list(&rows, format)?;
        if let Some(file_path) = output_file {
            fs::write(file_path, &output)
                .map_err(|e| CliError::io_error(format!("Error al escribir archivo: {}", e)))?;
            if format == "text" {
                println!("✅ Resultados guardados en: {}", file_path);
            }
        } else {
            println!("{}", output);
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

fn handle_live(&self, pid: u32) -> CliResult<()> {
    use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

    // Bandera para controlar la interrupción del loop
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Ctrl+C para salir del monitoreo
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("No se pudo establecer el handler de Ctrl+C");

    while running.load(Ordering::SeqCst) {
        sysinfo::System::new_all().refresh_processes();
        // Refresca la información del proceso
        let info = match monitor_process(pid) {
            Some(i) => i,
            None => {
                eprintln!("Proceso con PID {} no encontrado o finalizado.", pid);
                break;
            }
        };

        // Limpia la terminal para una vista en vivo
        print!("\x1B[2J\x1B[1;1H"); // ANSI escape codes para limpiar
        io::stdout().flush().unwrap();

        let output = self.formatter.format_process_info(&info, "csv")?;
        println!("{}", output);

        thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}

fn handle_compare(&self, pids: Option<Vec<u32>>, files: Option<Vec<PathBuf>>, format: &str, output: Option<&str>) -> CliResult<()> {
    if pids.is_none() && files.is_none() {
        return Err(CliError::new(
            crate::error::ErrorKind::ValidationError,
            "Debe proporcionar al menos un PID o un archivo para comparar",
        ));
    }

    Ok(())
}
fn handle_watchdog(&self, pid: u32, cpu_over: u8, memory_over: u8, on_exceed: Option<String>, interval: u64) -> CliResult<()> {
    // Implementar la lógica para el watchdog
    Ok(())
}

fn handle_tag(&self, name: &str, pid: u32) -> CliResult<()> {
    // Implementar la lógica para etiquetar procesos
    // Por ahora, solo imprimimos un mensaje
    println!("Etiqueta '{}' aplicada al proceso con PID {}", name, pid);
    
    // Aquí podrías guardar la etiqueta en una base de datos o archivo si es necesario
    Ok(())
}
}

// Conversión de sysinfo::Process a argos_export::ProcessRow
fn process_to_row(p: &sysinfo::Process) -> ProcessRow {
    use argos_core::users::utils::get_user_by_id;
    let myuser = p.user_id().and_then(|uid| get_user_by_id(uid.clone()));
    let user_name = myuser.as_ref().map(|u| u.name.as_str()).unwrap_or("-").to_string();
    let groups = myuser.as_ref().map(|u| u.groups.join(",")).unwrap_or_else(|| "-".to_string());
    let state = format!("{:?}", p.status());
    let exe = p.exe().map(|path| path.display().to_string()).unwrap_or_else(|| "-".to_string());
    let cmd = p.cmd().join(" ");
    let start_time = p.start_time();
    let parent_pid = p.parent().map(|pp| pp.as_u32());
    let virtual_memory_mb = p.virtual_memory() as f64 / 1024.0;
    let read_disk_usage = p.disk_usage().total_read_bytes as f64 / 1024.0;
    let write_disk_usage = p.disk_usage().total_written_bytes as f64 / 1024.0;

    ProcessRow {
        pid: p.pid().as_u32(),
        name: p.name().to_string(),
        cpu_usage: p.cpu_usage() as f64,
        memory_mb: p.memory() as f64 / 1024.0,
        user: user_name,
        groups,
        state,
        exe,
        cmd,
        start_time,
        parent_pid,
        virtual_memory_mb,
        read_disk_usage,
        write_disk_usage,
    }
}


