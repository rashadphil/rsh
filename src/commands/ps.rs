use crate::error::ShellError;

use super::Command;

#[derive(Debug)]
pub struct Ps {}

impl Command for Ps {
    fn run(&self) -> Result<(), ShellError> {
        Ok(())
    }
}
