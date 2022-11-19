use crate::{error::ShellError, types::primary::Value};

use super::{Args, Command};

#[derive(Debug)]
pub struct SortBy;

impl Command for SortBy {
    fn run(&self, args: Args) -> Result<Value, ShellError> {
        if args.args.is_empty() {
            return Err(ShellError::new("No sortby field provided"));
        }

        let instream = match args.instream {
            Some(stream) => stream,
            None => return Err(ShellError::new("No values given to sortby")),
        };

        let mut objects = match instream.values {
            Value::List(list) => list,
            _ => return Err(ShellError::new("sortby expects a list of objects")),
        };


        let sort_key = &args.args[0].to_string();

        objects.sort_by(|a, b| {
            let a = a.get_data_from_key(sort_key);
            let b = b.get_data_from_key(sort_key);

            match (a, b) {
                (Value::Primitive(a), Value::Primitive(b)) => a.cmp(b),
                _ => std::cmp::Ordering::Equal,
            }
        });

        Ok(Value::list(objects))
    }
}
