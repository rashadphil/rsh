use std::{env, path::PathBuf};

pub struct Environment {
    curr_dir: PathBuf,
}

impl Environment {
    pub fn cwd(&self) -> PathBuf {
        env::current_dir().unwrap()
    }

    pub fn set_cwd(&self, dir_path: &PathBuf) -> Result<(), std::io::Error> {
        env::set_current_dir(dir_path)
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            curr_dir: env::current_dir().unwrap(),
        }
    }
}
