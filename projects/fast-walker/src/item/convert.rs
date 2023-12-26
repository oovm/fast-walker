use super::*;
use std::path::Path;

// impl Add<isize> for WalkItem {
//     type Output = Self;
//
//     fn add(self, rhs: isize) -> Self::Output {
//         Self { depth: self.depth + rhs, ..self }
//     }
// }
impl<'i> From<&'i Path> for WalkItem {
    fn from(value: &Path) -> Self {
        Self { path: value.to_path_buf(), depth: 0 }
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
