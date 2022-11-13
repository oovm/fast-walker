use crate::{WalkItem, WalkResultQueue, WalkTaskQueue};
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

pub mod iterators;

pub mod sync_iter;
#[cfg(feature = "tokio")]
pub mod tokio_iter;

pub struct WalkPlan {
    /// Initial paths to search
    pub check_list: Vec<PathBuf>,
    /// Follow symlinks
    pub follow_symlinks: bool,
    /// Depth first search or breadth first search
    pub depth_first: bool,
    /// Number of threads to use
    pub threads: usize,
    /// Check if a directory should be rejected
    pub reject_when: fn(&Path, usize) -> bool,
    /// Stop if a item matches the condition
    pub finish_when: fn(&WalkItem) -> bool,
}

impl WalkPlan {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            check_list: vec![path.as_ref().to_path_buf()],
            follow_symlinks: false,
            depth_first: false,
            threads: 8,
            reject_when: |_, _| false,
            finish_when: |_| false,
        }
    }
    pub fn breadth_first_search(mut self) -> Self {
        self.depth_first = false;
        self
    }
    pub fn depth_first_search(mut self) -> Self {
        self.depth_first = true;
        self
    }
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }
    pub fn reject_if(mut self, f: fn(&Path, usize) -> bool) -> Self {
        self.reject_when = f;
        self
    }
    pub fn stop_if(mut self, f: fn(&WalkItem) -> bool) -> Self {
        self.finish_when = f;
        self
    }
}
