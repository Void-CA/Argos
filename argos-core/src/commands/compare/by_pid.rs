use crate::process::{model::ProcessRow, transform::process_to_row};

// argos_core::compare::by_pid.rs
pub fn compare_by_pid(pids: &[u32]) -> Result<Vec<ProcessRow>, crate::errors::CoreError> {
    let mut system = sysinfo::System::new_all();
    system.refresh_processes();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    system.refresh_processes();

    let processes: Vec<_> = system
        .processes()
        .values()
        .filter(|p| pids.contains(&p.pid().as_u32()))
        .collect();

    let rows: Vec<ProcessRow> = processes.iter().map(|p| process_to_row(p)).collect();
    Ok(rows)
}
