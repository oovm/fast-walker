use crate::{WalkItem, WalkResultQueue, WalkTaskQueue};
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

mod config;

pub mod queue;

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
    pub reject_directory: fn(&Path, usize) -> bool,
}

pub struct WalkSearcher {
    result_queue: WalkResultQueue,
}

impl Iterator for WalkSearcher {
    type Item = WalkItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.result_queue.receive()
    }
}

#[test]
fn test() {
    let project = "C:\\P4Root\\project\\OtherPlanet";
    let plan = WalkPlan::new(project).with_depth_first().reject_if(|path, _| path.starts_with("."));
    for item in plan.into_iter().take(100) {
        println!("{:?}", item);
    }
}
