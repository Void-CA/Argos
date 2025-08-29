use crate::{errors::{CoreError, CoreResult}, process::{model::ProcessRow, reader::ProcessReader}};

pub fn get_family(pid: u32) -> CoreResult<Vec<ProcessRow>> {
    let mut reader = ProcessReader::new();

    if let Some(process) = reader.get_by_pids(&[pid]).into_iter().next() {
        let mut family = vec![process.clone()];
        family.extend(reader.get_children(pid));
        Ok(family)
    } else {
        Err(CoreError::ProcessNotFound(pid))
    }
}
