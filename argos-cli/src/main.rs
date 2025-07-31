use clap::{Parser, Subcommand};
use argos_core::process_monitor::{
    monitor_process,
    monitor_during_execution,
};
use argos_core::db::process::insert_process;
use argos_core::db::manager::establish_connection;
use dotenvy;
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
    /// Muestra un snapshot del proceso
    Monitor {
        /// ID del proceso (PID)
        #[arg(short, long)]
        pid: u32,
    },
    /// Realiza un muestreo durante varios segundos
    Sample {
        /// ID del proceso (PID)
        #[arg(short, long)]
        pid: u32,

        /// Número de iteraciones
        #[arg(long, default_value_t = 10)]
        iterations: usize,

        /// Intervalo entre muestras (milisegundos)
        #[arg(short = 'i', long, default_value_t = 500)]
        interval_ms: u64,
    },
    /// (Placeholder) Muestra historial de procesos
    History,
}

fn main() {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Monitor { pid } => {
            match monitor_process(pid) {
                Some(info) => {
                
                    let mut conn = establish_connection();
                    let process = info.clone().into();
                    if let Err(e) = insert_process(&mut conn, &process) {
                        eprintln!("❌ Error al insertar el proceso en la base de datos: {}", e);
                    }
                    println!(
                        "Nombre: {}\nEstado: {}\nCPU: {:.2}%\nRAM: {:.2} MB\nInicio: {}\nPID Padre: {:?}",
                        info.name,
                        info.state,
                        info.cpu_usage,
                        info.memory_mb,
                        info.start_time,
                        info.parent_pid.unwrap_or(0),
                    );
                }
                None => eprintln!("❌ No se encontró el proceso con PID {}", pid),
            }
        }

        Commands::Sample { pid, iterations, interval_ms } => {
            println!(
                "🔍 Muestreo del PID {} por {} iteraciones ({} ms c/u):\n",
                pid, iterations, interval_ms
            );
            let samples = monitor_during_execution(pid, iterations, interval_ms);
            if samples.is_empty() {
                println!("❌ No se pudo obtener información del proceso.");
                return;
            }
            for s in samples {
                println!(
                    " {:.2}s |  {:>8} KB |  {:5.2}%",
                    s.timestamp, s.memory, s.cpu_usage
                );
            }
        }

        Commands::History => {
            println!("📦 Llamada a history (por implementar)");
        }
    }
}
