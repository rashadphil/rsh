use std::{env, fs, path::PathBuf};

use crate::{
    error::ShellError,
    types::{direntry::direntry_dict, primary::Value},
};

use super::{Args, Command};

#[derive(Debug)]
pub struct Ls;

impl Command for Ls {
    fn run(&self, args: Args) -> Result<Value, ShellError> {
        let cwd = env::current_dir()?;

        let target_dir = if !args.args.is_empty() {
            let path_arg = &args.args[0].to_string();
            let input_path = PathBuf::from(path_arg.to_string());

            if input_path.is_absolute() || input_path.starts_with("~") {
                let expanded = format!("{}", shellexpand::tilde(path_arg));
                PathBuf::from(expanded)
            } else {
                cwd.join(input_path)
            }
        } else {
            cwd
        };

        let paths = fs::read_dir(target_dir)?;

        let mut dir_entries = vec![];

        for path in paths {
            let dict = direntry_dict(path?);
            let entry = Value::object(dict?);
            dir_entries.push(entry);
        }

        Ok(Value::list(dir_entries))
    }
}
