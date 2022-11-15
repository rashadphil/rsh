use crate::{
    error::ShellError,
    types::{primary::Value, process::Process},
};
use derive_new::new;
use sysinfo::SystemExt;

use super::Command;

#[derive(new)]
pub struct Ps {
    system: sysinfo::System,
}

impl Command for Ps {
    fn run(&mut self) -> Result<Value, ShellError> {
        self.system.refresh_all();
        let process_list: Vec<sysinfo::Process> =
            self.system.get_process_list().values().cloned().collect();

        let mut process_entries = vec![];

        for process in process_list {
            let entry = Value::object(Process::new(process)?);
            process_entries.push(entry);
        }

        Ok(Value::list(process_entries))
    }
}
