use std::{
    error::Error,
    fmt::{Display, Formatter},
    path::PathBuf,
};

mod display;

#[derive(Debug)]
pub struct WalkError {
    kind: Box<WalkErrorKind>,
}

#[derive(Debug)]
pub enum WalkErrorKind {
    IO { path: PathBuf, error: std::io::Error },
}

impl WalkError {
    pub fn io_error(path: PathBuf, error: std::io::Error) -> Self {
        Self { kind: Box::new(WalkErrorKind::IO { path, error }) }
    }
}
