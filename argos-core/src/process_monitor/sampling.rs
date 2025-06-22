use std::{thread, time::Duration, time::Instant};
use sysinfo::{System};
use crate::process_monitor::types::Sample;

pub fn monitor_during_execution(pid: u32, iterations: usize, interval_ms: u64) -> Vec<Sample> {
    let mut samples = Vec::with_capacity(iterations);
    let mut system = System::new_all();
    let pid = sysinfo::Pid::from_u32(pid);
    let start = Instant::now();

    for _ in 0..iterations {
        system.refresh_process(pid);
        if let Some(proc) = system.process(pid) {
            samples.push(Sample {
                timestamp: start.elapsed().as_secs_f64(),
                cpu_usage: proc.cpu_usage(),
                memory: proc.memory(),
            });
        } else {
            break;
        }

        thread::sleep(Duration::from_millis(interval_ms));
    }

    samples
}
