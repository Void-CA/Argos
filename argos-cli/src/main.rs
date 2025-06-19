use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "argos")]
#[command(version = "0.1.0")]
#[command(about = "CLI para supervisión de procesos", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Supervisa un proceso
    Monitor {
        #[arg(short, long)]
        pid: u32,
    },
    /// Muestra historial de procesos
    History,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Monitor { pid } => {
            println!("(TEST) Llamada a monitor con PID: {}", pid);
        }
        Commands::History => {
            println!("(TEST) Llamada a history");
        }
    }
}
