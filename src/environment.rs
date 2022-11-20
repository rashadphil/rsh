use std::{env, path::PathBuf};

#[derive(Debug, Default)]
pub struct Environment {}

impl Environment {
    pub fn cwd(&self) -> PathBuf {
        env::current_dir().unwrap()
    }

    pub fn set_cwd(&self, dir_path: &PathBuf) -> Result<(), std::io::Error> {
        env::set_current_dir(dir_path)
    }
}
