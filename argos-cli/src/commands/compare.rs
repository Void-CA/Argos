use std::path::PathBuf;

use crate::{error::{CliError, CliResult}, output::OutputFormatter};



pub fn handle_compare(formatter : &OutputFormatter, pids: Option<Vec<u32>>, files: Option<Vec<PathBuf>>, format: &str, output: Option<&str>) -> CliResult<()> {
    if pids.is_none() && files.is_none() {
        return Err(CliError::new(
            crate::error::ErrorKind::ValidationError,
            "Debe proporcionar al menos un PID o un archivo para comparar",
        ));
    }
    
    Ok(())
}