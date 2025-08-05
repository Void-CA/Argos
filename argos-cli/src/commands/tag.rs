use crate::{error::CliResult, output::OutputFormatter};

pub fn handle_tag(formatter : &OutputFormatter, name: &str, pid: u32) -> CliResult<()> {
    // Implementar la lógica para etiquetar procesos
    // Por ahora, solo imprimimos un mensaje
    println!("Etiqueta '{}' aplicada al proceso con PID {}", name, pid);
    
    // Aquí podrías guardar la etiqueta en una base de datos o archivo si es necesario
    Ok(())
}