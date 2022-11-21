use std::fs;

use crate::error::ShellError;

use super::{datadict::DataDict, descriptor::Descriptor, primary::Value};

pub fn direntry_dict(entry: fs::DirEntry) -> Result<DataDict, ShellError> {
    let mut dict = DataDict::default();

    let file_name = entry.file_name();

    let metadata = entry.metadata()?;
    let len = metadata.len();
    let modified = metadata.modified()?;

    dict.insert("name", Value::string(file_name.to_string_lossy()));
    dict.insert("size", Value::size(len));
    dict.insert("modified", Value::time(modified));

    Ok(dict)
}
