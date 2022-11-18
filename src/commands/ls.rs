use std::{env, fs};

use crate::{
    error::ShellError,
    types::{direntry::DirEntry, primary::Value},
};

use super::{Args, Command};

#[derive(Debug)]
pub struct Ls;

impl Command for Ls {
    fn run(&self, args: Args) -> Result<Value, ShellError> {
        let cwd = env::current_dir()?;

        let target_dir = if !args.args.is_empty() {
            let input_path = &args.args[0].to_string();
            cwd.join(input_path)
        } else {
            cwd
        };

        let paths = fs::read_dir(target_dir)?;

        let mut dir_entries = vec![];

        for path in paths {
            let entry = Value::object(DirEntry::new(path?)?);
            dir_entries.push(entry);
        }

        Ok(Value::list(dir_entries))
    }
}
