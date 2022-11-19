use std::error;
use std::fmt;

#[derive(Debug)]
pub struct ShellError {
    title: String,
}

impl ShellError {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

impl error::Error for ShellError {}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

impl From<std::io::Error> for ShellError {
    fn from(input: std::io::Error) -> Self {
        ShellError {
            title: format!("{}", input),
        }
    }
}
