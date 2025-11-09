use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Entry {
    path: PathBuf,
    depth: usize,
}

impl Entry {
    pub fn new(path: PathBuf, depth: usize) -> Self {
        Self { path, depth }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        fs::metadata(&self.path)
    }

    pub fn symlink_metadata(&self) -> io::Result<fs::Metadata> {
        fs::symlink_metadata(&self.path)
    }

    pub fn file_type(&self) -> io::Result<fs::FileType> {
        fs::symlink_metadata(&self.path).map(|m| m.file_type())
    }
}
