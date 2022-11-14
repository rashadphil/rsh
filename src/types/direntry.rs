use std::fs;

use super::primary::RshObject;

#[derive(Debug)]
pub struct DirEntry {
    inner: fs::DirEntry,
}

impl DirEntry {
    pub fn new(entry: fs::DirEntry) -> Self {
        DirEntry { inner: entry }
    }
}

impl RshObject for DirEntry {}
