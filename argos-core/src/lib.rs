use sysinfo::{System};
use std::{thread, time::Duration};


pub fn monitor_process(pid: u32) -> String {
    let mut system = System::new_all();
    system.refresh_all();
    thread::sleep(Duration::from_millis(2000));
    system.refresh_all();

    let pid = sysinfo::Pid::from_u32(pid);
    if let Some(process) = system.process(pid) {
        format!(
    "Nombre: {}\nEstado: {:?}\nCPU: {:.2}%\nRAM: {} KB\nInicio: {}s\nPID Padre: {:?}\nCmd: {:?}",
    process.name(),
    process.status(),
    process.cpu_usage(),
    process.memory(),
    process.start_time(),
    process.parent(),
    process.cmd()
)

    } else {
        format!("No se encuentra el proceso")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_process_not_found() {
        let output = monitor_process(999999); // PID improbable
        assert!(output.contains("No se encuentra el proceso"));
    }
}