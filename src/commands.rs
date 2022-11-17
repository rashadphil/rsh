use std::rc::Rc;

use derive_new::new;

pub mod cd;
pub mod ls;
pub mod ps;

use crate::{error::ShellError, types::primary::Value};

pub enum CommandType {
    Internal(InternalCommand),
    External(ExternalCommand),
}

#[derive(new)]
pub struct InternalCommand {
    pub command: Rc<dyn Command>,
    pub args: Vec<Value>,
}

#[derive(Debug, new)]
pub struct ExternalCommand {
    pub command: String,
    pub args: Vec<String>,
}

pub trait Command {
    fn run(&self, args: Vec<Value>) -> Result<Value, ShellError>;
}
