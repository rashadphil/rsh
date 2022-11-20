use std::{collections::BTreeMap, rc::Rc};

use crate::{commands::Command, environment::Environment};

#[derive(Default)]
pub struct Context {
    pub env: Rc<Environment>,
    pub valid_commands: BTreeMap<String, Rc<dyn Command>>,
}

impl Context {
    pub fn insert_command(&mut self, name: impl Into<String>, command: Rc<dyn Command>) {
        self.valid_commands.insert(name.into(), command);
    }

    pub fn insert_commands(&mut self, commands: Vec<(impl Into<String>, Rc<dyn Command>)>) {
        for (name, command) in commands {
            self.insert_command(name, command);
        }
    }

}
