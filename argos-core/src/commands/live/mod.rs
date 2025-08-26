use std::{thread, time::Duration};
use crate::{
    errors::{CoreError, CoreResult},
    process::{model::ProcessRow, reader::ProcessReader},
};

/// Monitorea en vivo un proceso por PID, devolviendo un vector de ProcessRow con cada muestreo.
/// El callback se llama en cada iteración con el ProcessRow actualizado.
pub fn monitor_live_by_pid<F>(pid: u32, mut callback: F) -> CoreResult<()>
where
    F: FnMut(&ProcessRow),
{
    let reader = ProcessReader::new();
    loop {
        let rows = reader.get_by_pids(&[pid]);
        if let Some(row) = rows.into_iter().next() {
            callback(&row);
            thread::sleep(Duration::from_secs(1));
        } else {
            return Err(CoreError::ProcessNotFound(pid));
        }
    }
}