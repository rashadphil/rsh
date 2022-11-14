pub mod ls;
pub mod ps;

use crate::error::ShellError;

pub trait Command {
    fn run(&self) -> Result<(), ShellError>;
}
