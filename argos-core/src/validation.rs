use std::path::PathBuf;
use crate::errors::CoreError;
use sysinfo::{System, Pid};

fn pid_exists(pid: u32) -> bool {
    let system = System::new_all();
    system.process(Pid::from_u32(pid)).is_some()
}

pub fn validate_pids(pids: &Option<Vec<u32>>) -> Result<(), CoreError> {
    if let Some(pids) = pids {
        for &pid in pids {
            if !pid_exists(pid) {
                return Err(CoreError::Other(format!("Proceso no encontrado: PID {}", pid)));
            }
        }
    }
    Ok(())
}

pub fn validate_files(files: &Option<Vec<PathBuf>>) -> Result<(), CoreError> {
    if let Some(files) = files {
        for file in files {
            if !file.exists() {
                return Err(CoreError::Other(format!("El archivo no existe: {}", file.display())));
            }
            if !file.is_file() {
                return Err(CoreError::Other(format!("No es un archivo válido: {}", file.display())));
            }
        }
    }
    Ok(())
}

pub fn validate_comparison_inputs(
    pids: &Option<Vec<u32>>,
    files: &Option<Vec<PathBuf>>,
) -> Result<(), CoreError> {
    if pids.is_none() && files.is_none() {
        return Err(CoreError::Other(
            "Debe proporcionar al menos un PID o un archivo para comparar".to_string(),
        ));
    }

    if pids.is_some() && files.is_some() {
        return Err(CoreError::Other(
            "No puede proporcionar simultáneamente PIDs y archivos para comparar. Elija solo uno.".to_string(),
        ));
    }

    validate_pids(pids)?;
    validate_files(files)?;

    Ok(())
}