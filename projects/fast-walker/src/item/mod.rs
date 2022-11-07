use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum WalkItem {
    File { path: PathBuf },
    Directory { path: PathBuf },
    Error { directory: PathBuf, error: std::io::Error },
}

impl WalkItem {
    pub fn file<P: AsRef<Path>>(path: P) -> Self {
        Self::File { path: path.as_ref().to_path_buf() }
    }
    pub fn directory<P: AsRef<Path>>(path: P) -> Self {
        Self::Directory { path: path.as_ref().to_path_buf() }
    }
    pub fn error<P: AsRef<Path>>(path: P, error: std::io::Error) -> Self {
        Self::Error { directory: path.as_ref().to_path_buf(), error }
    }
    pub fn path(&self) -> &Path {
        match self {
            Self::File { path, .. } => path.as_path(),
            Self::Directory { path, .. } => path.as_path(),
            Self::Error { directory, .. } => directory.as_path(),
        }
    }
}
