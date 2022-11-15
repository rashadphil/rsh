pub mod ls;
pub mod ps;

use crate::{error::ShellError, types::primary::Value};

pub trait Command {
    fn run(&mut self) -> Result<Value, ShellError>;
}
