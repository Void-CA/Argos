use crate::error::CliResult;

pub fn handle_tui() -> CliResult<()> {
    // Iniciar la TUI
    argos_tui::run_tui().map_err(|e| {
        eprintln!("Error al iniciar la TUI: {}", e);
        crate::error::CliError::io_error("Error al iniciar la TUI")
    })
}