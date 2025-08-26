use crate::cli::{Commands, ConfigAction};
use crate::output::OutputFormatter;
use crate::error::{CliResult, CliError};
use crate::config::Config;

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
            Commands::List { format, output, .. } => {
                handle_list(&format, output.as_deref())
            }
            Commands::Monitor { pid, format, save } => {
                handle_monitor(pid, &format, save)
            }
            Commands::Sample { pid, iterations, interval_ms, format, output } => {
                handle_sample(pid, iterations, interval_ms, &format, output.as_deref())
            }
            Commands::History { pid, limit, format } => {
                handle_history(pid, limit, &format)
            }
            Commands::Live {pid} => {
                handle_live(pid)
            }
            Commands::Compare {pids, files, format, output} => {
                handle_compare(pids, files, &format, output.as_deref())
            }
            Commands::Watchdog { pid, cpu_over, memory_over, on_exceed, interval } => {
                handle_watchdog(pid, cpu_over, memory_over, on_exceed, interval)
            }
            Commands::Tag { name, pid } => {
                handle_tag(&name, pid)
            }
            Commands::Config { action } => {
                handle_config(action)
            }
        }
    }
}



