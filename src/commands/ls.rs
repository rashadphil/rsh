use std::{env, fs};

use crate::error::ShellError;

use super::Command;

#[derive(Debug)]
pub struct Ls {}

impl Command for Ls {
    fn run(&self) -> Result<(), ShellError> {
        let current_dir = env::current_dir()?;
        let paths = fs::read_dir(current_dir)?;
        for path in paths {
            println!("{:?}", path);
        }

        Ok(())
    }
}
