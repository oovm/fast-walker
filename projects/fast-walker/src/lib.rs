mod errors;

use std::path::{Path, PathBuf};
pub use errors::{Error, Result};


mod dir_walker;
mod config;

pub struct FastWalker {
    pub check_list: Vec<PathBuf>,
    pub follow_symlinks: bool,
    pub depth_first: bool,
    pub max_depth: usize,
    pub max_open: usize,
    pub threads: usize,
    pub reject_directory: fn(&Path, usize) -> bool,
}