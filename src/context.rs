use std::{collections::BTreeMap, path::Path, rc::Rc};

use crate::{commands::Command, environment::Environment, error::ShellError};

#[derive(Default, Clone)]
pub struct Context {
    pub env: Rc<Environment>,
    pub valid_commands: BTreeMap<String, Rc<dyn Command>>,
    pub external_commands: radix_trie::Trie<String, bool>,
}

impl Context {
    fn add_path_executables(&mut self, path: &Path) -> Result<(), ShellError> {
        let entries = std::fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                self.external_commands.insert(name.to_string(), true);
            }
        }
        Ok(())
    }

    pub fn generate_externals(&mut self) {
        self.external_commands = radix_trie::Trie::new();
        let paths = std::env::var("PATH").unwrap();
        let paths: Vec<&str> = paths.split(':').collect();
        for path in paths {
            let path = std::path::Path::new(path);
            self.add_path_executables(path);
        }
    }
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

    pub fn command_exists(&self, name: &str) -> bool {
        self.valid_commands.contains_key(name) || self.external_commands.get(name).is_some()
    }
}
