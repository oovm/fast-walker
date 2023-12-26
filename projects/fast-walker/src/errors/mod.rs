use std::path::PathBuf;

#[derive(Debug)]
pub struct WalkError {
    pub path: PathBuf,
    pub error: std::io::Error,
}
