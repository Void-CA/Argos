use crate::process::{model::ProcessRow, transform::process_to_row};

pub struct ProcessReader {
    system: sysinfo::System,
}

impl ProcessReader {
    pub fn new() -> Self {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
        Self { system }
    }

    pub fn refresh(&mut self) {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.system.refresh_all();
    }

    pub fn get_all(&self) -> Vec<ProcessRow> {
        self.system
            .processes()
            .values()
            .map(|p| process_to_row(p))
            .collect()
    }

    pub fn get_by_pids(&self, pids: &[u32]) -> Vec<ProcessRow> {
        self.system
            .processes()
            .values()
            .filter(|p| pids.contains(&p.pid().as_u32()))
            .map(|p| process_to_row(p))
            .collect()
    }
}
