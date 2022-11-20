use crate::{error::ShellError, stream::RushStream, types::primary::Value};

use super::{Args, Command};

#[derive(Debug)]
pub struct Rev;

impl Command for Rev {
    fn run(&self, args: Args) -> Result<Value, ShellError> {
        let mut objects = match args.instream {
            RushStream::Internal(Value::List(list)) => list,
            RushStream::External(_) => {
                return Err(ShellError::new("external streams not supported yet"))
            }
            _ => return Err(ShellError::new("rev expects a list of objects")),
        };

        objects.reverse();

        Ok(Value::list(objects))
    }
}
