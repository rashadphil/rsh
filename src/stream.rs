use std::process::Stdio;

use crate::types::primary::Value;

#[derive(Debug)]
pub enum RushStream {
    Internal(Value),
    External(Stdio),
    None,
}
