use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "argos")]
#[command(version = "0.1.0")]
#[command(about = "Herramienta de auditoría y análisis de recursos en tiempo real", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Muestra un snapshot del proceso
    Monitor {
        /// ID del proceso (PID)
        #[arg(short, long)]
        pid: u32,
        
        /// Formato de salida (text, json, csv)
        #[arg(short, long, default_value = "text")]
        format: String,
        
        /// Guardar en base de datos
        #[arg(long)]
        save: bool,
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
        #[arg(short = 'i', long, default_value_t = 200)]
        interval_ms: u64,
        
        /// Formato de salida (text, json, csv)
        #[arg(short, long, default_value = "text")]
        format: String,
        
        /// Archivo de salida (opcional)
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Muestra historial de procesos
    History {
        /// PID específico (opcional)
        #[arg(short, long)]
        pid: Option<u32>,
        
        /// Límite de registros
        #[arg(short, long, default_value_t = 50)]
        limit: usize,
        
        /// Formato de salida (text, json, csv)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    
    /// Lista todos los procesos activos
    List {
        /// Filtrar por nombre de proceso
        #[arg(short, long)]
        name: Option<String>,
        
        /// Filtrar por usuario
        #[arg(short, long)]
        user: Option<String>,
        
        /// Ordenar por (cpu, memory, name, pid)
        #[arg(long, default_value = "cpu")]
        sort_by: String,
        
        /// Formato de salida (text, json, csv)
        #[arg(short, long, default_value = "text")]
        format: String,

        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Configuración del sistema
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    Live {
        /// ID del proceso (PID)
        #[arg(short, long)]
        pid: u32,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Mostrar configuración actual
    Show,
    /// Establecer valor de configuración
    Set {
        key: String,
        value: String,
    },
    /// Resetear configuración a valores por defecto
    Reset,
}
