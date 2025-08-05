use std::path::PathBuf;

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

        /// Archivo de salida (opcional)
        #[arg(short, long)]
        output: Option<String>,

        /// Mostrar solo los primeros N resultados
        #[arg(long)]
        top: Option<usize>,
    },
    
    /// Configuración del sistema
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Muestra información en tiempo real de un proceso
    Live {
        /// ID del proceso (PID)
        #[arg(short, long)]
        pid: u32,
    },

    /// Comparacion entre dos procesos
    Compare {
        /// IDs de los procesos (PIDs)
        #[arg(long, num_args = 1.., conflicts_with("files"))]
        pids: Option<Vec<u32>>,

        /// Direccion de los files a comparar
        #[arg(long, num_args = 1.., conflicts_with("pids"))]
        files: Option<Vec<PathBuf>>,

        /// Formato de salida (text, json, csv)
        #[arg(short, long, default_value = "text")]
        format: String,

        /// Archivo de salida (opcional)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Genera logs y reportes de auditoría
    Watchdog {
        /// ID del proceso a monitorear (PID)
        #[arg(short, long)]
        pid: u32,

        /// Umbral de CPU para activar la alerta
        #[arg(long, default_value_t = 80)]
        cpu_over: u8,

        /// Umbral de memoria para activar la alerta
        #[arg(long, default_value_t = 80)]
        memory_over: u8,

        /// Acción a realizar cuando se exceden los umbrales
        #[arg(long)]
        on_exceed: Option<String>,

        /// Intervalo de tiempo para verificar los umbrales (en milisegundos)
        #[arg(long, default_value_t = 1000)]
        interval: u64,
    },

    /// Etiquetado de procesos
    Tag {
        /// Nombre de la etiqueta
        #[arg(short, long)]
        name: String,

        /// ID del proceso (PID)
        #[arg(short, long)]
        pid: u32,
    }
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
