use crate::error::ShellError;

use super::{datadict::DataDict, primary::Value};

pub fn process_dict(proc: sysinfo::Process) -> Result<DataDict, ShellError> {
    let mut dict = DataDict::default();

    let name = proc.name.to_owned();
    let pid = proc.pid;
    let memory = proc.memory;

    dict.insert("name", Value::string(name));
    dict.insert("pid", Value::int(pid));
    dict.insert("memory", Value::size(memory));

    Ok(dict)
}
