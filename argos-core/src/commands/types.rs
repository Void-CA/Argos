#[derive(Debug, Clone)]
pub enum Condition {
    CpuAbove(f32),   // Ej: CPU > 80%
    MemAbove(u64),   // Ej: Memoria > 1 GB
    ProcessExit,     // Ej: Proceso terminó
}

impl Condition {
    pub fn is_triggered(&self, cpu: f32, mem: u64) -> bool {
        match self {
            Condition::CpuAbove(limit) => cpu > *limit,
            Condition::MemAbove(limit) => mem > *limit,
            Condition::ProcessExit => false, // se maneja aparte
        }
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Log(String),
    Kill,
    Export(String),
}

impl Action {
    pub fn execute(&self, process: &sysinfo::Process) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Action::Log(msg) => {
                println!("[WATCHDOG] {}", msg);
            }
            Action::Kill => {
                process.kill();
            }
            Action::Export(path) => {
                // Aquí podrías reutilizar tu lógica de exportación
                println!("Exportando métricas a {}", path);
            }
        }
        Ok(())
    }
}
