use std::time::Duration;

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
        std::thread::sleep(Duration::from_millis(500));
        self.system.refresh_all();
    }

    pub fn get_all(&mut self) -> Vec<ProcessRow> {
        self.refresh(); 
        self.system
            .processes()
            .values()
            .map(|p| process_to_row(p))
            .collect()
    }

    pub fn get_by_pids(&mut self, pids: &[u32]) -> Vec<ProcessRow> {
        self.refresh();
        self.system
            .processes()
            .values()
            .filter(|p| pids.contains(&p.pid().as_u32()))
            .map(|p| process_to_row(p))
            .collect()
    }
}
