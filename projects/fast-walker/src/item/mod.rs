use std::path::PathBuf;

#[derive(Debug)]
pub enum WalkItem {
    File {
        path: PathBuf,
        depth: usize,
    },
    Directory {
        path: PathBuf,
        depth: usize,
    },
    Error {
        directory: PathBuf,
        error: std::io::Error
    },
}

