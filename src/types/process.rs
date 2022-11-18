use crate::error::ShellError;

use super::{
    datadict::DataDict,
    descriptor::Descriptor,
    primary::{RshObject, Value},
};

#[derive(Debug)]
pub struct Process {
    dict: DataDict,
}

impl Process {
    pub fn new(proc: sysinfo::Process) -> Result<Self, ShellError> {
        let mut dict = DataDict::default();

        let name = proc.name.to_owned();
        let pid = proc.pid;
        let memory = proc.memory;

        dict.insert("name", Value::string(name));
        dict.insert("pid", Value::int(pid));
        dict.insert("memory", Value::size(memory));

        Ok(Process { dict })
    }
}

impl RshObject for Process {
    fn data_descriptors(&self) -> Vec<Descriptor> {
        self.dict.data_descriptors()
    }
    fn get_data(&self, desc: &Descriptor) -> &Value {
        self.dict.get_data(desc)
    }
}
