use std::{env, path::PathBuf};

pub struct Environment {
    curr_dir: PathBuf,
}

impl Environment {
    pub fn cwd(&self) -> PathBuf {
        self.curr_dir.clone()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            curr_dir: env::current_dir().unwrap(),
        }
    }
}
