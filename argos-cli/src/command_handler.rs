use crate::cli::{Commands};
use crate::output::OutputFormatter;
use crate::error::{CliResult};
use crate::config::Config;
use crate::handlers::{list::handle_list,
                     monitor::handle_monitor,
                     sample::handle_sample,
                     live::handle_live,
                     compare::handle_compare,
                     watchdog::handle_watchdog,
                     config::handle_config};

                     
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
            Commands::Live {pid} => {
                handle_live(pid)
            }
            Commands::Compare {pids, files, format, output} => {
                let pid_opt = pids.and_then(|vec| vec.into_iter().next());
                let file_opt = files.and_then(|vec| vec.into_iter().next()).and_then(|pb| pb.to_str().map(|s| s.to_string()));
                handle_compare(pid_opt, file_opt.as_deref(), &format, output.as_deref())
            }
            Commands::Watchdog { pid, cpu_over, memory_over, on_exceed, interval } => {
                handle_watchdog(pid, cpu_over, memory_over, on_exceed, interval)
            }
            Commands::Tag { name, pid } => {
                print!("Tag command selected with name: {}, pid: {}\n", name, pid);
                Ok(())
            }
            Commands::Config { action } => {
                handle_config(&mut self.config, action)
            }
        }
    }
}



