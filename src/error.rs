use std::{fmt, io, path::PathBuf};

#[derive(Debug)]
pub enum WalkError {
    Io(io::Error),
    LoopDetected(PathBuf),
}

impl From<io::Error> for WalkError {
    fn from(e: io::Error) -> Self {
        WalkError::Io(e)
    }
}

impl fmt::Display for WalkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalkError::Io(e) => write!(f, "IO error: {}", e),
            WalkError::LoopDetected(p) => {
                write!(f, "Symbolic link loop detected at {}", p.display())
            }
        }
    }
}

impl std::error::Error for WalkError {}
