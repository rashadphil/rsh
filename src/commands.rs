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
    context::Context, environment::Environment, error::ShellError, stream::InStream,
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
    pub fn run(self, ctx: &Context, instream: Option<InStream>) -> Result<Value, ShellError> {
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
    pub fn run(&self, stdin: Stdio, stdout: Stdio) -> Result<Child, ShellError> {
        Ok(process::Command::new(&self.command)
            .args(&self.args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()?)
    }
}

#[derive(new)]
pub struct Args {
    pub env: Rc<Environment>,
    pub args: Vec<Value>,
    pub instream: Option<InStream>,
}

pub trait Command {
    fn run(&self, args: Args) -> Result<Value, ShellError>;
}
