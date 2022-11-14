use std::error;
use std::fmt;

#[derive(Debug)]
pub struct ShellError {}

impl error::Error for ShellError {}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Shell Error!")
    }
}

impl From<std::io::Error> for ShellError {
    fn from(_: std::io::Error) -> Self {
        ShellError {} 
    }
}
