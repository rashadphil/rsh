use crate::{error::ShellError, stream::RushStream, types::primary::Value};

use super::{Args, Command};

#[derive(Debug)]
pub struct Limit;

impl Command for Limit {
    fn run(&self, args: Args) -> Result<Value, ShellError> {
        if args.args.is_empty() {
            return Err(ShellError::new("No limit number provided"));
        }

        let mut objects = match args.instream {
            RushStream::Internal(Value::List(list)) => list,
            RushStream::External(_) => {
                return Err(ShellError::new("external streams not supported yet"))
            }
            _ => return Err(ShellError::new("limit expects a list of objects")),
        };

        let limit = &args.args[0].to_int()?;
        objects.truncate(*limit as usize);

        Ok(Value::list(objects))
    }
}
