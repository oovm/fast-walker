use std::{
    fmt::{Debug, Display, Formatter},
    fs::{DirEntry, File, ReadDir},
    ops::Add,
    path::PathBuf,
};

mod convert;

#[derive(Clone, Debug)]
pub struct WalkItem {
    pub path: PathBuf,
    pub depth: usize,
}

impl Display for WalkItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.path.display(), f)
    }
}

impl WalkItem {
    pub fn new(raw: PathBuf) -> Self {
        Self { path: raw, depth: 0 }
    }
    pub fn with_depth(self, depth: usize) -> Self {
        Self { depth, ..self }
    }
    pub fn is_link(&self) -> bool {
        self.path.is_symlink()
    }
    pub fn read_link(&self) -> std::io::Result<PathBuf> {
        debug_assert!(self.path.is_symlink());
        self.path.read_link()
    }
    pub fn is_directory(&self) -> bool {
        self.path.is_dir()
    }
    pub fn read_directory(&self) -> std::io::Result<ReadDir> {
        debug_assert!(self.path.is_dir());
        self.path.read_dir()
    }
    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }
    pub fn read_file(&self) -> std::io::Result<File> {
        debug_assert!(self.path.is_file());
        File::open(&self.path)
    }
    pub fn read_file_string(&self) -> std::io::Result<String> {
        debug_assert!(self.path.is_file());
        std::fs::read_to_string(&self.path)
    }
}
