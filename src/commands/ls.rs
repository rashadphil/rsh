use std::{env, fs};

use crate::{
    error::ShellError,
    types::{direntry::DirEntry, primary::Value},
};

use super::Command;

#[derive(Debug)]
pub struct Ls {}

impl Command for Ls {
    fn run(&self) -> Result<Value, ShellError> {
        let current_dir = env::current_dir()?;
        let paths = fs::read_dir(current_dir)?;

        let mut dir_entries = vec![];

        for path in paths {
            let entry = Value::object(DirEntry::new(path?)?);
            dir_entries.push(entry);
        }

        Ok(Value::list(dir_entries))
    }
}
