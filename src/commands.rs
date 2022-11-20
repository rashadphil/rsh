use std::{
    process::{self, Child, Stdio},
    rc::Rc,
};

use derive_new::new;

pub mod cd;
pub mod limit;
pub mod ls;
pub mod ps;
pub mod sortby;

use crate::{
    context::Context,
    environment::Environment,
    error::ShellError,
    stream::{RushStream},
    types::primary::Value,
};

pub enum CommandType {
    Internal(InternalCommand),
    External(ExternalCommand),
}

#[derive(new)]
pub struct InternalCommand {
    pub command: Rc<dyn Command>,
    pub args: Vec<Value>,
}

impl InternalCommand {
    pub fn run(self, ctx: &Context, instream: RushStream) -> Result<Value, ShellError> {
        let command = self.command;
        let args = Args::new(ctx.env.clone(), self.args, instream);
        command.run(args)
    }
}

#[derive(Debug, new)]
pub struct ExternalCommand {
    pub command: String,
    pub args: Vec<String>,
}

impl ExternalCommand {
    pub fn run(&self, instream: RushStream, stdout: Stdio) -> Result<Child, ShellError> {
        let stdin = match instream {
            RushStream::Internal(_) => {
                return Err(ShellError::new("internal -> external not supported yet"))
            }
            RushStream::External(stdin) => stdin,
            RushStream::None => Stdio::null(),
        };

        Ok(process::Command::new(&self.command)
            .args(&self.args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()?)
    }
}

#[derive(Debug, new)]
pub struct Args {
    pub env: Rc<Environment>,
    pub args: Vec<Value>,
    pub instream: RushStream,
}

pub trait Command {
    fn run(&self, args: Args) -> Result<Value, ShellError>;
}
