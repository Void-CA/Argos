mod cli;
mod commands;
mod error;
mod output;
mod config;

use clap::Parser;
use dotenvy;
use std::process;
use cli::Cli;
use commands::CommandHandler;

fn main() {
    // Cargar variables de entorno desde .env si existe
    dotenvy::dotenv().ok();
    
    // Inicializar logging básico
    env_logger::init();
    
    let cli = Cli::parse();
    let mut handler = CommandHandler::new();
    if let Err(error) = handler.handle_command(cli.command) {
        eprintln!("{}", error);
        process::exit(1);
    }
}
