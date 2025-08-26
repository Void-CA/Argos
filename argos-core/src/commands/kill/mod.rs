use crate::{errors::{CoreError, CoreResult}};
use sysinfo::{System, Signal, Pid};

pub fn kill_process(pid: u32) -> CoreResult<()> {
    let mut system = System::new_all();
    system.refresh_process(Pid::from_u32(pid));

    if let Some(process) = system.process(Pid::from_u32(pid)) {
        if process.kill_with(Signal::Kill).unwrap_or(false) {
            Ok(())
        } else {
            Err(CoreError::Other(format!("No se pudo enviar la señal al proceso {}", pid)))
        }
    } else {
        Err(CoreError::ProcessNotFound(pid))
    }
}