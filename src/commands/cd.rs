use std::path::PathBuf;

use crate::{
    error::ShellError,
    types::{primary::Value, process::Process},
};
use sysinfo::SystemExt;

use super::{Args, Command};

pub struct Cd;

impl Command for Cd {
    fn run(&self, args: Args) -> Result<Value, ShellError> {
        let env = args.env;
        let cwd = env.cwd();

        let home_path = match home::home_dir() {
            Some(path) => Ok(path),
            None => Err(ShellError::new(format!("cd : Could not find home path"))),
        };

        let new_path = if args.args.len() == 0 {
            home_path?
        } else {
            let input_path = &args.args[0].to_string();
            cwd.join(input_path.to_string())
        };

        match env.set_cwd(&new_path) {
            Ok(_) => Ok(Value::string(new_path.to_string_lossy())),
            Err(_) => Err(ShellError::new(format!("cd : no such directory"))),
        }
    }
}
