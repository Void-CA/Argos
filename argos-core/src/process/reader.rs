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

    pub fn get_children(&mut self, pid: u32) -> Vec<ProcessRow> {
        self.refresh(); // refresca una vez
        let all = self.get_all(); // todos los procesos una sola vez

        // mapa padre -> hijos
        let mut parent_map: std::collections::HashMap<u32, Vec<&ProcessRow>> = std::collections::HashMap::new();
        for p in &all {
            if let Some(ppid) = p.parent_pid {
                parent_map.entry(ppid).or_default().push(p);
            }
        }

        // función recursiva para recolectar hijos
        fn collect_children(
            pid: u32,
            parent_map: &std::collections::HashMap<u32, Vec<&ProcessRow>>,
            out: &mut Vec<ProcessRow>,
        ) {
            if let Some(children) = parent_map.get(&pid) {
                for child in children {
                    out.push((*child).clone());
                    collect_children(child.pid, parent_map, out);
                }
            }
        }

        let mut children = Vec::new();
        collect_children(pid, &parent_map, &mut children);
        children
    }


    pub fn get_zombies(&mut self) -> Vec<ProcessRow> {
        self.refresh();
        self.get_all()
            .into_iter()
            .filter(|p| p.state == "Zombie") // o usa un campo booleano si lo agregas
            .collect()
    }
}
