use std::{
    fmt::{Debug, Display, Formatter},
    fs::{DirEntry, File, ReadDir},
    ops::Add,
    path::PathBuf,
};

#[derive(Clone, Debug)]
pub struct WalkItem {
    pub path: PathBuf,
    pub depth: usize,
}

impl Add<usize> for WalkItem {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self { depth: self.depth + rhs, ..self }
    }
}

impl From<PathBuf> for WalkItem {
    fn from(value: PathBuf) -> Self {
        Self { path: value, depth: 0 }
    }
}

impl From<DirEntry> for WalkItem {
    fn from(value: DirEntry) -> Self {
        Self { path: value.path(), depth: 0 }
    }
}

// impl Debug for WalkItem {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             WalkItem::File { path } => f.debug_struct("File").field("path", &to_unix_path(path)).finish(),
//             WalkItem::Directory { path } => f.debug_struct("Directory").field("path", &to_unix_path(path)).finish(),
//             WalkItem::Error { directory, error } => {
//                 f.debug_struct("Error").field("directory", &to_unix_path(directory)).field("error", &error).finish()
//             }
//         }
//     }
// }

impl Display for WalkItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!();
        // match self {
        //     WalkItem::File { path } => write_unix_path(f, path),
        //     WalkItem::Directory { path } => write_unix_path(f, path),
        //     WalkItem::Error { directory, error } => {
        //         write!(f, "Error: {} in {}", error, directory.display())
        //     }
        // }
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
