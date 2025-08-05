use crate::cli::{Commands};
use crate::output::OutputFormatter;
use crate::error::{CliResult};
use crate::config::Config;
use crate::commands::{
    monitor::handle_monitor,
    sample::handle_sample,
    history::handle_history,
    list::handle_list,
    live::handle_live,
    compare::handle_compare,
    watchdog::handle_watchdog,
    tag::handle_tag,
    config::handle_config,
};



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
                handle_monitor(&self.formatter, &self.config, pid, &format, save)
            }
            Commands::Sample { pid, iterations, interval_ms, format, output } => {
                handle_sample(&self.formatter, pid, iterations, interval_ms, &format, output.as_deref())
            }
            Commands::History { pid, limit, format } => {
                handle_history(&self.formatter, pid, limit, &format)
            }
            Commands::List { name, user, sort_by, format , output} => {
                handle_list(&self.formatter, name.as_deref(), user.as_deref(), &sort_by, &format, output.as_deref())
            }
            Commands::Live {pid} => {
                handle_live(&self.formatter, pid)
            }
            Commands::Compare {pids, files, format, output} => {
                handle_compare(&self.formatter, pids, files, &format, output.as_deref())
            }
            Commands::Watchdog { pid, cpu_over, memory_over, on_exceed, interval } => {
                handle_watchdog(&self.formatter, pid, cpu_over, memory_over, on_exceed, interval)
            }
            Commands::Tag { name, pid } => {
                handle_tag(&self.formatter, &name, pid)
            }
            Commands::Config { action } => {
                handle_config(&mut self.config, action)
            }
        }
    }

    
}




