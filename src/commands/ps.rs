use crate::{
    error::ShellError,
    types::{primary::Value, process::Process},
};
use sysinfo::SystemExt;

use super::{Args, Command};

pub struct Ps;

impl Command for Ps {
    fn run(&self, _args: Args) -> Result<Value, ShellError> {
        let system = sysinfo::System::new();
        let process_list: Vec<sysinfo::Process> =
            system.get_process_list().values().cloned().collect();

        let mut process_entries = vec![];

        for process in process_list {
            let entry = Value::object(Process::new(process)?);
            process_entries.push(entry);
        }

        Ok(Value::list(process_entries))
    }
}
