use std::{thread, time::Duration};
use crate::{
    errors::{CoreError, CoreResult},
    process::{model::ProcessRow, reader::ProcessReader},
};

/// Muestrea un único proceso durante varias iteraciones a intervalos fijos.
pub fn sample_process(pid: u32, iterations: usize, interval_ms: u64) -> CoreResult<Vec<ProcessRow>> {
    if iterations == 0 {
        return Err(CoreError::ValidationError("El número de iteraciones debe ser mayor que 0".into()));
    }

    let mut results = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let reader = ProcessReader::new();
        let rows = reader.get_by_pids(&[pid]);

        if rows.is_empty() {
            return Err(CoreError::ProcessNotFound(pid));
        }

        results.push(rows[0].clone()); // Solo un proceso, así que tomamos el primero
        if iterations > 1 {
            thread::sleep(Duration::from_millis(interval_ms));
        }
    }

    Ok(results)
}
