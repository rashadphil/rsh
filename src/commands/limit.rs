use crate::{error::ShellError, types::primary::Value};

use super::{Args, Command};

#[derive(Debug)]
pub struct Limit;

impl Command for Limit {
    fn run(&self, args: Args) -> Result<Value, ShellError> {
        if args.args.is_empty() {
            return Err(ShellError::new("No limit number provided"));
        }

        let instream = match args.instream {
            Some(stream) => stream,
            None => return Err(ShellError::new("No values given to limit")),
        };

        let mut objects = match instream.values {
            Value::List(list) => list,
            _ => return Err(ShellError::new("limit expects a list of objects")),
        };

        let limit = &args.args[0].to_int()?;

        objects.truncate(*limit as usize);

        Ok(Value::list(objects))
    }
}
