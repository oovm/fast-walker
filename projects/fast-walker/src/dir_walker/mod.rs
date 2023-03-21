use crate::WalkItem;
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

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
    pub ignore_when: fn(OsString) -> bool,
    /// Stop if a item matches the condition
    pub finish_when: fn(&WalkItem) -> bool,
}

impl Default for WalkPlan {
    fn default() -> Self {
        Self {
            check_list: vec![],
            follow_symlinks: false,
            depth_first: false,
            threads: 8,
            reject_when: |_, _| false,
            ignore_when: |_| false,
            finish_when: |_| false,
        }
    }
}

impl WalkPlan {
    /// Create a new plan with initial path
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self { check_list: vec![path.as_ref().to_path_buf()], ..Default::default() }
    }
    /// Create a new plan with initial paths
    pub fn roots<I>(roots: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<Path>,
    {
        Self { check_list: roots.into_iter().map(|p| p.as_ref().to_path_buf()).collect(), ..Default::default() }
    }
    /// Search all subdirectories with breadth first
    pub fn breadth_first_search(mut self) -> Self {
        self.depth_first = false;
        self
    }
    /// Search all subdirectories with depth first
    pub fn depth_first_search(mut self) -> Self {
        self.depth_first = true;
        self
    }
    /// Search with threads
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }
    /// Reject directories if it matches the condition
    ///
    /// # Examples
    ///
    /// - ignore hidden directories
    ///
    /// ```
    /// # use fast_walker::WalkPlan;
    /// let plan = WalkPlan::new(".").reject_if(|path, _| path.starts_with("."));
    /// ```
    pub fn reject_if(mut self, f: fn(&Path, usize) -> bool) -> Self {
        self.reject_when = f;
        self
    }
    /// Ignore files if it's name matches the condition
    ///
    /// Notice that it does not ignore directories whose name matches the condition
    ///
    /// # Examples
    ///
    /// - ignore non-ascii files
    ///
    /// ```
    /// # use fast_walker::WalkPlan;
    /// let plan = WalkPlan::new(".").ignore_if(|path| !path.is_ascii());
    /// ```
    pub fn ignore_if(mut self, f: fn(OsString) -> bool) -> Self {
        self.ignore_when = f;
        self
    }
    pub fn finish_if(mut self, f: fn(&WalkItem) -> bool) -> Self {
        self.finish_when = f;
        self
    }
}
