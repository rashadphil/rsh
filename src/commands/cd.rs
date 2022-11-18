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
            let input_path = &args.args[0].to_string();
            cwd.join(input_path)
        };

        match env.set_cwd(&new_path) {
            Ok(_) => Ok(Value::string(new_path.to_string_lossy())),
            Err(_) => Err(ShellError::new("cd : no such directory".to_string())),
        }
    }
}
