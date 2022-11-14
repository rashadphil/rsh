use crate::{error::ShellError, types::primary::Value};

use super::Command;

#[derive(Debug)]
pub struct Ps {}

impl Command for Ps {
    fn run(&self) -> Result<Value, ShellError> {
        Ok(Value::string("Not yet Implemented!"))
    }
}
