pub mod ls;
pub mod ps;

use crate::{error::ShellError, types::primary::Value};

pub trait Command {
    fn run(&self) -> Result<Value, ShellError>;
}
