use crate::cli::{Commands};
use crate::handlers::tui::handle_tui;
use crate::output::OutputFormatter;
use crate::error::{CliResult};
use crate::config::Config;
use crate::handlers::{list::handle_list,
                     monitor::handle_monitor,
                     sample::handle_sample,
                     live::handle_live,
                     compare::handle_compare,
                     watchdog::handle_watchdog,
                     config::handle_config,
                     family::handle_family};

                     
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
            Commands::List { format, output, name, user, top, sort_by} => {
                handle_list(&format, output.as_deref(), name, user, top, sort_by)
            }
            Commands::Monitor { pid, format, save } => {
                handle_monitor(pid, &format, save)
            }
            Commands::Sample { pid, iterations, interval_ms, format, output } => {
                handle_sample(pid, iterations, interval_ms, &format, output.as_deref())
            }
            Commands::History { pid, limit, format } => {
                print!("History command selected with pid: {:?}, limit: {}, format: {}\n", pid, limit, format);
                Ok(())
            }
            Commands::Live {pid, output, format} => {
                handle_live(pid, output.as_deref(), format.as_deref())
            }
            Commands::Compare {pids, files, format, output, interval} => {
                handle_compare(pids, files, &format, output.as_deref(), interval)
            }
            Commands::Watchdog { pid, cpu_over, memory_over, on_exceed, interval } => {
                handle_watchdog(pid, cpu_over, memory_over, on_exceed, interval)
            }
            Commands::Tag { name, pid } => {
                print!("Tag command selected with name: {}, pid: {}\n", name, pid);
                Ok(())
            }
            Commands::Family { pid, format } => {
                handle_family(pid, &format)
            }
            Commands::Tui {} => {
                handle_tui()
            }
            Commands::Config { action } => {
                handle_config(&mut self.config, action)
            }
        }
    }
}



