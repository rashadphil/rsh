use std::{collections::BTreeMap, fs};

use crate::error::ShellError;

use super::{
    datadict::DataDict,
    descriptor::Descriptor,
    primary::{RshObject, Value},
};

#[derive(Debug)]
pub struct DirEntry {
    inner: fs::DirEntry,
    dict: DataDict,
}

impl DirEntry {
    pub fn new(entry: fs::DirEntry) -> Result<Self, ShellError> {
        let mut dict = DataDict::default();

        let file_name = entry.file_name();

        let metadata = entry.metadata()?;
        let len = metadata.len();
        let modified = metadata.modified()?;
        let accessed = metadata.accessed()?;

        dict.insert("file_name", Value::string(file_name.to_string_lossy()));
        dict.insert("size", Value::int(len as i64));
        dict.insert("modified", Value::time(modified));
        dict.insert("accessed", Value::time(accessed));

        Ok(DirEntry { inner: entry, dict })
    }
}

impl RshObject for DirEntry {
    fn data_descriptors(&self) -> Vec<Descriptor> {
        self.dict.data_descriptors()
    }
    fn get_data(&self, desc: &Descriptor) -> &Value {
        self.dict.get_data(desc)
    }
}
