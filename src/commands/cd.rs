use std::path::PathBuf;

use super::{Args, Command};
use crate::{error::ShellError, types::primary::Value};

pub struct Cd;

impl Command for Cd {
    fn run(&self, args: Args) -> Result<Value, ShellError> {
        let env = args.env;
        let cwd = env.cwd();

        let home_path = match home::home_dir() {
            Some(path) => Ok(path),
            None => Err(ShellError::new("cd : Could not find home path".to_string())),
        };

        let new_path = if args.args.is_empty() {
            home_path?
        } else {
            let path_arg = &args.args[0].to_string();
            let input_path = PathBuf::from(path_arg);

            if input_path.is_absolute() || input_path.starts_with("~") {
                let expanded = format!("{}", shellexpand::tilde(path_arg));
                PathBuf::from(expanded)
            } else {
                cwd.join(input_path)
            }
        };

        match env.set_cwd(&new_path) {
            Ok(_) => Ok(Value::string(new_path.to_string_lossy())),
            Err(_) => Err(ShellError::new("cd : no such directory".to_string())),
        }
    }
}
