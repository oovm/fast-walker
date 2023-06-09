use std::{
    fmt::{Debug, Display, Formatter, Write},
    path::{Path, PathBuf},
};

pub enum WalkItem {
    File { path: PathBuf },
    Directory { path: PathBuf },
    Error { directory: PathBuf, error: std::io::Error },
}

impl Debug for WalkItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WalkItem::File { path } => f.debug_struct("File").field("path", &to_unix_path(path)).finish(),
            WalkItem::Directory { path } => f.debug_struct("Directory").field("path", &to_unix_path(path)).finish(),
            WalkItem::Error { directory, error } => {
                f.debug_struct("Error").field("directory", &to_unix_path(directory)).field("error", &error).finish()
            }
        }
    }
}

impl Display for WalkItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WalkItem::File { path } => write_unix_path(f, path),
            WalkItem::Directory { path } => write_unix_path(f, path),
            WalkItem::Error { directory, error } => {
                write!(f, "Error: {} in {}", error, directory.display())
            }
        }
    }
}

fn to_unix_path(path: &Path) -> String {
    path.to_string_lossy().trim_start_matches("\\\\?\\").replace("\\", "/")
}

fn write_unix_path(f: &mut Formatter<'_>, path: &Path) -> std::fmt::Result {
    for c in path.to_string_lossy().trim_start_matches("\\\\?\\").chars() {
        if c == '\\' { f.write_char('/')? } else { f.write_char(c)? }
    }
    Ok(())
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
